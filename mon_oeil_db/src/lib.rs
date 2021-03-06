use std::env;

use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod};
use futures::future;
use linked_hash_map::LinkedHashMap;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{error::SqlState, types::ToSql, Error, NoTls};
use uuid::Uuid;

mod models;

use models::raw::*;
pub use models::*;

pub fn connect_db() -> GestureClientPool {
    let (host, port, user, password, dbname) = (
        env::var("PG_HOST").unwrap(),
        env::var("PG_PORT").unwrap(),
        env::var("PG_USER").unwrap(),
        env::var("PG_PWD").unwrap(),
        env::var("PG_DB_NAME").unwrap(),
    );

    GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap()
}

#[derive(PartialEq, Eq, Debug)]
pub enum DbError {
    ForeignKeyViolation(String),
    NotFound,
    Other(String),
}

impl From<Error> for DbError {
    fn from(err: Error) -> DbError {
        match err.code().map(SqlState::code) {
            Some(x) if x == SqlState::FOREIGN_KEY_VIOLATION.code() => {
                DbError::ForeignKeyViolation(format!("{:?}", err))
            }
            _ => DbError::Other(format!("{:?}", err)),
        }
    }
}

#[derive(Clone)]
pub struct GestureClientPool(Pool);

impl GestureClientPool {
    /// Create db pool
    pub fn connect(
        host: &str,
        port: &str,
        user: &str,
        password: &str,
        dbname: &str,
    ) -> Result<Self, DbError> {
        let mut cfg = Config::new();
        cfg.host = Some(host.to_owned());
        cfg.port = Some(
            port.to_owned()
                .parse()
                .map_err(|e| DbError::Other(format!("Config port err : {:?}", e)))?,
        );
        cfg.user = Some(user.to_owned());
        cfg.password = Some(password.to_owned());
        cfg.dbname = Some(dbname.to_owned());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = cfg
            .create_pool(NoTls)
            .map_err(|e| DbError::Other(format!("Conf pool err : {:?}", e)))?;

        Ok(Self(pool))
    }

    /// Borow a client from the pool
    /// Future wait until one is available
    /// On drop return it to the pool
    pub async fn get(&self) -> Result<GestureClient, DbError> {
        let client: Client = self
            .0
            .get()
            .await
            .map_err(|e| DbError::Other(format!("Getting pool failed {:?} {}", e, e)))?;

        Ok(GestureClient { client })
    }
}

pub struct GestureClient {
    client: Client,
}

impl GestureClient {
    /// Retrieve all gestures from db
    pub async fn all_gestures(
        &self,
        pagination: PaginationRequest,
        search: Option<String>,
    ) -> Result<(Vec<Gesture>, u16), DbError> {
        let client = &self.client;

        let offset = (pagination.page - 1) * pagination.max;

        let search = search.map(|s| format!("{}:*", s));

        let (gestures, gestures_count_query) = match search.clone() {
            Some(search) => {
                let gestures_query = &format!(
                    "SELECT {g_table}.* FROM {s_table}
                    LEFT JOIN {g_table} ON {s_table}.{id_g} = {g_table}.{id_g}
                    WHERE {s_table}.{document} @@ to_tsquery($1)
                    ORDER BY ts_rank({s_table}.{document}, to_tsquery($1)) DESC
                    LIMIT {} OFFSET {}",
                    pagination.max,
                    offset,
                    id_g = ID_G_COL,
                    s_table = SEARCHABLE_VIEW,
                    g_table = G_TABLE,
                    document = DOCUMENT
                );

                let gestures = select::<RawGesture>(client, &gestures_query, &[&search]).await?;

                let gestures_count_query = format!(
                    "SELECT COUNT(*) FROM {}
                    WHERE {} @@ to_tsquery($1)",
                    SEARCHABLE_VIEW, DOCUMENT
                );
                (gestures, gestures_count_query)
            }
            _ => {
                let gestures_query = format!(
                    "SELECT * FROM gestures ORDER BY {} DESC LIMIT {} OFFSET {}",
                    CREATION_COL, pagination.max, offset
                );
                let gestures = select::<RawGesture>(client, &gestures_query, &[]).await?;

                let gestures_count_query = "SELECT COUNT(*) FROM gestures".to_owned();
                (gestures, gestures_count_query)
            }
        };

        let ids_gestures = gestures.iter().map(|g| g.id_gesture).collect::<Vec<Uuid>>();

        // Select evrything from db
        let gestures_count_query = async {
            match search {
                Some(search) => client
                    .query_one(gestures_count_query.as_str(), &[&search])
                    .await
                    .map_err(DbError::from),
                _ => client
                    .query_one(gestures_count_query.as_str(), &[])
                    .await
                    .map_err(DbError::from),
            }
        };

        let descriptions_query = format!(
            "SELECT * FROM {} WHERE {} = ANY($1) ORDER BY {}",
            D_TABLE, ID_G_COL, CREATION_COL
        );
        let meanings_query = format!(
            "SELECT * FROM {} WHERE {} = ANY($1) OR {} = ANY($1) ORDER BY {}",
            M_TABLE_WITH_G_ID, ID_G_COL, ID_DG_COL, CREATION_COL
        );
        let pictures_query = format!(
            "SELECT * FROM {} WHERE {} = ANY($1) ORDER BY {}",
            P_TABLE, ID_G_COL, CREATION_COL
        );

        let (descriptions, meanings, pictures, total) = future::try_join4(
            select::<RawDescription>(client, &descriptions_query, &[&ids_gestures]),
            select::<RawMeaning>(client, &meanings_query, &[&ids_gestures]),
            select::<RawPicture>(client, &pictures_query, &[&ids_gestures]),
            gestures_count_query,
        )
        .await?;

        // group every data by gesture id as plain datas
        let (descriptions, _) = group_by_id_gesture(descriptions);
        let (meanings_g, meanings_o) = group_by_id_gesture(meanings);
        let (pictures, _) = group_by_id_gesture(pictures);

        // group nested description meaning
        let (meanings_d, _) = group_by_id_description(meanings_o);

        let gestures = merge(gestures, descriptions, meanings_g, meanings_d, pictures);

        let total: i64 = total.get(0);
        // merge as nested datas our pre-grouped datas
        Ok((gestures, total as u16))
    }

    /// Add a gesture in db
    pub async fn add_gesture(&self, new_gesture: NewGesture) -> Result<String, DbError> {
        let new_id = Uuid::new_v4();
        insert(&self.client, RawGesture::from(new_gesture, new_id))
            .await
            .map(|_| new_id.to_hyphenated().to_string())
    }

    pub async fn update_gesture(
        &self,
        id: &str,
        updatable_gesture: NewGesture,
    ) -> Result<(), DbError> {
        let id = Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        update(&self.client, InnerGesture::from(updatable_gesture, id)).await
    }

    /// Add a description and nested data in db for a gesture
    pub async fn add_description(
        &self,
        new_description: NewDescription,
        id_gesture: &str,
    ) -> Result<String, DbError> {
        let id_gesture = Uuid::parse_str(id_gesture)
            .map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        let new_id = Uuid::new_v4();

        insert(
            &self.client,
            RawDescription::from(new_description, id_gesture, new_id),
        )
        .await
        .map(|_| new_id.to_hyphenated().to_string())
    }

    pub async fn update_description(
        &self,
        id: &str,
        new_description: NewDescription,
    ) -> Result<(), DbError> {
        let id = Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        update(&self.client, InnerDescription::from(new_description, id)).await
    }

    /// Add a meaning in db for a gesture or description
    pub async fn add_meaning(
        &self,
        meaning: NewMeaning,
        id_gesture: Option<&str>,
        id_description: Option<&str>,
    ) -> Result<String, DbError> {
        let id_gesture = match id_gesture.map(|id| {
            Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))
        }) {
            Some(Err(e)) => return Err(e),
            Some(Ok(i)) => Some(i),
            None => None,
        };
        let id_description = match id_description.map(|id| {
            Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))
        }) {
            Some(Err(e)) => return Err(e),
            Some(Ok(i)) => Some(i),
            None => None,
        };

        let new_id = Uuid::new_v4();

        insert(
            &self.client,
            RawMeaning::from(meaning, id_gesture, id_description, new_id),
        )
        .await
        .map(|_| new_id.to_hyphenated().to_string())
    }

    pub async fn update_meaning(&self, id: &str, new_meaning: NewMeaning) -> Result<(), DbError> {
        let id = Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        update(&self.client, InnerMeaning::from(new_meaning, id)).await
    }

    /// Add a picture and nested data in db for a gesture
    pub async fn add_picture(
        &self,
        picture: NewPicture,
        id_gesture: &str,
    ) -> Result<String, DbError> {
        let id_gesture = Uuid::parse_str(id_gesture)
            .map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        let new_id = Uuid::new_v4();

        insert(&self.client, RawPicture::from(picture, id_gesture, new_id))
            .await
            .map(|_| new_id.to_hyphenated().to_string())
    }

    pub async fn update_picture_meta(
        &self,
        id: &str,
        new_picture_meta: NewPictureMeta,
    ) -> Result<(), DbError> {
        let id = Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        update(&self.client, InnerPictureMeta::from(new_picture_meta, id)).await
    }

    pub async fn update_picture_format(
        &self,
        id: &str,
        new_picture_file_info: NewPictureFileInfo,
    ) -> Result<(), DbError> {
        let id = Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;
        update(
            &self.client,
            PictureFileInfo::from(new_picture_file_info, id),
        )
        .await
    }

    /// Delete gesture and nested object from db
    pub async fn delete_gesture_cascade(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            G_TABLE,
            ID_G_COL,
            &Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    pub async fn get_picture_format(&self, id: &str) -> Result<String, DbError> {
        let uuid =
            Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?;

        let query = format!(
            "SELECT {} FROM {} WHERE {}=$1",
            FORMAT_P_COL, P_TABLE, ID_P_COL
        );
        let row = self.client.query_opt(query.as_str(), &[&uuid]).await?;
        match row {
            Some(row) => Ok(row.get(FORMAT_P_COL)),
            _ => Err(DbError::NotFound),
        }
    }

    /// Delete description and nested data from db
    pub async fn delete_description_cascade(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            D_TABLE,
            ID_D_COL,
            &Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    /// Delete meaning from db
    pub async fn delete_meaning(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            M_TABLE,
            ID_M_COL,
            &Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    /// Delete picture from db
    pub async fn delete_picture(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            P_TABLE,
            ID_P_COL,
            &Uuid::parse_str(id).map_err(|e| DbError::Other(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>, DbError> {
        let mut user = select::<RawUser>(
            &self.client,
            &format!("SELECT * FROM {} WHERE {}=$1", U_TABLE, USERNAME_COL),
            &[&username],
        )
        .await?;

        Ok(user.pop().map(User::from_raw))
    }
}

/// Query the bdd
async fn select<T: FromTokioPostgresRow>(
    client: &Client,
    sql: &str,
    params: &[&(dyn ToSql + Sync)],
) -> Result<Vec<T>, DbError> {
    let rows = client.query(sql, params).await?;

    Ok(rows
        .into_iter()
        .map(T::from_row)
        .map(Result::unwrap)
        .collect())
}

/// Query the bdd
async fn insert<T: Insertable>(client: &Client, item: T) -> Result<(), DbError> {
    client
        .execute(item.insert_query().as_ref() as &str, &item.query_params())
        .await?;
    Ok(())
}

async fn update<T: Updatable>(client: &Client, item: T) -> Result<(), DbError> {
    let nb_modif = client
        .execute(item.update_query().as_ref() as &str, &item.query_params())
        .await?;

    if nb_modif > 0 {
        Ok(())
    } else {
        Err(DbError::NotFound)
    }
}

pub async fn delete(client: &Client, table: &str, id_col: &str, id: &Uuid) -> Result<(), DbError> {
    let sql = format!("DELETE FROM {} WHERE {} = $1", table, id_col);
    let sql: &str = sql.as_ref();

    let nb = client.execute(sql, &[&id]).await?;
    if nb < 1 {
        Err(DbError::NotFound)
    } else {
        Ok(())
    }
}

/// Group item in HashMap like: [(id_gesture, items)] and others (that are non linked to gesture) are partitioned next to it
fn group_by_id_gesture<T: GestureReliant>(items: Vec<T>) -> (LinkedHashMap<Uuid, Vec<T>>, Vec<T>) {
    let (some, none): (Vec<T>, Vec<T>) = items
        .into_iter()
        .partition(|item| item.id_gesture().is_some());

    let map = some
        .into_iter()
        // use LinkedHashMap to keep order
        .fold(LinkedHashMap::<Uuid, Vec<T>>::new(), |mut acc, item| {
            match acc.get_mut(item.id_gesture().unwrap()) {
                Some(v) => v.push(item),
                _ => {
                    acc.insert(*item.id_gesture().unwrap(), vec![item]);
                }
            };
            acc
        });

    (map, none)
}

/// Group meaning in HashMap like: [(id_description, items)] and others (that are non linked to description) are partitioned next to it
fn group_by_id_description(
    meanings: Vec<RawMeaning>,
) -> (LinkedHashMap<Uuid, Vec<RawMeaning>>, Vec<RawMeaning>) {
    let (some, none): (Vec<RawMeaning>, Vec<RawMeaning>) = meanings
        .into_iter()
        .partition(|meaning| meaning.id_description.is_some());

    let map = some
        .into_iter()
        // use LinkedHashMap to keep order
        .fold(
            LinkedHashMap::<Uuid, Vec<RawMeaning>>::new(),
            |mut acc, meaning| {
                match acc.get_mut(meaning.id_description.as_ref().unwrap()) {
                    Some(v) => v.push(meaning),
                    _ => {
                        acc.insert(*meaning.id_description.as_ref().unwrap(), vec![meaning]);
                    }
                };
                acc
            },
        );

    (map, none)
}

/// Make nested structures from flat id grouped structure
fn merge(
    gestures: Vec<RawGesture>,
    mut descriptions: LinkedHashMap<Uuid, Vec<RawDescription>>,
    mut meanings: LinkedHashMap<Uuid, Vec<RawMeaning>>,
    mut description_meanings: LinkedHashMap<Uuid, Vec<RawMeaning>>,
    mut pictures: LinkedHashMap<Uuid, Vec<RawPicture>>,
) -> Vec<Gesture> {
    gestures
        .into_iter()
        // for each gesture add nested data by id (from id grouped Map)
        .map(|gesture| {
            let descriptions = descriptions
                .remove(&gesture.id_gesture)
                .or_else(|| Some(vec![]))
                .unwrap()
                .into_iter()
                .map(|d| {
                    // for each meaning add nested data by id (from id grouped Map)
                    let description_meanings = description_meanings
                        .remove(&d.id_description)
                        .or_else(|| Some(vec![]))
                        .unwrap()
                        .into_iter()
                        .map(Meaning::from_raw)
                        .collect();

                    Description::from_raw(d, description_meanings)
                })
                .collect();

            let meanings = meanings
                .remove(&gesture.id_gesture)
                .or_else(|| Some(vec![]))
                .unwrap()
                .into_iter()
                .map(Meaning::from_raw)
                .collect();

            let pictures = pictures
                .remove(&gesture.id_gesture)
                .or_else(|| Some(vec![]))
                .unwrap()
                .into_iter()
                .map(Picture::from_raw)
                .collect();

            Gesture::from_raw(gesture, descriptions, meanings, pictures)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod group_by_id_gesture {
        use super::*;

        #[test]
        fn should_group_gesture_by_gesture_and_reject_none() {
            let mut md = LinkedHashMap::new();
            md.insert(id_g1(), vec![raw_g1()]);
            md.insert(id_g2(), vec![raw_g2()]);
            assert_eq!((md, vec![]), group_by_id_gesture(vec![raw_g1(), raw_g2()]));
        }

        #[test]
        fn should_group_description_by_gesture_and_reject_none() {
            let mut md = LinkedHashMap::new();
            md.insert(id_g1(), vec![raw_d1(), raw_d2()]);
            md.insert(id_g2(), vec![raw_d3()]);
            assert_eq!(
                (md, vec![]),
                group_by_id_gesture(vec![raw_d1(), raw_d2(), raw_d3()])
            );
        }

        #[test]
        fn should_group_meanings_by_gesture_and_reject_others() {
            let mut md = LinkedHashMap::new();
            md.insert(id_g1(), vec![raw_m1(), raw_m2()]);
            md.insert(id_g2(), vec![raw_m3()]);
            assert_eq!(
                (md, vec![raw_m4(), raw_m5(), raw_m6()]),
                group_by_id_gesture(vec![
                    raw_m1(),
                    raw_m2(),
                    raw_m3(),
                    raw_m4(),
                    raw_m5(),
                    raw_m6()
                ])
            );
        }

        #[test]
        fn should_group_picture_by_gesture_and_reject_none() {
            let mut md = LinkedHashMap::new();
            md.insert(id_g1(), vec![raw_p1(), raw_p2()]);
            md.insert(id_g2(), vec![raw_p3()]);
            assert_eq!(
                (md, vec![]),
                group_by_id_gesture(vec![raw_p1(), raw_p2(), raw_p3()])
            );
        }
    }

    #[cfg(test)]
    mod group_by_id_description {
        use super::*;

        #[test]
        fn should_group_description_and_reject_gesture_link() {
            let mut md = LinkedHashMap::new();
            md.insert(id_d1(), vec![raw_m4()]);
            md.insert(id_d3(), vec![raw_m5(), raw_m6()]);
            assert_eq!(
                (md, vec![raw_m1(), raw_m2(), raw_m3()]),
                group_by_id_description(vec![
                    raw_m1(),
                    raw_m2(),
                    raw_m3(),
                    raw_m4(),
                    raw_m5(),
                    raw_m6()
                ])
            );
        }
    }

    #[cfg(test)]
    mod merge {
        use super::*;

        #[test]
        fn a_lonely_gesture_should_only_map_it() {
            assert_eq!(
                vec![g1(vec![], vec![], vec![])],
                merge(
                    vec![raw_g1()],
                    LinkedHashMap::new(),
                    LinkedHashMap::new(),
                    LinkedHashMap::new(),
                    LinkedHashMap::new()
                )
            );
        }

        #[test]
        fn _2_gestures_should_map_them() {
            assert_eq!(
                vec![g1(vec![], vec![], vec![]), g2(vec![], vec![], vec![])],
                merge(
                    vec![raw_g1(), raw_g2()],
                    LinkedHashMap::new(),
                    LinkedHashMap::new(),
                    LinkedHashMap::new(),
                    LinkedHashMap::new()
                )
            );
        }

        #[test]
        fn gestures_with_direct_links_should_map_them() {
            let mut descriptions = LinkedHashMap::new();
            descriptions.insert(id_g1(), vec![raw_d1(), raw_d2()]);
            descriptions.insert(id_g2(), vec![raw_d3()]);

            let mut meanings = LinkedHashMap::new();
            meanings.insert(id_g1(), vec![raw_m1(), raw_m2()]);
            meanings.insert(id_g2(), vec![raw_m3()]);

            let mut pictures = LinkedHashMap::new();
            pictures.insert(id_g1(), vec![raw_p1(), raw_p2()]);
            pictures.insert(id_g2(), vec![raw_p3()]);

            assert_eq!(
                vec![
                    g1(
                        vec![d1(vec![]), d2(vec![])],
                        vec![m1(), m2()],
                        vec![p1(), p2()]
                    ),
                    g2(vec![d3(vec![])], vec![m3()], vec![p3()]),
                ],
                merge(
                    vec![raw_g1(), raw_g2()],
                    descriptions,
                    meanings,
                    LinkedHashMap::new(),
                    pictures
                )
            );
        }

        #[test]
        fn gestures_with_meanings_of_descriptions_should_group_them() {
            let mut descriptions = LinkedHashMap::new();
            descriptions.insert(id_g1(), vec![raw_d1(), raw_d2()]);
            descriptions.insert(id_g2(), vec![raw_d3()]);

            let mut meaning_d = LinkedHashMap::new();
            meaning_d.insert(id_d1(), vec![raw_m4()]);
            meaning_d.insert(id_d3(), vec![raw_m5()]);

            assert_eq!(
                vec![
                    g1(vec![d1(vec![m4()]), d2(vec![])], vec![], vec![]),
                    g2(vec![d3(vec![m5()])], vec![], vec![]),
                ],
                merge(
                    vec![raw_g1(), raw_g2()],
                    descriptions,
                    LinkedHashMap::new(),
                    meaning_d,
                    LinkedHashMap::new()
                )
            );
        }
    }
    // --------------------
    // making data for test
    // --------------------

    fn g1(
        descriptions: Vec<Description>,
        meanings: Vec<Meaning>,
        pictures: Vec<Picture>,
    ) -> Gesture {
        Gesture {
            id: id_g1().to_hyphenated().to_string(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions,
            meanings,
            pictures,
        }
    }

    fn g2(
        descriptions: Vec<Description>,
        meanings: Vec<Meaning>,
        pictures: Vec<Picture>,
    ) -> Gesture {
        Gesture {
            id: id_g2().to_hyphenated().to_string(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions,
            meanings,
            pictures,
        }
    }

    fn d1(meanings: Vec<Meaning>) -> Description {
        Description {
            id: id_d1().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valued1".to_owned(),
            meanings,
        }
    }

    fn d2(meanings: Vec<Meaning>) -> Description {
        Description {
            id: id_d2().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valued2".to_owned(),
            meanings,
        }
    }

    fn d3(meanings: Vec<Meaning>) -> Description {
        Description {
            id: id_d3().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valued3".to_owned(),
            meanings,
        }
    }

    fn m1() -> Meaning {
        Meaning {
            id: id_m1().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valuem1".to_owned(),
        }
    }

    fn m2() -> Meaning {
        Meaning {
            id: id_m2().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valuem2".to_owned(),
        }
    }

    fn m3() -> Meaning {
        Meaning {
            id: id_m3().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valuem3".to_owned(),
        }
    }

    fn m4() -> Meaning {
        Meaning {
            id: id_m4().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valuem4".to_owned(),
        }
    }

    fn m5() -> Meaning {
        Meaning {
            id: id_m5().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            value: "valuem5".to_owned(),
        }
    }

    fn p1() -> Picture {
        Picture {
            id: id_p1().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn p2() -> Picture {
        Picture {
            id: id_p2().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn p3() -> Picture {
        Picture {
            id: id_p3().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn raw_g1() -> RawGesture {
        RawGesture {
            id_gesture: id_g1(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
        }
    }

    fn raw_g2() -> RawGesture {
        RawGesture {
            id_gesture: id_g2(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
        }
    }

    fn raw_d1() -> RawDescription {
        RawDescription {
            id_description: id_d1(),
            id_gesture: id_g1(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valued1".to_owned(),
        }
    }

    fn raw_d2() -> RawDescription {
        RawDescription {
            id_description: id_d2(),
            id_gesture: id_g1(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valued2".to_owned(),
        }
    }

    fn raw_d3() -> RawDescription {
        RawDescription {
            id_description: id_d3(),
            id_gesture: id_g2(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valued3".to_owned(),
        }
    }

    fn raw_m1() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m1(),
            id_gesture: Some(id_g1()),
            id_description: None,
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem1".to_owned(),
        }
    }

    fn raw_m2() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m2(),
            id_gesture: Some(id_g1()),
            id_description: None,
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem2".to_owned(),
        }
    }

    fn raw_m3() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m3(),
            id_gesture: Some(id_g2()),
            id_description: None,
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem3".to_owned(),
        }
    }

    fn raw_m4() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m4(),
            id_gesture: None,
            id_description: Some(id_d1()),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem4".to_owned(),
        }
    }

    fn raw_m5() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m5(),
            id_gesture: None,
            id_description: Some(id_d3()),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem5".to_owned(),
        }
    }

    fn raw_m6() -> RawMeaning {
        RawMeaning {
            id_meaning: id_m6(),
            id_gesture: None,
            id_description: Some(id_d3()),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            val: "valuem6".to_owned(),
        }
    }

    fn raw_p1() -> RawPicture {
        RawPicture {
            id_picture: id_p1(),
            id_gesture: id_g1(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn raw_p2() -> RawPicture {
        RawPicture {
            id_picture: id_p2(),
            id_gesture: id_g1(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn raw_p3() -> RawPicture {
        RawPicture {
            id_picture: id_p3(),
            id_gesture: id_g2(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            format: "png".to_owned(),
        }
    }

    fn id_g1() -> Uuid {
        Uuid::parse_str("ec953c9e-a1c1-4095-a038-4ea0d2e8efd7").unwrap()
    }
    fn id_g2() -> Uuid {
        Uuid::parse_str("d1d0706c-8b0e-4213-8839-ce0580a3974f").unwrap()
    }
    fn id_d1() -> Uuid {
        Uuid::parse_str("07330b55-7842-429c-b79d-aeb557cb625b").unwrap()
    }
    fn id_d2() -> Uuid {
        Uuid::parse_str("7ae20051-7661-4830-ab8d-f1709421a379").unwrap()
    }
    fn id_d3() -> Uuid {
        Uuid::parse_str("ae572770-ce65-40de-a1f2-fa1f7a6aa15f").unwrap()
    }
    fn id_m1() -> Uuid {
        Uuid::parse_str("8d9f51cc-8b10-48c2-ae9b-5c94cb66afc4").unwrap()
    }
    fn id_m2() -> Uuid {
        Uuid::parse_str("c8c63fe8-c72b-4404-81e7-cecd81f44758").unwrap()
    }
    fn id_m3() -> Uuid {
        Uuid::parse_str("c3f8b842-64e2-4441-840e-374c211e93e2").unwrap()
    }
    fn id_m4() -> Uuid {
        Uuid::parse_str("8e193178-3109-4509-bfeb-086eb1e05662").unwrap()
    }
    fn id_m5() -> Uuid {
        Uuid::parse_str("fa5a68e6-30a9-404f-8453-59fd784dcc50").unwrap()
    }
    fn id_m6() -> Uuid {
        Uuid::parse_str("fa23a3cf-8e82-4f0f-994e-6a7cf23671c6").unwrap()
    }
    fn id_p1() -> Uuid {
        Uuid::parse_str("dae4ee92-853b-4757-9b58-2e908971d8b9").unwrap()
    }
    fn id_p2() -> Uuid {
        Uuid::parse_str("74a63d67-fcb6-4def-b98a-6adc4c8fce5a").unwrap()
    }
    fn id_p3() -> Uuid {
        Uuid::parse_str("ce27c124-e47b-490f-b8fe-3f37d5dbbef6").unwrap()
    }
}

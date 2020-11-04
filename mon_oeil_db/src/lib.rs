use std::env;

use deadpool_postgres::{Client, Config, ManagerConfig, Pool, RecyclingMethod};
use futures::future;
use linked_hash_map::LinkedHashMap;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::{types::ToSql, Error, NoTls};
use uuid::Uuid;

mod models;

use models::raw::*;
pub use models::*;

pub fn connect_db() -> GestureClientPool {
    let (host, port, user, password, dbname) = (
        env::var("PG_HOST").unwrap(),
        env::var("PG_PORT").unwrap(),
        env::var("PG_DB_NAME").unwrap(),
        env::var("PG_USER").unwrap(),
        env::var("PG_PWD").unwrap(),
    );

    GestureClientPool::connect(&host, &port, &user, &password, &dbname).unwrap()
}

#[derive(PartialEq, Eq, Debug)]
pub struct DbError(String);

impl From<Error> for DbError {
    fn from(err: Error) -> DbError {
        DbError(format!("{:?}", err))
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
                .map_err(|e| DbError(format!("Config port err : {:?}", e)))?,
        );
        cfg.user = Some(user.to_owned());
        cfg.password = Some(password.to_owned());
        cfg.dbname = Some(dbname.to_owned());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = cfg
            .create_pool(NoTls)
            .map_err(|e| DbError(format!("Conf pool err : {:?}", e)))?;

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
            .map_err(|e| DbError(format!("Getting pool failed {:?}", e)))?;

        Ok(GestureClient { client })
    }
}

pub struct GestureClient {
    client: Client,
}

impl GestureClient {
    /// Retrieve all gestures from db
    pub async fn gestures(&self) -> Result<Vec<Gesture>, DbError> {
        let client = &self.client;

        // Select evrything from db
        let (gestures, descriptions, meanings, pictures) = future::try_join4(
            select::<RawGesture>(client, "SELECT * FROM gestures", &[]),
            select::<RawDescription>(client, "SELECT * FROM descriptions", &[]),
            select::<RawMeaning>(client, "SELECT * FROM meanings", &[]),
            select::<RawPicture>(client, "SELECT * FROM pictures", &[]),
        )
        .await?;

        // group every data by gesture id as plain datas
        let (descriptions, _) = group_by_id_gesture(descriptions);
        let (meanings_g, meanings_o) = group_by_id_gesture(meanings);
        let (pictures, _) = group_by_id_gesture(pictures);

        // group nested description meaning
        let (meanings_d, _) = group_by_id_description(meanings_o);

        // merge as nested datas our pre-grouped datas
        Ok(merge(
            gestures,
            descriptions,
            meanings_g,
            meanings_d,
            pictures,
        ))
    }

    /// Add a gesture and nested data in db
    pub async fn add_gesture(&self, gesture: NewGesture) -> Result<(), DbError> {
        // flat nested structure as Insertable linked by ids
        let (gesture, descriptions, meanings, pictures) = debunk_new_gesture(gesture);

        // future based inserts of each data
        let g_req = insert(&self.client, gesture);
        let d_reqs =
            future::try_join_all(descriptions.into_iter().map(|d| insert(&self.client, d)));
        let m_reqs = future::try_join_all(meanings.into_iter().map(|d| insert(&self.client, d)));
        let p_reqs = future::try_join_all(pictures.into_iter().map(|d| insert(&self.client, d)));

        // execute requests
        let res = future::try_join4(g_req, d_reqs, m_reqs, p_reqs).await;

        res.map(|_| ())
    }

    /// Add a description and nested data in db for a gesture
    pub async fn add_description(
        &self,
        description: NewDescription,
        id_gesture: &str,
    ) -> Result<(), DbError> {
        let id_gesture =
            Uuid::parse_str(id_gesture).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?;
        // flat nested structure as Insertable linked by ids
        let (description, meanings) = debunk_new_description(description, id_gesture);

        // future based inserts of each data
        let d_req = insert(&self.client, description);
        let m_reqs = future::try_join_all(meanings.into_iter().map(|d| insert(&self.client, d)));

        // execute requests
        let res = future::try_join(d_req, m_reqs).await;

        res.map(|_| ())
    }

    /// Add a meaning in db for a gesture or description
    pub async fn add_meaning(
        &self,
        meaning: NewMeaning,
        id_gesture: Option<&str>,
        id_description: Option<&str>,
    ) -> Result<(), DbError> {
        let id_gesture = match id_gesture
            .map(|id| Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e))))
        {
            Some(Err(e)) => return Err(e),
            Some(Ok(i)) => Some(i),
            None => None,
        };
        let id_description = match id_description
            .map(|id| Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e))))
        {
            Some(Err(e)) => return Err(e),
            Some(Ok(i)) => Some(i),
            None => None,
        };

        let id_meaning = Uuid::new_v4();

        insert(
            &self.client,
            RawMeaning::from(meaning, id_gesture, id_description, id_meaning),
        )
        .await
        .map(|_| ())
    }

    /// Add a picture and nested data in db for a gesture
    pub async fn add_picture(&self, picture: NewPicture, id_gesture: &str) -> Result<(), DbError> {
        let id_gesture =
            Uuid::parse_str(id_gesture).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?;
        let id_picture = Uuid::new_v4();

        insert(
            &self.client,
            RawPicture::from(picture, id_gesture, id_picture),
        )
        .await
        .map(|_| ())
    }

    /// Delete gesture and nested object from db
    pub async fn delete_gesture_cascade(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            G_TABLE,
            ID_G_COL,
            &Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    /// Delete description and nested data from db
    pub async fn delete_description_cascade(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            D_TABLE,
            ID_D_COL,
            &Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    /// Delete meaning from db
    pub async fn delete_meaning(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            M_TABLE,
            ID_M_COL,
            &Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?,
        )
        .await
    }

    /// Delete picture from db
    pub async fn delete_picture(&self, id: &str) -> Result<(), DbError> {
        delete(
            &self.client,
            P_TABLE,
            ID_P_COL,
            &Uuid::parse_str(id).map_err(|e| DbError(format!("Wrong uuid {:?}", e)))?,
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
    let rows = client.query(sql, params).await.map_err(DbError::from)?;

    Ok(rows
        .into_iter()
        .map(T::from_row)
        .map(Result::unwrap)
        .collect())
}

/// Flatten gesture as Insertable
fn debunk_new_gesture(
    mut gesture: NewGesture,
) -> (
    RawGesture,
    Vec<RawDescription>,
    Vec<RawMeaning>,
    Vec<RawPicture>,
) {
    let id_gesture = Uuid::new_v4();

    // Flatten description as Insertable
    let (descriptons, mut meanings) = gesture
        .descriptions
        .drain(..)
        .map(|d| debunk_new_description(d, id_gesture))
        .collect::<Vec<(RawDescription, Vec<RawMeaning>)>>()
        .into_iter()
        .fold(
            (vec![], vec![]),
            |(mut descriptions, mut meanings), (d, ms)| {
                descriptions.push(d);
                meanings.extend(ms);
                (descriptions, meanings)
            },
        );

    // meanings as Insertable
    let gesture_s_meaning = gesture
        .meanings
        .drain(..)
        .map(|m| {
            let id_meaning = Uuid::new_v4();
            RawMeaning::from(m, Some(id_gesture), None, id_meaning)
        })
        .collect::<Vec<RawMeaning>>();

    // merge description's and gesture's meanings
    meanings.extend(gesture_s_meaning);

    // pictures as Insertable
    let pictures = gesture
        .pictures
        .drain(..)
        .map(|p| {
            let id_picture = Uuid::new_v4();
            RawPicture::from(p, id_gesture, id_picture)
        })
        .collect::<Vec<RawPicture>>();

    // gesture as Insertable
    let gesture = RawGesture::from(gesture, id_gesture);

    (gesture, descriptons, meanings, pictures)
}

fn debunk_new_description(
    mut description: NewDescription,
    id_gesture: Uuid,
) -> (RawDescription, Vec<RawMeaning>) {
    let id_description = Uuid::new_v4();

    // meanings as Insertable
    let meanings = description
        .meanings
        .drain(..)
        .map(|m| {
            let id_meaning = Uuid::new_v4();
            RawMeaning::from(m, None, Some(id_description), id_meaning)
        })
        .collect();

    // description as Insertable
    let description = RawDescription::from(description, id_gesture, id_description);

    (description, meanings)
}

/// Query the bdd
async fn insert<T: Insertable>(client: &Client, item: T) -> Result<(), DbError> {
    client
        .execute(item.insert_query().as_ref() as &str, &item.query_params())
        .await
        .map_err(DbError::from)
        .map(|_| ())
}

pub async fn delete(client: &Client, table: &str, id_col: &str, id: &Uuid) -> Result<(), DbError> {
    let sql = format!("DELETE FROM {} WHERE {} = $1", table, id_col);
    let sql: &str = sql.as_ref();

    match client.execute(sql, &[&id]).await.map_err(DbError::from) {
        Ok(nb) => {
            // check if there is at least one row deleted
            if nb < 1 {
                Err(DbError("No items were deleted".to_string()))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
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
    #[cfg(test)]
    mod debunk_new_gesture {
        use super::*;

        #[test]
        fn gesture_with_some_on_each_links() {
            let (gesture, descriptions, meanings, pictures) = debunk_new_gesture(NewGesture {
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![
                    NewDescription {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valued1".to_owned(),
                        meanings: vec![
                            NewMeaning {
                                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                                value: "valuem1".to_owned(),
                            },
                            NewMeaning {
                                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                                value: "valuem2".to_owned(),
                            },
                        ],
                    },
                    NewDescription {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valued2".to_owned(),
                        meanings: vec![],
                    },
                ],
                meanings: vec![
                    NewMeaning {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valuem3".to_owned(),
                    },
                    NewMeaning {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valuem4".to_owned(),
                    },
                ],
                pictures: vec![
                    NewPicture {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                    },
                    NewPicture {
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                    },
                ],
            });

            let mut g1 = raw_g1();
            g1.id_gesture = gesture.id_gesture;
            assert_eq!(g1, gesture);

            let mut d1 = raw_d1();
            d1.id_gesture = gesture.id_gesture;
            d1.id_description = descriptions[0].id_description;
            assert_eq!(d1, descriptions[0]);

            let mut d2 = raw_d2();
            d2.id_gesture = gesture.id_gesture;
            d2.id_description = descriptions[1].id_description;
            assert_eq!(d2, descriptions[1]);

            let mut m1 = raw_m1();
            m1.id_gesture = None;
            m1.id_description = meanings[0].id_description;
            m1.id_meaning = meanings[0].id_meaning;
            assert_eq!(m1, meanings[0]);

            let mut m2 = raw_m2();
            m2.id_gesture = None;
            m2.id_description = meanings[1].id_description;
            m2.id_meaning = meanings[1].id_meaning;
            assert_eq!(m2, meanings[1]);

            let mut m3 = raw_m3();
            m3.id_gesture = meanings[2].id_gesture;
            m3.id_description = None;
            m3.id_meaning = meanings[2].id_meaning;
            assert_eq!(m3, meanings[2]);

            let mut m4 = raw_m4();
            m4.id_gesture = meanings[3].id_gesture;
            m4.id_description = None;
            m4.id_meaning = meanings[3].id_meaning;
            assert_eq!(m4, meanings[3]);

            let mut p1 = raw_p1();
            p1.id_gesture = gesture.id_gesture;
            p1.id_picture = pictures[0].id_picture;
            assert_eq!(p1, pictures[0]);

            let mut p2 = raw_p2();
            p2.id_gesture = gesture.id_gesture;
            p2.id_picture = pictures[1].id_picture;
            assert_eq!(p2, pictures[1]);
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
        }
    }

    fn p2() -> Picture {
        Picture {
            id: id_p2().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
        }
    }

    fn p3() -> Picture {
        Picture {
            id: id_p3().to_hyphenated().to_string(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
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
        }
    }

    fn raw_p2() -> RawPicture {
        RawPicture {
            id_picture: id_p2(),
            id_gesture: id_g1(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
        }
    }

    fn raw_p3() -> RawPicture {
        RawPicture {
            id_picture: id_p3(),
            id_gesture: id_g2(),
            langs: vec!["lang1".to_owned(), "lang2".to_owned()],
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

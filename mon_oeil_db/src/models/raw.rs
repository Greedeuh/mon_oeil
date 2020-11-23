use crate::*;
use std::cmp::{Eq, PartialEq};
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

/// Has a direct link to a gesture (id_gesture should be retrievable)
pub trait GestureReliant {
    fn id_gesture(&self) -> Option<&Uuid>;
}

pub const G_TABLE: &str = "gestures";
pub const D_TABLE: &str = "descriptions";
pub const M_TABLE: &str = "meanings";
pub const M_TABLE_WITH_G_ID: &str = "meanings_with_gesture_id";
pub const SEARCHABLE_VIEW: &str = "searchable";
pub const P_TABLE: &str = "pictures";
pub const U_TABLE: &str = "users";

pub const ID_G_COL: &str = "id_gesture";
pub const ID_DG_COL: &str = "id_description_gesture";
pub const TAGS_COL: &str = "tags";
pub const CREATION_COL: &str = "creation_date";
pub const ID_D_COL: &str = "id_description";
pub const VALUE_D_COL: &str = "val";
pub const LANG_D_COL: &str = "langs";
pub const ID_M_COL: &str = "id_meaning";
pub const VALUE_M_COL: &str = "val";
pub const LANG_M_COL: &str = "langs";
pub const ID_P_COL: &str = "id_picture";
pub const LANG_P_COL: &str = "langs";
pub const FORMAT_P_COL: &str = "format";
pub const USERNAME_COL: &str = "username";
pub const _PASSWORD_COL: &str = "password";
pub const DOCUMENT: &str = "document";

pub trait Insertable {
    /// Parametrized insert query
    fn insert_query(&self) -> String;
    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

pub trait Updatable {
    /// Parametrized insert query
    fn update_query(&self) -> String;
    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "gestures")]
pub struct RawGesture {
    pub id_gesture: Uuid,
    pub tags: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct InnerGesture {
    pub id_gesture: Uuid,
    pub tags: Vec<String>,
}

impl Insertable for RawGesture {
    fn insert_query(&self) -> String {
        format!(
            "INSERT INTO {} ({}, {}) VALUES ($1, $2)",
            G_TABLE, ID_G_COL, TAGS_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.id_gesture, &self.tags]
    }
}

impl Updatable for InnerGesture {
    fn update_query(&self) -> String {
        format!(
            "UPDATE {} SET {}=$1 WHERE {}=$2",
            G_TABLE, TAGS_COL, ID_G_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.tags, &self.id_gesture]
    }
}

impl GestureReliant for RawGesture {
    fn id_gesture(&self) -> Option<&Uuid> {
        Some(&self.id_gesture)
    }
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "descriptions")]
pub struct RawDescription {
    pub id_description: Uuid,
    pub id_gesture: Uuid,
    pub val: String,
    pub langs: Vec<String>,
}

impl Insertable for RawDescription {
    fn insert_query(&self) -> String {
        format!(
            "INSERT INTO {} ({}, {}, {}, {}) VALUES ($1, $2, $3, $4)",
            D_TABLE, ID_D_COL, ID_G_COL, VALUE_D_COL, LANG_D_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id_description,
            &self.id_gesture,
            &self.val,
            &self.langs,
        ]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InnerDescription {
    pub id_description: Uuid,
    pub val: String,
    pub langs: Vec<String>,
}

impl Updatable for InnerDescription {
    fn update_query(&self) -> String {
        format!(
            "UPDATE {} SET {}=$1, {}=$2 WHERE {}=$3",
            D_TABLE, VALUE_D_COL, LANG_D_COL, ID_D_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.val, &self.langs, &self.id_description]
    }
}

impl GestureReliant for RawDescription {
    fn id_gesture(&self) -> Option<&Uuid> {
        Some(&self.id_gesture)
    }
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "meanings")]
pub struct RawMeaning {
    pub id_meaning: Uuid,
    pub id_gesture: Option<Uuid>,
    pub id_description: Option<Uuid>,
    pub val: String,
    pub langs: Vec<String>,
}

impl Insertable for RawMeaning {
    fn insert_query(&self) -> String {
        format!(
            "INSERT INTO {} ({}, {}, {}, {}) VALUES ($1, $2, $3, $4)",
            M_TABLE,
            ID_M_COL,
            match self.id_gesture {
                Some(_) => ID_G_COL,
                _ => ID_D_COL,
            },
            VALUE_M_COL,
            LANG_M_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id_meaning,
            match &self.id_gesture {
                Some(_) => &self.id_gesture,
                _ => &self.id_description,
            },
            &self.val,
            &self.langs,
        ]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InnerMeaning {
    pub id_meaning: Uuid,
    pub val: String,
    pub langs: Vec<String>,
}

impl Updatable for InnerMeaning {
    fn update_query(&self) -> String {
        format!(
            "UPDATE {} SET {}=$1, {}=$2 WHERE {}=$3",
            M_TABLE, VALUE_M_COL, LANG_M_COL, ID_M_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.val, &self.langs, &self.id_meaning]
    }
}

impl GestureReliant for RawMeaning {
    fn id_gesture(&self) -> Option<&Uuid> {
        match &self.id_gesture {
            Some(x) => Some(&x),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "pictures")]
pub struct RawPicture {
    pub id_picture: Uuid,
    pub id_gesture: Uuid,
    pub langs: Vec<String>,
    pub format: String,
}

impl Insertable for RawPicture {
    fn insert_query(&self) -> String {
        format!(
            "INSERT INTO {} ({}, {}, {}, {}) VALUES ($1, $2, $3, $4)",
            P_TABLE, ID_P_COL, ID_G_COL, LANG_P_COL, FORMAT_P_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![
            &self.id_picture,
            &self.id_gesture,
            &self.langs,
            &self.format,
        ]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct InnerPictureMeta {
    pub id_picture: Uuid,
    pub langs: Vec<String>,
}

impl Updatable for InnerPictureMeta {
    fn update_query(&self) -> String {
        format!(
            "UPDATE {} SET {}=$1 WHERE {}=$2",
            P_TABLE, LANG_P_COL, ID_P_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.langs, &self.id_picture]
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct PictureFileInfo {
    pub id_picture: Uuid,
    pub format: String,
}

impl Updatable for PictureFileInfo {
    fn update_query(&self) -> String {
        format!(
            "UPDATE {} SET {}=$1 WHERE {}=$2",
            P_TABLE, FORMAT_P_COL, ID_P_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.format, &self.id_picture]
    }
}

impl GestureReliant for RawPicture {
    fn id_gesture(&self) -> Option<&Uuid> {
        Some(&self.id_gesture)
    }
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct RawUser {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod from_new {
        use super::*;

        #[test]
        fn gesture_should_map_raw() {
            let id_gesture = Uuid::new_v4();
            assert_eq!(
                raw_g1(id_gesture),
                RawGesture::from(
                    NewGesture {
                        tags: vec!["ah".to_owned(), "ha".to_owned()],
                    },
                    id_gesture,
                ),
            )
        }

        #[test]
        fn description_should_map_raw() {
            let id_gesture = Uuid::new_v4();
            let id_description = Uuid::new_v4();
            assert_eq!(
                raw_d1(id_description, id_gesture),
                RawDescription::from(
                    NewDescription {
                        value: "value".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    id_gesture,
                    id_description
                ),
            )
        }

        #[test]
        fn description_s_meaning_should_map_raw() {
            let id_description = Some(Uuid::new_v4());
            let id_meaning = Uuid::new_v4();
            assert_eq!(
                raw_m1(id_description, None, id_meaning),
                RawMeaning::from(
                    NewMeaning {
                        value: "value".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    None,
                    id_description,
                    id_meaning
                ),
            )
        }

        #[test]
        fn gesture_s_meaning_should_map_raw() {
            let id_gesture = Some(Uuid::new_v4());
            let id_meaning = Uuid::new_v4();
            assert_eq!(
                raw_m1(None, id_gesture, id_meaning),
                RawMeaning::from(
                    NewMeaning {
                        value: "value".to_owned(),
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                    },
                    id_gesture,
                    None,
                    id_meaning
                ),
            )
        }

        #[test]
        fn picture_should_map_raw() {
            let id_gesture = Uuid::new_v4();
            let id_picture = Uuid::new_v4();
            assert_eq!(
                raw_p1(id_gesture, id_picture),
                RawPicture::from(
                    NewPicture {
                        langs: vec!["fr".to_owned(), "us".to_owned()],
                        format: "png".to_owned()
                    },
                    id_gesture,
                    id_picture
                ),
            )
        }
    }

    pub fn raw_g1(id_gesture: Uuid) -> RawGesture {
        RawGesture {
            id_gesture,
            tags: vec!["ah".to_owned(), "ha".to_owned()],
        }
    }

    pub fn raw_d1(id_description: Uuid, id_gesture: Uuid) -> RawDescription {
        RawDescription {
            id_description,
            id_gesture,
            val: "value".to_owned(),
            langs: vec!["fr".to_owned(), "us".to_owned()],
        }
    }

    pub fn raw_m1(
        id_description: Option<Uuid>,
        id_gesture: Option<Uuid>,
        id_meaning: Uuid,
    ) -> RawMeaning {
        RawMeaning {
            id_description,
            id_gesture,
            id_meaning,
            val: "value".to_owned(),
            langs: vec!["fr".to_owned(), "us".to_owned()],
        }
    }

    pub fn raw_p1(id_gesture: Uuid, id_picture: Uuid) -> RawPicture {
        RawPicture {
            id_gesture,
            id_picture,
            langs: vec!["fr".to_owned(), "us".to_owned()],
            format: "png".to_owned(),
        }
    }
}

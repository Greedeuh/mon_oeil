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
pub const P_TABLE: &str = "pictures";
pub const U_TABLE: &str = "users";

pub const ID_G_COL: &str = "id_gesture";
pub const TAGS_COL: &str = "tags";
pub const ID_D_COL: &str = "id_description";
pub const VALUE_D_COL: &str = "val";
pub const LANG_D_COL: &str = "langs";
pub const ID_M_COL: &str = "id_meaning";
pub const VALUE_M_COL: &str = "val";
pub const LANG_M_COL: &str = "langs";
pub const ID_P_COL: &str = "id_picture";
pub const LANG_P_COL: &str = "langs";
pub const USERNAME_COL: &str = "username";
pub const _PASSWORD_COL: &str = "password";

pub trait Insertable {
    /// Parametrized insert query
    fn insert_query(&self) -> String;
    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

#[derive(PartialEq, Eq, Debug, PostgresMapper)]
#[pg_mapper(table = "gestures")]
pub struct RawGesture {
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
}

impl Insertable for RawPicture {
    fn insert_query(&self) -> String {
        format!(
            "INSERT INTO {} ({}, {}, {}) VALUES ($1, $2, $3)",
            P_TABLE, ID_P_COL, ID_G_COL, LANG_P_COL
        )
    }

    fn query_params(&self) -> Vec<&(dyn ToSql + Sync)> {
        vec![&self.id_picture, &self.id_gesture, &self.langs]
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
                        descriptions: vec![],
                        meanings: vec![],
                        pictures: vec![],
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
                        meanings: vec![],
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
                    },
                    id_gesture,
                    id_picture
                ),
            )
        }
    }

    #[cfg(test)]
    mod insert_query {
        use super::*;

        #[test]
        fn gesture() {
            assert_eq!(
                "INSERT INTO gestures (id_gesture, tags) VALUES ($1, $2)".to_owned(),
                raw_g1(Uuid::new_v4()).insert_query()
            );
        }

        #[test]
        fn decription() {
            assert_eq!(
                "INSERT INTO descriptions (id_description, id_gesture, val, langs) VALUES ($1, $2, $3, $4)"
                    .to_owned(),
                raw_d1(Uuid::new_v4(),Uuid::new_v4()).insert_query()
            );
        }

        #[test]
        fn description_s_meaning() {
            assert_eq!(
                "INSERT INTO meanings (id_meaning, id_description, val, langs) VALUES ($1, $2, $3, $4)"
                    .to_owned(),
                raw_m1(Some(Uuid::new_v4()), None, Uuid::new_v4()).insert_query()
            );
        }

        #[test]
        fn gesture_s_meaning() {
            assert_eq!(
                "INSERT INTO meanings (id_meaning, id_gesture, val, langs) VALUES ($1, $2, $3, $4)"
                    .to_owned(),
                raw_m1(None, Some(Uuid::new_v4()), Uuid::new_v4()).insert_query()
            );
        }

        #[test]
        fn picture() {
            assert_eq!(
                "INSERT INTO pictures (id_picture, id_gesture, langs) VALUES ($1, $2, $3)"
                    .to_owned(),
                raw_p1(Uuid::new_v4(), Uuid::new_v4()).insert_query()
            );
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
        }
    }
}

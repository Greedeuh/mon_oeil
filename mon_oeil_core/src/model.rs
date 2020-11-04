use mon_oeil_db as db;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Gesture {
    pub id: String,
    pub tags: Vec<String>,
    pub descriptions: Vec<Description>,
    pub meanings: Vec<Meaning>,
    pub pictures: Vec<Picture>,
}

impl From<db::Gesture> for Gesture {
    fn from(item: db::Gesture) -> Self {
        let db::Gesture {
            id,
            tags,
            descriptions,
            meanings,
            pictures,
        } = item;
        Self {
            id,
            tags,
            descriptions: descriptions.into_iter().map(From::from).collect(),
            meanings: meanings.into_iter().map(From::from).collect(),
            pictures: pictures.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Description {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<Meaning>,
}

impl From<db::Description> for Description {
    fn from(item: db::Description) -> Self {
        let db::Description {
            id,
            value,
            langs,
            meanings,
        } = item;
        Self {
            id,
            value,
            langs,
            meanings: meanings.into_iter().map(From::from).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Meaning {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
}

impl From<db::Meaning> for Meaning {
    fn from(item: db::Meaning) -> Self {
        let db::Meaning { id, value, langs } = item;
        Self { id, value, langs }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Picture {
    pub id: String,
    pub langs: Vec<String>,
}

impl From<db::Picture> for Picture {
    fn from(item: db::Picture) -> Self {
        let db::Picture { id, langs } = item;
        Self { id, langs }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NewGesture {
    pub tags: Vec<String>,
    pub descriptions: Vec<NewDescription>,
    pub meanings: Vec<NewMeaning>,
    pub pictures: Vec<NewPicture>,
}

impl Into<db::NewGesture> for NewGesture {
    fn into(self) -> db::NewGesture {
        let Self {
            tags,
            descriptions,
            meanings,
            pictures,
        } = self;
        db::NewGesture {
            tags,
            descriptions: descriptions.into_iter().map(|i| i.into()).collect(),
            meanings: meanings.into_iter().map(|i| i.into()).collect(),
            pictures: pictures.into_iter().map(|i| i.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NewDescription {
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<NewMeaning>,
}

impl Into<db::NewDescription> for NewDescription {
    fn into(self) -> db::NewDescription {
        let Self {
            value,
            langs,
            meanings,
        } = self;
        db::NewDescription {
            value,
            langs,
            meanings: meanings.into_iter().map(|i| i.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NewMeaning {
    pub value: String,
    pub langs: Vec<String>,
}

impl Into<db::NewMeaning> for NewMeaning {
    fn into(self) -> db::NewMeaning {
        let Self { value, langs } = self;
        db::NewMeaning { value, langs }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NewPicture {
    pub langs: Vec<String>,
}

impl Into<db::NewPicture> for NewPicture {
    fn into(self) -> db::NewPicture {
        let Self { langs } = self;
        db::NewPicture { langs }
    }
}

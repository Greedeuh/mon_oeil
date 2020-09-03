pub(crate) mod raw;

use raw::*;
use std::cmp::{Eq, PartialEq};

#[derive(PartialEq, Eq, Debug)]
pub struct Gesture {
    pub id: String,
    pub tags: Vec<String>,
    pub descriptions: Vec<Description>,
    pub meanings: Vec<Meaning>,
    pub pictures: Vec<Picture>,
}

impl Gesture {
    pub fn from_raw(
        raw: RawGesture,
        descriptions: Vec<Description>,
        meanings: Vec<Meaning>,
        pictures: Vec<Picture>,
    ) -> Self {
        let RawGesture { id_gesture, tags } = raw;
        Self {
            id: format!("{}", id_gesture),
            tags,
            descriptions,
            meanings,
            pictures,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Description {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<Meaning>,
}

impl Description {
    pub fn from_raw(raw: RawDescription, meanings: Vec<Meaning>) -> Self {
        let RawDescription {
            id_description,
            val,
            langs,
            ..
        } = raw;
        Self {
            id: format!("{}", id_description),
            value: val,
            langs,
            meanings,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Meaning {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
}

impl Meaning {
    pub fn from_raw(raw: RawMeaning) -> Self {
        let RawMeaning {
            id_meaning,
            val,
            langs,
            ..
        } = raw;
        Self {
            id: format!("{}", id_meaning),
            value: val,
            langs,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Picture {
    pub id: String,
    pub langs: Vec<String>,
}

impl Picture {
    pub fn from_raw(raw: RawPicture) -> Self {
        let RawPicture {
            id_picture, langs, ..
        } = raw;
        Self {
            id: format!("{}", id_picture),
            langs,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn from_raw(raw: RawUser) -> Self {
        let RawUser {
            username, password, ..
        } = raw;
        Self { username, password }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewGesture {
    pub tags: Vec<String>,
    pub descriptions: Vec<NewDescription>,
    pub meanings: Vec<NewMeaning>,
    pub pictures: Vec<NewPicture>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewDescription {
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<NewMeaning>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewMeaning {
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewPicture {
    pub langs: Vec<String>,
}

use mon_oeil_db as db;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};

pub mod mappers;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Gesture {
    pub id: String,
    pub tags: Vec<String>,
    pub descriptions: Vec<Description>,
    pub meanings: Vec<Meaning>,
    pub pictures: Vec<Picture>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Description {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<Meaning>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Meaning {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Picture {
    pub id: String,
    pub langs: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct NewGesture {
    pub tags: Vec<String>,
    pub descriptions: Vec<NewDescription>,
    pub meanings: Vec<NewMeaning>,
    pub pictures: Vec<NewPicture>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct NewDescription {
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<NewMeaning>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct NewMeaning {
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct NewPicture {
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct DbError(String);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub enum Error {
    Bug(String),
    Auth,
}

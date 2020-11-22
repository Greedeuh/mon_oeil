pub(crate) mod raw;

mod mappers;

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

#[derive(PartialEq, Eq, Debug)]
pub struct Description {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
    pub meanings: Vec<Meaning>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Meaning {
    pub id: String,
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Picture {
    pub id: String,
    pub langs: Vec<String>,
    pub format: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewGesture {
    pub tags: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewDescription {
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewMeaning {
    pub value: String,
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewPicture {
    pub langs: Vec<String>,
    pub format: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewPictureMeta {
    pub langs: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct NewPictureFileInfo {
    pub format: String,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PaginationRequest {
    pub max: u16,
    pub page: u16,
}

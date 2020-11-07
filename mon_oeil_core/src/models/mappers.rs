use super::*;

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

impl From<db::Meaning> for Meaning {
    fn from(item: db::Meaning) -> Self {
        let db::Meaning { id, value, langs } = item;
        Self { id, value, langs }
    }
}

impl From<db::Picture> for Picture {
    fn from(item: db::Picture) -> Self {
        let db::Picture { id, langs } = item;
        Self { id, langs }
    }
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

impl Into<db::NewMeaning> for NewMeaning {
    fn into(self) -> db::NewMeaning {
        let Self { value, langs } = self;
        db::NewMeaning { value, langs }
    }
}

impl Into<db::NewPicture> for NewPicture {
    fn into(self) -> db::NewPicture {
        let Self { langs } = self;
        db::NewPicture { langs }
    }
}

impl From<db::DbError> for DbError {
    fn from(err: db::DbError) -> DbError {
        DbError(format!("{:?}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_gesture() {
        let meanings = vec![
            db::Meaning {
                id: "id_m1".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valuem1".to_owned(),
            },
            db::Meaning {
                id: "id_m2".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valuem2".to_owned(),
            },
        ];

        let descriptions = vec![
            db::Description {
                id: "id_d1".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valued1".to_owned(),
                meanings: vec![],
            },
            db::Description {
                id: "id_d2".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valued2".to_owned(),
                meanings,
            },
        ];

        let meanings = vec![
            db::Meaning {
                id: "id_m1".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valuem1".to_owned(),
            },
            db::Meaning {
                id: "id_m2".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                value: "valuem2".to_owned(),
            },
        ];

        let pictures = vec![
            db::Picture {
                id: "id_p1".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            },
            db::Picture {
                id: "id_p2".to_string(),
                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
            },
        ];

        let gesture = db::Gesture {
            id: "id_g1".to_string(),
            tags: vec!["tag1".to_owned(), "tag2".to_owned()],
            descriptions,
            meanings,
            pictures,
        };
        assert_eq!(
            Gesture {
                id: "id_g1".to_string(),
                tags: vec!["tag1".to_owned(), "tag2".to_owned()],
                descriptions: vec![
                    Description {
                        id: "id_d1".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valued1".to_owned(),
                        meanings: vec![],
                    },
                    Description {
                        id: "id_d2".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valued2".to_owned(),
                        meanings: vec![
                            Meaning {
                                id: "id_m1".to_string(),
                                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                                value: "valuem1".to_owned(),
                            },
                            Meaning {
                                id: "id_m2".to_string(),
                                langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                                value: "valuem2".to_owned(),
                            }
                        ],
                    },
                ],
                meanings: vec![
                    Meaning {
                        id: "id_m1".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valuem1".to_owned(),
                    },
                    Meaning {
                        id: "id_m2".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                        value: "valuem2".to_owned(),
                    },
                ],
                pictures: vec![
                    Picture {
                        id: "id_p1".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                    },
                    Picture {
                        id: "id_p2".to_string(),
                        langs: vec!["lang1".to_owned(), "lang2".to_owned()],
                    },
                ],
            },
            Gesture::from(gesture)
        )
    }
}

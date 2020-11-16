use super::*;
use uuid::Uuid;

impl RawGesture {
    pub fn from(new: NewGesture, id_gesture: Uuid) -> Self {
        let NewGesture { tags, .. } = new;

        Self { id_gesture, tags }
    }
}

impl RawDescription {
    pub fn from(new: NewDescription, id_gesture: Uuid, id_description: Uuid) -> Self {
        let NewDescription { value, langs, .. } = new;

        Self {
            id_description,
            id_gesture,
            val: value,
            langs,
        }
    }
}

impl RawMeaning {
    pub fn from(
        new: NewMeaning,
        id_gesture: Option<Uuid>,
        id_description: Option<Uuid>,
        id_meaning: Uuid,
    ) -> Self {
        let NewMeaning { value, langs, .. } = new;

        Self {
            id_description,
            id_gesture,
            id_meaning,
            val: value,
            langs,
        }
    }
}

impl RawPicture {
    pub fn from(new: NewPicture, id_gesture: Uuid, id_picture: Uuid) -> Self {
        let NewPicture { langs, format, .. } = new;

        Self {
            id_picture,
            id_gesture,
            langs,
            format,
        }
    }
}

impl InnerGesture {
    pub fn from(new: NewGesture, id_gesture: Uuid) -> Self {
        let NewGesture { tags } = new;

        Self { id_gesture, tags }
    }
}

impl InnerDescription {
    pub fn from(new: NewDescription, id_description: Uuid) -> Self {
        let NewDescription { value, langs } = new;

        Self {
            id_description,
            val: value,
            langs,
        }
    }
}

impl InnerMeaning {
    pub fn from(new: NewMeaning, id_meaning: Uuid) -> Self {
        let NewMeaning { value, langs, .. } = new;

        Self {
            id_meaning,
            val: value,
            langs,
        }
    }
}

impl InnerPictureMeta {
    pub fn from(new: NewPictureMeta, id_picture: Uuid) -> Self {
        let NewPictureMeta { langs } = new;

        Self { id_picture, langs }
    }
}

impl PictureFileInfo {
    pub fn from(new: NewPictureFileInfo, id_picture: Uuid) -> Self {
        let NewPictureFileInfo { format } = new;

        Self { id_picture, format }
    }
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

impl Picture {
    pub fn from_raw(raw: RawPicture) -> Self {
        let RawPicture {
            id_picture,
            langs,
            format,
            ..
        } = raw;
        Self {
            id: format!("{}", id_picture),
            langs,
            format,
        }
    }
}

impl User {
    pub fn from_raw(raw: RawUser) -> Self {
        let RawUser {
            username, password, ..
        } = raw;
        Self { username, password }
    }
}

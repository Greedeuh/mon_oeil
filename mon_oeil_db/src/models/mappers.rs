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
        let NewPicture { langs, .. } = new;

        Self {
            id_picture,
            id_gesture,
            langs,
        }
    }
}

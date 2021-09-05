use crate::utils::table::AVATARS;

pub fn get_avatar(name: &str) -> &str {
    if let Some(id) = AVATARS.iter().position(|item| item.0 == name) {
        AVATARS[id].1
    } else {
        ""
    }
}
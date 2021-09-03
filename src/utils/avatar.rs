use crate::utils::table::AVATARS;

pub fn get_avatar(name: &str) -> &str {
    if let Some(id) = AVATARS.get(name) {
        return id
    } else {
        return ""
    }
}
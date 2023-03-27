pub fn is_valid_password(password: &str) -> bool {
    let restricted_chars = ['\\', '/', ':', ';', '\"', '\''];
    password.chars().all(|ch| !restricted_chars.contains(&ch)) && password.len() >= 8
}

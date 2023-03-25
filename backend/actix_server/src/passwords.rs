use mysql::Conn;

use crate::database;

pub fn is_valid_password(password: &str) -> bool {
    let restricted_chars = ['\\', '/', ':', ';', '\"', '\''];
    password.chars().all(|ch| !restricted_chars.contains(&ch)) && password.len() >= 8
}

pub fn check_password(connection: &mut Conn, email: &str, password: &str) -> (String, String) {
    let user = database::find_user(connection, &email, false);
    if user.is_none() {
        return ("FAILED".to_string(), "User does not exist".to_string());
    }
    let user = user.unwrap();
    if user.password == password {
        return ("OK".to_string(), "".to_string());
    } else {
        return ("FAILED".to_string(), "Invalid password".to_string());
    }
}

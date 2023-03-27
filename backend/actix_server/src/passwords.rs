use mysql::Conn;

use crate::database::{self};

pub fn is_valid_password(password: &str) -> bool {
    let restricted_chars = ['\\', '/', ':', ';', '\"', '\''];
    password.chars().all(|ch| !restricted_chars.contains(&ch)) && password.len() >= 8
}

pub fn check_password(connection: &mut Conn, email: &str, password: &str) -> (String, String) {
    let user = database::find_user_by_email(connection, &email);
    if user.is_none() {
        return ("FAILED".to_owned(), "User does not exist".to_owned());
    }
    let user = user.unwrap();
    if user.password == password {
        return ("OK".to_owned(), "".to_owned());
    } else {
        return ("FAILED".to_owned(), "Invalid password".to_owned());
    }
}

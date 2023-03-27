use crate::{passwords::is_valid_password, email::is_valid_email, database::{self, User, DBconfig, try_connect}};
use mysql::Conn;

#[derive(Debug)]
pub enum AuthError {
    InvalidEmail,
    InvalidPassword,
    FailedToConnect,
    UserDoesNotExist,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidEmail => write!(f, "Invalid email"),
            AuthError::InvalidPassword => write!(f, "Invalid password"),
            AuthError::FailedToConnect => write!(f, "Failed to connect to database"),
            AuthError::UserDoesNotExist => write!(f, "User does not exist"),
        }
    }
}

pub fn auth_get_user_connect(
    email: &str,
    password: &str,
    db_config: &DBconfig,
    retries: u32,
) -> Result<(User, Conn), AuthError> {
    if !is_valid_email(email) {
        return Err(AuthError::InvalidEmail);
    }
    if !is_valid_password(password) {
        return Err(AuthError::InvalidPassword);
    }
    let mut connection = match try_connect(db_config, retries) {
        Ok(connection) => connection,
        Err(err) => return Err(AuthError::FailedToConnect),
    };
    let user = match database::find_user_by_email(&mut connection, &email) {
        Some(user) => user,
        None => return Err(AuthError::UserDoesNotExist),
    };
    if user.password != password {
        return Err(AuthError::InvalidPassword);
    }
    Ok((user, connection))
}



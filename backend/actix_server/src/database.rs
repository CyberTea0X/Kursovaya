use crate::email;
use crate::register::RegisterInfo;
use mysql;
use mysql::prelude::Queryable;
use mysql::{params, Conn, OptsBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
#[derive(Serialize, Deserialize, Clone)]
pub struct DBconfig {
    pub ip: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub rating: u32,
    pub about: Option<String>,
    pub age: Option<String>,
    pub gender: Option<String>,
    pub last_online: Option<String>,
    pub reg_date: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct EditRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub rating: Option<u32>,
    pub about: Option<String>,
    pub age: Option<String>,
    pub gender: Option<String>,
    pub reg_date: Option<String>,
}

pub fn get_all_users(
    connection: &mut Conn,
    hide_passwords: bool,
) -> Result<Vec<User>, mysql::Error> {
    connection.query_map(
        r"SELECT id, username, email, password, firstname,
    lastname, rating, about, age, gender, last_online, reg_date
        FROM `users`",
        |(
            id,
            username,
            email,
            password,
            firstname,
            lastname,
            rating,
            about,
            age,
            gender,
            last_online,
            reg_date,
        )| {
            let password = if hide_passwords {
                "secret".to_string()
            } else {
                password
            };
            User {
                id,
                username,
                email,
                password,
                firstname,
                lastname,
                rating,
                about,
                age,
                gender,
                last_online,
                reg_date,
            }
        },
    )
}

pub fn is_valid_sql(text: &str) -> bool {
    let restricted_chars = ['\\', '/', ':', ';', '\"', '\''];
    text.chars().all(|ch| !restricted_chars.contains(&ch))
}

pub fn connect_to_db(db_config: &DBconfig) -> Result<Conn, mysql::Error> {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(&db_config.ip))
        .tcp_port(db_config.port)
        .user(Some(&db_config.user))
        .pass(Some(&db_config.password))
        .db_name(Some(&db_config.database));
    Conn::new(opts)
}

pub fn register_user(
    connection: &mut Conn,
    username: &str,
    email: &str,
    password: &str,
    info: &RegisterInfo,
) -> Result<(), mysql::Error> {
    let register_date = chrono::offset::Local::now();
    let reg_date = register_date.format("%Y-%m-%d").to_string();
    let last_online = register_date.format("%Y-%m-%d %H-%M-%S").to_string();

    connection.exec_drop(
        r"INSERT INTO USERS (id, username, email, password, firstname,
        lastname, about, age, gender, last_online, reg_date)
        values (:id, :username, :email, :password, :firstname,
        :lastname, :about, :age, :gender, :last_online, :reg_date)",
        params! {
            "id" => None::<u32>,
            "username" => username,
            "email" => email,
            "password" => password,
            "firstname" => &info.firstname,
            "lastname" => &info.lastname,
            "about" => &info.about,
            "age" => &info.age,
            "gender" => &info.gender,
            "last_online" => last_online,
            "reg_date" => reg_date,
        },
    )
}

pub fn make_user_online(connection: &mut Conn, email: &str) -> Result<(), mysql::Error> {
    let last_online = chrono::offset::Local::now()
        .format("%Y-%m-%d %H-%M-%S")
        .to_string();
    connection.exec_drop(
        "UPDATE USERS SET last_online=:last_online WHERE email = :email",
        params! {
            "last_online" => last_online,
            "email" => email
        },
    )
}

pub fn edit_user(
    connection: &mut Conn,
    email: &str,
    info: &EditRequest,
) -> Result<(), mysql::Error> {
    let mut expression = format!(
        "UPDATE USERS SET {}{}{}{}{}{}{}{}{} WHERE email = :email",
        info.username.as_deref().map_or(String::new(), |u| format!("username=\"{}\", ", u)),
        info.email.as_deref().map_or(String::new(), |e| format!("email=\"{}\", ", e)),
        info.password.as_deref().map_or(String::new(), |p| format!("password=\"{}\", ", p)),
        info.firstname.as_deref().map_or(String::new(), |f| format!("firstname=\"{}\", ", f)),
        info.lastname.as_deref().map_or(String::new(), |l| format!("lastname=\"{}\", ", l)),
        info.rating.map_or(String::new(), |r| format!("rating=\"{}\", ", r)),
        info.about.as_deref().map_or(String::new(), |a| format!("about=\"{}\", ", a)),
        info.age.as_deref().map_or(String::new(), |a| format!("age=\"{}\", ", a)),
        info.gender.as_deref().map_or(String::new(), |g| format!("gender=\"{}\", ", g))
    );
    let trailing_comma = expression.rfind(',').unwrap();
    expression.remove(trailing_comma);
    connection.exec_drop(
        expression,
        params! {
            "email" => email,
        },
    )
}

pub fn delete_user(connection: &mut Conn, email: &str) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "DELETE FROM USERS
    WHERE email=:email",
        params! {
            "email" => email
        },
    )
}

pub fn user_exists(connection: &mut Conn, email: &str) -> bool {
    match connection.query::<String, String>(format!(
        "SELECT `email` FROM `users` WHERE email = \"{}\"",
        email
    )) {
        Ok(registered) => !registered.is_empty(),
        Err(err) => {
            println!("{:?}", err);
            true
        }
    }
}

pub fn visit_exists(connection: &mut Conn, visitor_email: &str, visiting_id: u32) -> bool {
    match connection.query::<String, String>(format!(
        "SELECT `visitor_email` FROM `visits` WHERE visitor_email = \"{}\" AND visiting_id = \"{}\"",
        visitor_email, visiting_id
    )) {
        Ok(visitors) => !visitors.is_empty(),
        Err(err) => {
            println!("{:?}", err);
            true
        }
    }
}

pub fn add_visit(
    connection: &mut Conn,
    visitor_email: &str,
    visiting_id: u32,
) -> Result<(), mysql::Error> {
    let visit_date = chrono::offset::Local::now();
    let visit_date = visit_date.format("%Y-%m-%d").to_string();
    connection.exec_drop(
        r"INSERT INTO VISITS (visitor_email, visiting_id, visit_date)
        values (:visitor_email, :visiting_id, :visit_date)",
        params! {
            "visitor_email" => visitor_email,
            "visiting_id" => visiting_id,
            "visit_date" => visit_date
        },
    )
}

pub fn find_user(connection: &mut Conn, email: &str, hide_password: bool) -> Option<User> {
    let query_result = get_all_users(connection, hide_password);
    if query_result.is_err() {
        return None;
    }
    let query_result = query_result.unwrap();
    query_result.into_iter().find(|user| user.email == email)
}

pub fn parse_config() -> DBconfig {
    let mut file = File::open("DB2config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let db_config: DBconfig = serde_json::from_str(&data).unwrap();
    db_config
}

// pub fn try_reconnect(connection: &mut Conn) -> bool{
//     let mut connected = false;
//     for _ in 0..3 {
//         if connection.reset().is_ok() {
//             connected = true;
//             break;
//         }
//     }
//     connected
// }

pub fn try_connect(db_config: &DBconfig, number_of_retries: u32) -> Result<Conn, mysql::Error> {
    for i in 0..(number_of_retries) {
        let connection = connect_to_db(db_config);
        if connection.is_ok() {
            return connection;
        }
        println!("Trying to connect to database. Retries: {}", i)
    }
    println!(
        "Trying to connect to database. Retries: {}",
        number_of_retries
    );
    return connect_to_db(db_config);
}

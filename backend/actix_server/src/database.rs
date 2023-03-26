use crate::email;
use crate::register::RegisterInfo;
use mysql;
use mysql::prelude::{FromRow, Queryable};
use mysql::{params, Conn, OptsBuilder};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone)]
pub struct FindUserRequest {
    email: Option<String>,
    id: Option<u32>,
}

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

impl FromRow for User {
    fn from_row(row: mysql::Row) -> Self {
        Self {
            id: row.get("id").unwrap(),
            username: row.get("username").unwrap(),
            email: row.get("email").unwrap(),
            password: row.get("password").unwrap(),
            firstname: row.get("firstname").unwrap(),
            lastname: row.get("lastname").unwrap(),
            rating: row.get("rating").unwrap(),
            about: row.get("about").unwrap(),
            age: row.get("age").unwrap(),
            gender: row.get("gender").unwrap(),
            last_online: row.get("last_online").unwrap(),
            reg_date: row.get("reg_date").unwrap(),
        }
    }
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (
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
        ) = mysql::from_row_opt(row)?;
        Ok(Self {
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
        })
    }
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Chat {
    id: u32,
    userid1: u32,
    userid2: u32,
    created_at: Option<String>,
}

pub fn get_user_chats(connection: &mut Conn, userid1: u32) -> Result<Vec<Chat>, mysql::Error> {
    connection.query_map(
        format!(
            "SELECT * FROM chats WHERE userid1 = {0} OR userid2 = {0}",
            userid1
        ),
        |(id, userid1, userid2, created_at)| Chat {
            id,
            userid1,
            userid2,
            created_at,
        },
    )
}

pub fn create_chat(connection: &mut Conn, userid1: u32, userid2: u32) -> Result<(), mysql::Error> {
    let created_at = chrono::offset::Local::now().format("%Y-%m-%d").to_string();
    connection.exec_drop(
        "INSERT INTO chats (userid1, userid2, created_at) VALUES (:userid1, :userid2, :created_at)",
        params! {
            "userid1" => userid1,
            "userid2" => userid2,
            "created_at" => created_at,
        },
    )
}

pub fn delete_chat(connection: &mut Conn, userid1: u32, userid2: u32) -> Result<(), mysql::Error> {
    connection.exec_drop(
        r"DELETE FROM chats WHERE (userid1 = :userid1 AND userid2 = :userid2)
        OR (userid1 = :userid2 AND userid2 = :userid1)",
        params! {
            "userid1" => userid1,
            "userid2" => userid2
        },
    )
}

pub fn is_chat_exists(connection: &mut Conn, userid1: u32, userid2: u32) -> bool {
    let chat = find_chats(connection, userid1, userid2);
    if chat.is_err() {
        println!("{:?}", chat);
        return true;
    }
    chat.unwrap().last().is_some()
}

pub fn find_chats(
    connection: &mut Conn,
    userid1: u32,
    userid2: u32,
) -> Result<Vec<Chat>, mysql::Error> {
    connection.query_map(
        format!("SELECT id, userid1, userid2, created_at FROM chats WHERE (userid1 = {0} AND userid2 = {1})
        OR (userid1 = {1} AND userid2 = {0})", userid1, userid2),
        |(
            id,
            userid1,
            userid2,
            created_at,
        )|
            Chat {
                id,
                userid1,
                userid2,
                created_at,
                ..Default::default()
            }
        )
}

pub fn get_all_users(
    connection: &mut Conn,
    hide_passwords: bool,
) -> Result<Vec<User>, mysql::Error> {
    connection.query_map(
        r"SELECT id, username, email, password, firstname,
    lastname, rating, about, age, gender, last_online, reg_date
        FROM `users`",
        |user| {
            let mut user: User = user;
            if hide_passwords {
                user.password = "secret".to_owned();
            }
            user
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
        info.username
            .as_deref()
            .map_or(String::new(), |u| format!("username=\"{}\", ", u)),
        info.email
            .as_deref()
            .map_or(String::new(), |e| format!("email=\"{}\", ", e)),
        info.password
            .as_deref()
            .map_or(String::new(), |p| format!("password=\"{}\", ", p)),
        info.firstname
            .as_deref()
            .map_or(String::new(), |f| format!("firstname=\"{}\", ", f)),
        info.lastname
            .as_deref()
            .map_or(String::new(), |l| format!("lastname=\"{}\", ", l)),
        info.rating
            .map_or(String::new(), |r| format!("rating=\"{}\", ", r)),
        info.about
            .as_deref()
            .map_or(String::new(), |a| format!("about=\"{}\", ", a)),
        info.age
            .as_deref()
            .map_or(String::new(), |a| format!("age=\"{}\", ", a)),
        info.gender
            .as_deref()
            .map_or(String::new(), |g| format!("gender=\"{}\", ", g))
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

pub fn find_user(
    connection: &mut Conn,
    email: Option<&str>,
    id: Option<u32>,
    hide_password: bool,
) -> Option<User> {
    email
        .and_then(|e| find_user_by_email(connection, e, hide_password))
        .or_else(|| id.and_then(|i| find_user_by_id(connection, i, hide_password)))
}

pub fn find_user_by_id(connection: &mut Conn, id: u32, hide_password: bool) -> Option<User> {
    match get_all_users(connection, hide_password) {
        Ok(users) => users.into_iter().find(|user| user.id == id),
        Err(_) => None,
    }
}

pub fn find_user_by_email(connection: &mut Conn, email: &str, hide_password: bool) -> Option<User> {
    match get_all_users(connection, hide_password) {
        Ok(users) => users.into_iter().find(|user| user.email == email),
        Err(_) => None,
    }
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

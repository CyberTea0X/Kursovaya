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
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub logo_id: String,
    pub rating: String,
    pub about_user: String,
    pub chats_folder: String,
    pub login: String,
    pub password: String,
    pub reg_date: String,
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
    first_name: &str,
    last_name: &str,
    password: &str,
    email: &str,
    logo: &str,
    about: &str,
) -> bool {
    let register_date = chrono::offset::Local::now();
    let reg_date = register_date.format("%Y-%m-%d").to_string();
    let status = connection.exec_drop(
        r"INSERT INTO USERS (id, first_name, last_name, email, logo_id,
        raiting, about_user, chats_folder, login, password, reg_date)
        values (:id, :first_name, :last_name, :email, :logo_id, :rating, :about_user, :chats_folder,
         :login, :password, :reg_date)",
        params! {
            "id" => "",
            "first_name" => first_name,
            "last_name" => last_name,
            "email" => email,
            "logo_id" => logo,
            "rating" => "0",
            "about_user" => about,
            "chats_folder" => "CHAT_FOLDER_TO_DO",
            "login" => "?",
            "password" => password,
            "reg_date" => reg_date,
        },
    );
    if status.is_err() {
        println!("{:?}", status);
        return false;
    }
    true
}

pub fn user_exists(
    connection: &mut Conn,
    email: &str
) -> bool {
    match connection.query::<String, String>(
        format!("SELECT `email` FROM `users` WHERE email = \"{}\"", email)
    ) {
        Ok(registered) => {
            !registered.is_empty()
        }
        Err(err) => {
            println!("{:?}", err);
            true
        }
    } 
 
}

pub fn find_user(
    connection: &mut Conn,
    email: &str
) -> Option<User> {
    let query_result = connection.query_map(
        format!("SELECT id, first_name, last_name, email, logo_id,
            raiting, about_user, chats_folder, login, password, reg_date 
            FROM `users` WHERE email = \"{}\"", email),
        |(
            id, first_name, last_name, email, logo_id,
            rating, about_user, chats_folder, login, password, reg_date
        )| {
            User {id, first_name, last_name, email, logo_id, rating, about_user, chats_folder, login, password, reg_date}
        },
    );
    match query_result {
        Ok(mut query) => {
            let user = query.pop();
            if user.is_some() {
                return Some(user.unwrap());
            }
            return None;
        }
        Err(err) => {
            println!("{:?}", err);
            return None;
        }
    }
 
}

pub fn parse_config() -> DBconfig {
    let mut file = File::open("DBconfig.json").unwrap();
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

pub fn try_connect(
    db_config: &DBconfig,
    number_of_retries: u32
) -> Result<Conn, mysql::Error>
{
    for i in 0..(number_of_retries) {
        let connection = connect_to_db(db_config);
        if connection.is_ok() {
            return connection;
        }
        println!("Trying to connect to database. Retries: {}", i)
    }
    println!("Trying to connect to database. Retries: {}", number_of_retries);
    return connect_to_db(db_config);
}

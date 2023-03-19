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

pub fn register_user(
    db_config: DBconfig,
    first_name: &str,
    last_name: &str,
    password: &str,
    email: &str,
    logo: &str,
    about: &str,
) -> bool {
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(db_config.ip))
        .tcp_port(db_config.port)
        .user(Some(db_config.user))
        .pass(Some(db_config.password))
        .db_name(Some(db_config.database));
    let register_date = chrono::offset::Local::now();
    let reg_date = register_date.format("%Y-%m-%d").to_string();
    let connection = Conn::new(opts);
    if connection.is_err() {
        return false;
    }
    let mut connection = connection.unwrap();
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

pub fn parse_config() -> DBconfig {
    let mut file = File::open("DBconfig.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let db_config: DBconfig = serde_json::from_str(&data).unwrap();
    db_config
}

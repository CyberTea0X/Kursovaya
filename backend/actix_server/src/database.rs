use crate::register::RegisterInfo;
use mysql::prelude::{FromRow, Queryable};
use mysql::{self, from_row, Params};
use mysql::{params, Conn, OptsBuilder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EditUserRequest {
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
    pub id: u32,
    pub userid1: u32,
    pub userid2: u32,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Message {
    pub id: u32,
    pub chat_id: u32,
    pub content: String,
    pub owner_id: u32,
    pub owner_name: String,
    pub send_time: String,
    pub is_read: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ImageData {
    pub id: u32,
    pub owner_id: u32,
    pub published_at: String,
    pub about: String,
    pub image_name: String,
    pub extension: String,
    pub tags: String,
    pub views: u32,
    pub likes: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EditImageRequest {
    pub about: Option<String>,
    pub image_name: Option<String>,
    pub extension: Option<String>,
    pub tags: Option<String>,
    pub views: Option<u32>,
    pub likes: Option<u32>,
}

impl FromRow for Message {
    fn from_row(row: mysql::Row) -> Self {
        let (id, chat_id, content, owner_id, owner_name, send_time, is_read) = mysql::from_row(row);
        Message {
            id,
            chat_id,
            content,
            owner_id,
            owner_name,
            send_time,
            is_read,
        }
    }

    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, chat_id, content, owner_id, owner_name, send_time, is_read) =
            mysql::from_row_opt(row)?;
        Ok(Self {
            id,
            chat_id,
            content,
            owner_id,
            owner_name,
            send_time,
            is_read,
        })
    }
}

pub struct TagInfo {
    pub id: u64,
    pub user_id: u64,
    pub tags: String,
}

impl FromRow for TagInfo {
    fn from_row_opt(row: mysql::Row) -> Result<Self, mysql::FromRowError>
    where
        Self: Sized,
    {
        let (id, user_id, tags) = mysql::from_row_opt(row)?;
        Ok(Self { id, user_id, tags })
    }
    fn from_row(row: mysql::Row) -> Self
    where
        Self: Sized,
    {
        Self::from_row_opt(row).unwrap()
    }
}
pub fn user_tags_exists(connection: &mut Conn, user_id: u32) -> Result<bool, mysql::Error> {
    Ok(get_user_tags(connection, user_id)?.is_some())
}

pub fn get_user_tags_table(connection: &mut Conn) -> Result<Vec<TagInfo>, mysql::Error> {
    connection.query("SELECT * FROM `user_tags`")
}

pub fn get_user_tags(connection: &mut Conn, user_id: u32) -> Result<Option<String>, mysql::Error> {
    let query = format!(
        "SELECT tags FROM `user_tags` WHERE user_id = {} LIMIT 1",
        user_id
    );
    connection.query_first(query)
}

pub fn get_users_tags(
    connection: &mut Conn,
    user_ids: &[u32],
) -> Result<Vec<(u32, Vec<String>)>, mysql::Error> {
    let query = format!(
        "SELECT user_id, tags FROM user_tags WHERE user_id IN ({})",
        user_ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
    connection.query_map(query, |(user_id, tags): (u32, String)| {
        let tags = tags.split(",").map(|tag| tag.to_string()).collect();
        (user_id, tags)
    })
}

pub fn delete_user_tags(connection: &mut Conn, user_id: u32) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "DELETE FROM `user_tags` WHERE user_id = :id",
        params! {
            "id" => user_id
        },
    )
}

pub fn add_user_tags(connection: &mut Conn, user_id: u32, tags: &str) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "INSERT INTO `user_tags` (user_id, tags) VALUES (?, ?);",
        (user_id, tags),
    )
}

// pub fn get_logo_id(connection: &mut Conn, owner_id: u32) -> Result<Option<u32>, mysql::Error> {
//     let query = format!(
//         "SELECT id FROM `logos` WHERE owner_id = {} LIMIT 1",
//         owner_id
//     );
//     connection.query_first(query)
// }

pub fn get_logo_image_id(connection: &mut Conn, owner_id: u32) -> Result<Option<u32>, mysql::Error> {
    let query = format!(
        "SELECT image_id FROM `logos` WHERE owner_id = {} LIMIT 1",
        owner_id
    );
    connection.query_first(query)
}

pub fn set_logo(connection: &mut Conn, img_id: u32, owner_id: u32) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "INSERT INTO `logos` (image_id, owner_id) VALUES (?, ?);",
        (img_id, owner_id),
    )
}

pub fn delete_logo(connection: &mut Conn, owner_id: u32) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "DELETE FROM `logos` WHERE owner_id = :owner",
        params! {
            "owner" => owner_id
        },
    )
}

pub fn edit_image(
    connection: &mut Conn,
    image_id: u64,
    info: &EditImageRequest,
) -> Result<(), mysql::Error> {
    let mut params = Vec::new();
    let mut clauses = Vec::new();
    if let Some(about) = &info.about {
        clauses.push("about = ?");
        params.push(about);
    }
    if let Some(image_name) = &info.image_name {
        clauses.push("image_name = ?");
        params.push(image_name);
    }

    if let Some(extension) = &info.extension {
        clauses.push("extension = ?");
        params.push(extension);
    }
    if let Some(tags) = &info.tags {
        clauses.push("tags = ?");
        params.push(tags);
    }
    let views = info.views.map(|v| v.to_string());
    if let Some(views) = &views {
        clauses.push("views = ?");
        params.push(views);
    }

    let likes = info.likes.map(|l| l.to_string());
    if let Some(likes) = &likes {
        clauses.push("likes = ?");
        params.push(likes);
    }
    if params.is_empty() {
        return Ok(());
    }
    let set_clause = clauses.join(",");
    let query = format!("UPDATE `images` SET {} WHERE id = {}", set_clause, image_id);
    connection.exec_drop(query, params)
}

pub fn get_image(connection: &mut Conn, image_id: u64) -> Result<Option<ImageData>, mysql::Error> {
    let query = format!("SELECT * FROM `images` WHERE id = '{}' LIMIT 1", image_id);
    Ok(connection
        .query_map(
            query,
            |(id, owner_id, published_at, about, image_name, extension, tags, views, likes)| {
                ImageData {
                    id,
                    owner_id,
                    published_at,
                    about,
                    image_name,
                    extension,
                    tags,
                    views,
                    likes,
                }
            },
        )?
        .pop())
}

pub fn delete_image(connection: &mut Conn, id: u64) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "DELETE FROM `images` WHERE id = :id",
        params! {
            "id" => id
        },
    )
}

pub fn get_images(connection: &mut Conn, owner_id: u32) -> Result<Vec<ImageData>, mysql::Error> {
    let query = format!("SELECT * FROM `images` WHERE owner_id = '{}'", owner_id);
    Ok(connection.query_map(
        query,
        |(id, owner_id, published_at, about, image_name, extension, tags, views, likes)| {
            ImageData {
                id,
                owner_id,
                published_at,
                about,
                image_name,
                extension,
                tags,
                views,
                likes,
            }
        },
    )?)
}

pub fn add_image(
    connection: &mut Conn,
    owner_id: u32,
    about: &str,
    image_name: &str,
    extension: &str,
    tags: &str,
) -> Result<(), mysql::Error> {
    let published_at = chrono::offset::Local::now()
        .format("%Y-%m-%d %H-%M-%S")
        .to_string();
    connection.exec_drop(
        "INSERT INTO images (owner_id, published_at, about, image_name, extension, tags, views, likes)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        (owner_id, published_at, about, image_name, extension, tags, 0, 0),
    )
}

pub fn mark_messages_as_read(
    connection: &mut Conn,
    chat_id: u32,
    messages: &Vec<u32>,
) -> Result<(), mysql::Error> {
    if messages.is_empty() {
        return Ok(());
    }
    let items = messages
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let stmt = format!(
        "UPDATE `messages` SET is_read=true WHERE id IN ({}) AND chat_id=:chat_id",
        items
    );
    connection.exec_drop(
        stmt,
        params! {
            chat_id,
        },
    )
}

pub fn read_all_messages(
    connection: &mut Conn,
    chat_id: u32,
    owner_id: u32,
) -> Result<(), mysql::Error> {
    let stmt = "UPDATE `messages` SET is_read=true WHERE chat_id=:chat_id AND owner_id=:owner_id";
    connection.exec_drop(
        stmt,
        params! {
            chat_id,
            owner_id
        },
    )
}

pub fn get_messages(
    connection: &mut Conn,
    chat_id: u32,
    start: Option<usize>,
    end: Option<usize>,
) -> Result<Vec<Message>, mysql::Error> {
    let query = format!("SELECT * FROM `messages` WHERE chat_id={}", chat_id);
    let mut messages = connection.query(query)?;
    let len = messages.len();
    let start = start.unwrap_or(0).min(len);
    let end = end.unwrap_or(len).min(len);
    let messages = messages.drain(start..end).collect();
    Ok(messages)
}

pub fn send_message(
    connection: &mut Conn,
    chat_id: u32,
    owner_id: u32,
    owner_name: &str,
    content: &str,
) -> Result<(), mysql::Error> {
    let send_time = chrono::offset::Local::now()
        .format("%Y-%m-%d %H-%M-%S")
        .to_string();
    connection.exec_drop(
        "INSERT INTO `messages` (chat_id, content, owner_id, owner_name, send_time, is_read)
        VALUES(:chat_id, :content, :owner_id, :owner_name, :send_time, :is_read)",
        params! {
            chat_id,
            content,
            owner_id,
            owner_name,
            send_time,
            "is_read" => false,
        },
    )
}

pub fn count_unread_messages(connection: &mut Conn, chat_id: u32) -> Result<u32, mysql::Error> {
    let query = format!("SELECT * FROM `messages` WHERE chat_id={} AND is_read=false", chat_id);
    let messages : Vec<mysql::Row> = connection.query(query)?;
    Ok(messages.len() as u32)
}

pub fn get_last_chat_message(connection: &mut Conn, chat_id: u32) -> Result<Option<Message>, mysql::Error> {
    let query = format!("SELECT * FROM `messages` WHERE chat_id={} ORDER BY id DESC LIMIT 1", chat_id);
    let message = connection.query_first(query)?;
    Ok(message)
}

pub fn get_user_chats(connection: &mut Conn, userid1: u32) -> Result<Vec<Chat>, mysql::Error> {
    connection.query_map(
        format!(
            "SELECT * FROM `chats` WHERE userid1 = {0} OR userid2 = {0}",
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
        "INSERT INTO `chats` (userid1, userid2, created_at) VALUES (:userid1, :userid2, :created_at)",
        params! {
            "userid1" => userid1,
            "userid2" => userid2,
            "created_at" => created_at,
        },
    )
}

pub fn delete_chat(connection: &mut Conn, userid1: u32, userid2: u32) -> Result<(), mysql::Error> {
    connection.exec_drop(
        r"DELETE FROM `chats` WHERE (userid1 = :userid1 AND userid2 = :userid2)
        OR (userid1 = :userid2 AND userid2 = :userid1)",
        params! {
            "userid1" => userid1,
            "userid2" => userid2
        },
    )
}

pub fn is_chat_exists(connection: &mut Conn, userid1: u32, userid2: u32) -> bool {
    let chat = find_chat(connection, userid1, userid2);
    if chat.is_err() {
        println!("{:?}", chat);
        return true;
    }
    chat.unwrap().is_some()
}

pub fn find_chat(
    connection: &mut Conn,
    userid1: u32,
    userid2: u32,
) -> Result<Option<Chat>, mysql::Error> {
    Ok(find_chats(connection, userid1, userid2)?.pop())
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

pub fn user_email_to_id(connection: &mut Conn, email: &str) -> Result<Option<u32>, mysql::Error> {
    let query = format!("SELECT id FROM `users` WHERE email ='{}'", email);
    connection.query_first(query)
}

pub fn get_all_users(connection: &mut Conn) -> Result<Vec<User>, mysql::Error> {
    connection.query(
        r"SELECT id, username, email, password, firstname,
    lastname, rating, about, age, gender, last_online, reg_date
        FROM `users`",
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
        r"INSERT INTO `users` (id, username, email, password, firstname,
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
        "UPDATE `users` SET last_online=:last_online WHERE email = :email",
        params! {
            "last_online" => last_online,
            "email" => email
        },
    )
}

pub fn edit_user(
    connection: &mut Conn,
    email: &str,
    info: &EditUserRequest,
) -> Result<(), mysql::Error> {
    let mut set_clauses = Vec::new();
    let mut params = Vec::new();

    if let Some(username) = info.username.as_deref() {
        set_clauses.push("username = ?");
        params.push(username);
    }
    if let Some(email) = info.email.as_deref() {
        set_clauses.push("email = ?");
        params.push(email);
    }
    if let Some(password) = info.password.as_deref() {
        set_clauses.push("password = ?");
        params.push(password);
    }
    if let Some(firstname) = info.firstname.as_deref() {
        set_clauses.push("firstname = ?");
        params.push(firstname);
    }
    if let Some(lastname) = info.lastname.as_deref() {
        set_clauses.push("lastname = ?");
        params.push(lastname);
    }
    let rating = info.rating.map(|r| r.to_string());
    if let Some(rating) = rating.as_deref() {
        set_clauses.push("rating = ?");
        params.push(rating);
    }
    if let Some(about) = info.about.as_deref() {
        set_clauses.push("about = ?");
        params.push(about);
    }
    if let Some(age) = info.age.as_deref() {
        set_clauses.push("age = ?");
        params.push(age);
    }
    if let Some(gender) = info.gender.as_deref() {
        set_clauses.push("gender = ?");
        params.push(gender);
    }
    if params.is_empty() {
        return Ok(());
    }
    params.push(email);
    let set_clause = set_clauses.join(", ");
    let query = format!("UPDATE `users` SET {} WHERE email = ?", set_clause);

    connection.exec_drop(query, params)
}

pub fn delete_user(connection: &mut Conn, email: &str) -> Result<(), mysql::Error> {
    connection.exec_drop(
        "DELETE FROM `users`
    WHERE email=:email",
        params! {
            "email" => email
        },
    )
}

pub fn user_exists(connection: &mut Conn, identifier: &str) -> Result<bool, mysql::Error> {
    let query = if identifier.parse::<u32>().is_ok() {
        format!("SELECT id FROM users WHERE id = ?")
    } else {
        format!("SELECT email FROM users WHERE email = ?")
    };
    let user: Option<mysql::Row> = connection.exec_first(query, (identifier,))?;
    Ok(user.is_some())
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
        r"INSERT INTO `visits` (visitor_email, visiting_id, visit_date)
        values (:visitor_email, :visiting_id, :visit_date)",
        params! {
            "visitor_email" => visitor_email,
            "visiting_id" => visiting_id,
            "visit_date" => visit_date
        },
    )
}

pub fn find_user(connection: &mut Conn, email: Option<&str>, id: Option<u32>) -> Option<User> {
    email
        .and_then(|e| find_user_by_email(connection, e))
        .or_else(|| id.and_then(|i| find_user_by_id(connection, i)))
}

pub fn find_user_by_id(connection: &mut Conn, id: u32) -> Option<User> {
    match get_all_users(connection) {
        Ok(users) => users.into_iter().find(|user| user.id == id),
        Err(_) => None,
    }
}

pub fn find_user_by_email(connection: &mut Conn, email: &str) -> Option<User> {
    match get_all_users(connection) {
        Ok(users) => users.into_iter().find(|user| user.email == email),
        Err(_) => None,
    }
}

pub fn parse_config() -> DBconfig {
    match File::open("DBconfig.json") {
        Ok(file) => return parse_config_file(file),
        Err(_) => return parse_config_env(),
    };
}

pub fn parse_config_file(mut file: File) -> DBconfig {
    println!("Trying to get DB config from file...");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let db_config: DBconfig =
        serde_json::from_str(&data).expect("Конфиг базы данных имеет неверный формат");
    println!("Got DB config from file");
    db_config
}

pub fn parse_config_env() -> DBconfig {
    println!("Trying to get DB config from enviroment...");
    let ip = std::env::var("DB_IP").expect("DB_IP not set");
    let port = std::env::var("DB_PORT")
        .expect("DB_PORT not set")
        .parse::<u16>()
        .unwrap();
    let user = std::env::var("DB_USER").expect("DB_USER not set");
    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD not set");
    let database = std::env::var("DB_DATABASE").expect("DB_DATABASE not set");
    println!("Got DB config from enviroment");
    DBconfig {
        ip,
        port,
        user,
        password,
        database,
    }
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

DROP DATABASE IF EXISTS api_server;
CREATE DATABASE api_server;
USE api_server;
DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE,
    username VARCHAR(32) NOT NULL,
	email VARCHAR(256) UNIQUE NOT NULL,
    password VARCHAR(32) NOT NULL,
    firstname VARCHAR(32) NULL, 
    lastname VARCHAR(32) NULL,
    rating INTEGER NOT NULL DEFAULT 1000,
    about text NULL,
    age INTEGER NULL,
    gender VARCHAR(6) NULL,
    last_online DATETIME NOT NULL,
    reg_date DATE NOT NULL
);

DROP TABLE IF EXISTS visits;
CREATE TABLE visits (
    visitor_email VARCHAR(256) UNIQUE NOT NULL,
    visiting_id INTEGER NOT NULL,
    visit_date DATE NOT NULL,
    FOREIGN KEY (visiting_id)  REFERENCES users (id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS chats;
CREATE TABLE chats (
	id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE, 
	userid1 INTEGER NOT NULL,
    userid2 INTEGER NOT NULL,
    created_at DATE NOT NULL,
    FOREIGN KEY (userid1)  REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (userid2)  REFERENCES users (id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS messages;
CREATE TABLE messages (
	id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE, 
    chat_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    owner_id INTEGER NOT NULL,
    owner_name VARCHAR(32) NOT NULL,
    send_time DATETIME NOT NULL,
    is_read BOOL NOT NULL,
    FOREIGN KEY (chat_id)  REFERENCES chats (id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS images;
CREATE TABLE images (
	id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE, 
    owner_id INTEGER NOT NULL,
    published_at DATETIME NOT NULL,
    about TEXT NOT NULL,
    image_name TEXT NOT NULL,
    tags TEXT NOT NULL,
    views INTEGER NOT NULL,
    likes INTEGER NOT NULL,
    FOREIGN KEY (owner_id)  REFERENCES users (id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS logos;
CREATE TABLE logos (
	id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE, 
	image_id INTEGER NOT NULL,
    owner_id INTEGER NOT NULL,
    FOREIGN KEY (owner_id)  REFERENCES users (id),
    FOREIGN KEY (image_id)  REFERENCES images (id) ON DELETE CASCADE
);

DROP TABLE IF EXISTS user_tags;
CREATE TABLE user_tags(
	id INTEGER AUTO_INCREMENT PRIMARY KEY UNIQUE, 
	user_id INTEGER NOT NULL UNIQUE,
    tags TEXT NOT NULL,
    FOREIGN KEY (user_id)  REFERENCES users (id) ON DELETE CASCADE
);
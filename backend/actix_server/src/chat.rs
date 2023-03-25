use crate::database::{self, DBconfig, User};
use actix_web::{
    dev::ConnectionInfo, get, post, web, App, HttpResponse, HttpServer, Responder,
    Result as ActxResult,
};
use mysql::Conn;
use serde::{Deserialize, Serialize};
use serde_json::json;
use similar_string::compare_similarity;

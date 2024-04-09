use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

type ArticleId = i16;
type UserId = String;

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub password: String,
    pub introduction: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "article_type", rename_all = "lowercase")]
pub enum ArticleType {
    #[serde(rename(deserialize = "consultation"))]
    Consultation,
    #[serde(rename(deserialize = "experience"))]
    Experience,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Article {
    pub article_type: ArticleType,
    pub id: ArticleId,
    pub user_id: UserId,
    pub empathy: i16,
    pub nice: i16,
    pub title: Option<String>,
    pub text: String,
}

pub struct Comment {
    pub id: ArticleId,
    pub user_id: UserId,
    pub article_id: ArticleId,
    pub empathy: i16,
    pub nice: i16,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PostUser<T = UserId> {
    pub id: T,
    pub name: T,
}

#[derive(Debug, Deserialize)]
pub struct ArticlePost {
    pub article_type: ArticleType,
    pub user_id: UserId,
    pub title: Option<String>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct CommentPost {
    pub user_id: UserId,
    pub article_id: ArticleId,
    pub text: String,
}

#[derive(Serialize)]
pub struct Response<T: Serialize> {
    pub status: bool,
    pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub const fn new(status: bool, data: T) -> Response<T> {
        Response {
            status,
            data: Some(data),
        }
    }
    pub const fn ok(data: T) -> Response<T> {
        Response::new(true, data)
    }
    pub const fn error(data: T) -> Response<T> {
        Response::new(false, data)
    }
}

impl Response<bool> {
    pub const fn ok_none() -> Response<bool> {
        Response {
            status: true,
            data: None,
        }
    }
}

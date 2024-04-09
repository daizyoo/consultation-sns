use actix_web::{cookie::Cookie, web::Json, HttpRequest, HttpResponse};
use serde::Serialize;
use sqlx::{query, query_as, Postgres};
use tracing::error;

use crate::{
    types::{PostUser, Response, User},
    PoolD,
};

// Post
pub async fn create(pool: PoolD, Json(user): Json<User>) -> HttpResponse {
    println!("{:#?}", user);

    match query("INSERT INTO users ( id, name, password, introduction ) VALUES ( $1, $2, $3, $4 );")
        .bind(user.id.clone())
        .bind(user.name)
        .bind(user.password)
        .bind(user.introduction)
        .execute(&pool.0)
        .await
    {
        Ok(_) => {
            let mut cookie = Cookie::new("id", user.id);
            cookie.set_path("/");

            HttpResponse::Ok().cookie(cookie).json(Response::ok_none())
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::NotFound().json(Response::error("ユーザーの作成に失敗"))
        }
    }
}

// Get
pub async fn _login(pool: PoolD, req: HttpRequest) -> HttpResponse {
    let Some(cookie) = req.cookie("session_id") else {
        return HttpResponse::NotFound().into();
    };
    let id = cookie.value().to_string();

    match query_as::<Postgres, PostUser>("SELECT id, name FROM users WHERE id = $1;")
        .bind(id)
        .fetch_one(&pool.0)
        .await
    {
        Ok(user) => HttpResponse::Ok().json(Response::ok(user)),
        Err(e) => {
            error!("{}", e);
            HttpResponse::NotFound().json(Response::error("???サーバーエラー???"))
        }
    }
}

#[derive(Serialize)]
pub struct SearchUserResult {
    users: Vec<PostUser>,
}

pub async fn search(pool: PoolD, Json(user): Json<PostUser<Option<String>>>) -> HttpResponse {
    let result: Result<Vec<PostUser>, sqlx::Error> = match user {
        PostUser {
            id: Some(id),
            name: None,
        } => {
            query_as("SELECT id, name FROM users WHERE id = $1;")
                .bind(id)
                .fetch_all(&pool.0)
                .await
        }
        PostUser {
            id: None,
            name: Some(name),
        } => {
            query_as("SELECT id, name FROM users WHERE name = $1;")
                .bind(name)
                .fetch_all(&pool.0)
                .await
        }
        PostUser {
            id: Some(id),
            name: Some(name),
        } => {
            query_as("SELECT id, name FROM users WHERE id = $1 AND name = $2;")
                .bind(id)
                .bind(name)
                .fetch_all(&pool.0)
                .await
        }
        _ => return HttpResponse::NotFound().json(Response::error("query error")),
    };

    match result {
        Ok(users) => {
            println!("{:#?}", users);
            if users.len() == 0 {
                return HttpResponse::NotFound()
                    .json(Response::error("ユーザーが見つかりませんでした"));
            }

            HttpResponse::Ok().json(Response::ok(SearchUserResult { users }))
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::NotFound().json(Response::error("???サーバーエラー???"))
        }
    }
}

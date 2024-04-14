use actix_session::Session;
use actix_web::{web::Json, HttpResponse};
use serde::Deserialize;
use sqlx::{query, query_as, Postgres};
use tracing::{error, info};

use crate::{
    types::{PostUser, Response, User},
    PoolD,
};

// Post
pub async fn create(pool: PoolD, session: Session, Json(user): Json<User>) -> HttpResponse {
    if let Some(user_id) = session.get::<String>("user_id").ok() {
        if user_id.is_some() {
            return HttpResponse::Ok().json(Response::error("作成済"));
        }
    }

    info!("{:#?}", user);

    match query("INSERT INTO users ( id, name, password, introduction ) VALUES ( $1, $2, $3, $4 );")
        .bind(user.id.clone())
        .bind(user.name)
        .bind(user.password)
        .bind(user.introduction)
        .execute(&pool.0)
        .await
    {
        Ok(_) => {
            if let Err(e) = session.insert("user_id", user.id) {
                error!("{}", e);
                return HttpResponse::Ok().json(Response::error("session insert error"));
            }
            HttpResponse::Ok().json(Response::ok_none())
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::NotFound().json(Response::error("ユーザーの作成に失敗"))
        }
    }
}

// Get
pub async fn login_get(pool: PoolD, session: Session) -> HttpResponse {
    let id: String;
    if let Ok(user_id) = session.get::<String>("user_id") {
        if let Some(user_id) = user_id {
            println!("{}", user_id);
            id = user_id;
        } else {
            return HttpResponse::NotFound().json(Response::error("not found user_id"));
        }
    } else {
        return HttpResponse::NotFound().json(Response::error("session get user_id error"));
    }

    match query_as::<Postgres, PostUser>("SELECT id, name, introduction FROM users WHERE id = $1;")
        .bind(id)
        .fetch_one(&pool.0)
        .await
    {
        Ok(user) => {
            info!("{:#?}", user);
            HttpResponse::Ok().json(Response::ok(user))
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::NotFound().json(Response::error("サーバーエラー?"))
        }
    }
}

#[derive(Deserialize)]
pub struct Login {
    pub id: String,
    pub password: String,
}

pub async fn login_post(pool: PoolD, session: Session, Json(login): Json<Login>) -> HttpResponse {
    match query("SELECT id, password FROM users WHERE id = $1 AND password = $2")
        .bind(login.id.clone())
        .bind(login.password)
        .execute(&pool.0)
        .await
    {
        Ok(res) => {
            let row = res.rows_affected();
            println!("{}", row);
            if row == 0 {
                return HttpResponse::NotFound()
                    .json(Response::error("idまたはpasswordが違います"));
            }
            if let Err(e) = session.insert("user_id", login.id) {
                error!("{}", e)
            }
            HttpResponse::Ok().json(Response::ok_none())
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::Ok().json(Response::error("Error"))
        }
    }
}

pub async fn search(pool: PoolD, Json(user): Json<PostUser<Option<String>>>) -> HttpResponse {
    let result: Result<Vec<PostUser>, sqlx::Error> = match user {
        PostUser {
            id: Some(id),
            name: None,
            ..
        } => {
            query_as("SELECT id, name FROM users WHERE id = $1;")
                .bind(id)
                .fetch_all(&pool.0)
                .await
        }
        PostUser {
            id: None,
            name: Some(name),
            ..
        } => {
            query_as("SELECT id, name FROM users WHERE name = $1;")
                .bind(name)
                .fetch_all(&pool.0)
                .await
        }
        PostUser {
            id: Some(id),
            name: Some(name),
            ..
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
            info!("{:#?}", users);
            if users.len() == 0 {
                return HttpResponse::NotFound()
                    .json(Response::error("ユーザーが見つかりませんでした"));
            }

            HttpResponse::Ok().json(Response::ok(users))
        }
        Err(e) => {
            error!("{:?}", e);
            HttpResponse::NotFound().json(Response::error("???サーバーエラー???"))
        }
    }
}

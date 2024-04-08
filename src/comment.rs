use actix_web::{web::Json, HttpResponse};
use sqlx::query;
use tracing::error;

use crate::{
    types::{CommentPost, Response},
    PoolD,
};

pub async fn post(pool: PoolD, Json(comment): Json<CommentPost>) -> HttpResponse {
    let result = query!(
        "INSERT INTO comment ( user_id, article_id, text ) VALUES ( $1, $2, $3 )",
        comment.user_id,
        comment.article_id,
        comment.text
    )
    .execute(&pool.0)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(Response::ok_none()),
        Err(e) => {
            error!("{}", e);
            HttpResponse::NotFound().json(Response::error(format!(
                "存在しないuser_idまたはarticle_id? user_id={} article_id={}",
                comment.user_id, comment.article_id
            )))
        }
    }
}

pub async fn delete() -> HttpResponse {
    HttpResponse::NotFound().json(Response::error(""))
}

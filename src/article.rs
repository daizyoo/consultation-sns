use actix_web::{web::Json, HttpResponse};
use serde::Deserialize;
use sqlx::{query, query_as};
use urlencoding::decode;

use crate::{
    types::{Article, ArticlePost, ArticleType, Logic, Response},
    PoolD,
};

pub async fn post(pool: PoolD, Json(article): Json<ArticlePost>) -> HttpResponse {
    println!("{:#?}", article);
    let result = query!(
        r#"INSERT INTO article (article_type, user_id, title, text) VALUES ( $1, $2, $3, $4 )"#,
        article.article_type as ArticleType,
        article.user_id,
        article.title,
        article.text
    )
    .execute(&pool.0)
    .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(Response::ok_none()),
        Err(e) => {
            tracing::error!("{:?}", e);

            HttpResponse::NotFound().json(Response::error(format!(
                "存在しないユーザー？: id={}",
                article.user_id
            )))
        }
    }
}

pub async fn search(pool: PoolD, Json(option): Json<SearchArticle>) -> HttpResponse {
    match option.to_query(pool).await {
        Ok(articles) => {
            if articles.len() == 0 {
                return HttpResponse::NotFound().json(Response::error("見つかりませんでした"));
            }
            HttpResponse::Ok().json(Response::ok(articles))
        }
        Err(e) => {
            tracing::error!("{:?}", e);
            HttpResponse::NotFound().json(Response::error("見つかりませんでした"))
        }
    }
}

#[derive(Deserialize)]
pub struct SearchArticle {
    #[serde(rename(deserialize = "type"))]
    article_type: ArticleType,
    logic: Option<Logic>,
    empathy: Option<i16>,
    nice: Option<i16>,
    title: Option<String>,
    text: Option<String>,
}

impl SearchArticle {
    const QUERY_TEMP: &'static str = "SELECT * FROM article WHERE article_type = $1";

    async fn to_query(self, pool: PoolD) -> Result<Vec<Article>, sqlx::Error> {
        let logic = self.logic.unwrap_or(Logic::Or).string();
        let mut query = Self::QUERY_TEMP.to_string();
        let mut value_count = 2;

        if let Some(_) = self.empathy {
            query.push_str(&format!(" {} empathy = ${}", logic, value_count));
            value_count += 1;
        }
        if let Some(_) = self.nice {
            query.push_str(&format!(" {} nice = ${}", logic, value_count));
            value_count += 1;
        }
        if let Some(_) = self.title {
            query.push_str(&format!(" {} title LIKE ${}", logic, value_count));
            value_count += 1;
        }
        if let Some(_) = self.text {
            query.push_str(&format!(" {} text LIKE ${}", logic, value_count));
        }

        let mut query = query_as(query.as_str()).bind(self.article_type as ArticleType);
        if let Some(empathy) = self.empathy {
            query = query.bind(empathy)
        }
        if let Some(nice) = self.nice {
            query = query.bind(nice)
        }
        if let Some(title) = self.title {
            if let Ok(title) = decode(&title) {
                query = query.bind(format!("%{}%", title.to_string()))
            } else {
                tracing::error!("decode error {}", title)
            }
        }
        if let Some(text) = self.text {
            if let Ok(text) = decode(&text) {
                query = query.bind(format!("%{}%", text.to_string()))
            } else {
                tracing::error!("decode error {}", text)
            }
        }

        query.fetch_all(&pool.0).await
    }
}

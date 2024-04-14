use actix_cors::Cors;
use actix_files::Files;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{time::Duration, Key},
    middleware::Logger,
    web::{get, post, resource, scope, Data, ServiceConfig},
    App, HttpServer,
};
use sqlx::{PgPool, Pool, Postgres};

use tracing::info;
use tracing_subscriber::EnvFilter;

mod article;
mod comment;
mod types;
mod user;

type PoolD = Data<DBPool>;

struct DBPool(Pool<Postgres>);

const SESSION_LIFECYCLE: Duration = Duration::WEEK;

fn user_config(cfg: &mut ServiceConfig) {
    cfg.route("/create", post().to(user::create))
        .route("/search", get().to(user::search))
        .service(
            resource("/login")
                .route(get().to(user::login_get))
                .route(post().to(user::login_post)),
        );
}

fn article_config(cfg: &mut ServiceConfig) {
    cfg.route("/post", post().to(article::post))
        .route("/search", get().to(article::search));
}

fn comment_config(cfg: &mut ServiceConfig) {
    cfg.route("/post", post().to(comment::post));
}

// #[tokio::main(flavor = "multi_thread", worker_threads = 1)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    info!("starting http://127.0.0.1:3478");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("<%r> %s <%{User-Agent}i> %T"))
            .wrap(
                Cors::default()
                    // .allowed_origin("http://localhost:3478")
                    .allowed_methods(["GET", "POST"]),
            )
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .session_lifecycle(PersistentSession::default().session_ttl(SESSION_LIFECYCLE))
                    .cookie_name("session_id".to_owned())
                    .cookie_path("/".to_owned())
                    .cookie_secure(false)
                    .build(),
            )
            .service(
                scope("/api")
                    .service(scope("/user").configure(user_config))
                    .service(scope("/article").configure(article_config))
                    .service(scope("/comment").configure(comment_config)),
            )
            .service(Files::new("/script", "app/script/"))
            .service(Files::new("/style", "app/style/"))
            .service(Files::new("/user", "app/pages/user").index_file("index.html"))
            .service(Files::new("/article", "app/pages/article").index_file("index.html"))
            .service(Files::new("/", "app/pages").index_file("index.html"))
            .app_data(Data::new(DBPool(pool.clone())))
    })
    .bind(("127.0.0.1", 3478))?
    .workers(1)
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {}

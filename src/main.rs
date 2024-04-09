use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::{
    dev::{fn_service, ServiceRequest, ServiceResponse},
    middleware::Logger,
    web::{get, post, scope, Data},
    App, HttpServer,
};
use sqlx::{PgPool, Pool, Postgres};

use tracing::info;
use tracing_subscriber::EnvFilter;

mod article;
mod comment;
mod types;
mod user;

struct DBPool(Pool<Postgres>);

type PoolD = Data<DBPool>;

// #[tokio::main(flavor = "multi_thread", worker_threads = 1)]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .init();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;

    info!("http://127.0.0.1:3478");

    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://127.0.0.1:3478")
            .allowed_methods(["GET", "POST"]);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                scope("/api")
                    .service(
                        scope("/article")
                            .route("/post", post().to(article::post))
                            .route("/search", get().to(article::search)),
                    )
                    .service(
                        scope("/user")
                            .route("/create", post().to(user::create))
                            .route("/search", get().to(user::search)),
                    )
                    .service(scope("/comment").route("/post", post().to(comment::post))),
            )
            .service(Files::new("/script", "app/script/"))
            .service(Files::new("/style", "app/style/"))
            .service(Files::new("/user", "app/pages/user").index_file("index.html"))
            .service(Files::new("/", "app/pages").index_file("index.html"))
            .default_service(fn_service(|req: ServiceRequest| async {
                let (req, _) = req.into_parts();
                let file = NamedFile::open_async("../app/pages/404.html").await?;
                let res = file.into_response(&req);
                Ok(ServiceResponse::new(req, res))
            }))
            .app_data(Data::new(DBPool(pool.clone())))
    })
    .bind(("127.0.0.1", 3478))?
    .workers(1)
    .run()
    .await?;

    Ok(())
}

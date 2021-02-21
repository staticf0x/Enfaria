use crate::prelude::*;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use std::sync::Arc;
use tide::utils::After;

pub mod api;
pub mod error;
pub mod index;
pub mod login;
pub mod logout;
pub mod prelude;
pub mod register;
pub mod template;

#[derive(Clone)]
pub struct State {
    pub tera: Arc<Tera>,
    pub pool: Arc<MySqlPool>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    let tera = Arc::new(Tera::new("templates/*").unwrap());
    let pool = Arc::new(
        MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&env::var("DATABASE").unwrap())
            .await
            .expect("Failed to connect to database."),
    );

    let state = State { tera, pool };
    let mut app = tide::with_state(state);

    app.at("/static").serve_dir("static/").unwrap();

    api::routes(&mut app);
    index::routes(&mut app);
    login::routes(&mut app);
    logout::routes(&mut app);
    register::routes(&mut app);

    app.with(After(|res: Response| async move { error::handle_error(res).await }));

    app.listen("0.0.0.0:8000").await?;
    Ok(())
}

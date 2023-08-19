use std::env;
use std::error::Error;

pub async fn start_db() -> Result<sqlx::PgPool, Box<dyn Error>> {
  let user = env::var("DB_USER")?;
  let password = env::var("DB_PASSWORD")?;
  let host = env::var("DB_HOST")?;
  let name = env::var("DB_NAME")?;

  let url = format!("postgres://{:}:{:}@{:}/{:}", &user, &password, &host, &name);

  println!("url {}", &url);

  let pool = sqlx::postgres::PgPool::connect(&url).await?;

  sqlx::migrate!("./migrations").run(&pool).await?;

  Ok(pool)
}
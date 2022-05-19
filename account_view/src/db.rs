pub use self::db::*;

pub mod db {

  /*  pub async fn readiness_check(client: &Client) {
    client
      .database("admin")
      .run_command(doc! {"ping": 1}, None)
      .await
      .expect("DB connection should work");
  } */

  pub async fn create_client(postgres_host: &str) {}
}

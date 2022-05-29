pub use self::db::*;
use super::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub mod db {
  use super::*;

  use schema::accounts::dsl::*;

  pub async fn create_client(database_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
      .build(manager)
      .expect("Failed to create pool.")
  }

  pub async fn create_account(
    pool: Pool,
    item: models::InputAccount,
  ) -> Result<models::Account, diesel::result::Error> {
    let conn = pool.get().unwrap();
    let new_account = models::NewAccount {
      id: &Uuid::new_v4().to_string(),
      first_name: &item.first_name,
      last_name: &item.last_name,
      email: &item.email,
      created_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(accounts)
      .values(&new_account)
      .get_result(&conn)
  }

  pub async fn get_account(
    pool: Pool,
    _id: &str,
  ) -> Result<models::Account, diesel::result::Error> {
    let conn = pool.get().unwrap();
    accounts.find(_id).first::<models::Account>(&conn)
  }
}

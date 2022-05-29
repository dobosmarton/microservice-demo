// use chrono::{DateTime, Utc};
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputAccount {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Account {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
}

impl std::fmt::Display for Account {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let json_account = serde_json::to_string(&self).unwrap();
    write!(f, "{}", json_account)
  }
}

#[derive(Insertable, Debug)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
  pub id: &'a str,
  pub first_name: &'a str,
  pub last_name: &'a str,
  pub email: &'a str,
  pub created_at: chrono::NaiveDateTime,
}

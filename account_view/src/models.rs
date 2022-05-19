// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
}

use super::*;
use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use serde::{Deserialize, Serialize};

use juniper::{GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
#[graphql(description = "Account entity")]
pub struct Account {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New account")]
struct AccountInput {
  pub first_name: String,
  pub last_name: String,
  pub email: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
  async fn account(_id: String) -> FieldResult<Account> {
    let account = service::gateway_service::get_account(&_id)
      .await
      .expect("Couldn't find the account!");

    Ok(account)
  }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
  fn create_account(new_account: AccountInput) -> FieldResult<Account> {
    let date =
      chrono::NaiveDateTime::parse_from_str("2015-07-01 08:59:60.123", "%Y-%m-%d %H:%M:%S%.f")
        .expect("Date parsing failed");

    // Dummy data
    Ok(Account {
      id: String::from(""),
      first_name: new_account.first_name,
      last_name: new_account.last_name,
      email: new_account.email,
      created_at: date,
    })
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

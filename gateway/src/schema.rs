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

#[derive(GraphQLObject, Debug, Serialize, Deserialize)]
#[graphql(description = "Account id response object")]
pub struct AccountId {
  pub id: String,
}

#[derive(GraphQLInputObject, Debug, Serialize, Deserialize)]
#[graphql(description = "New account")]
pub struct AccountInput {
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
  async fn create_account(account: AccountInput) -> FieldResult<AccountId> {
    let account_id_obj = service::create_account(account)
      .await
      .expect("Account creation failed!");

    Ok(account_id_obj)
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}

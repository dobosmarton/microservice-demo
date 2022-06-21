pub use self::db::*;

pub mod db {

    use crate::models::AccountEvent;
    use mongodb::{bson::doc, error, options::ClientOptions, results::InsertOneResult, Client};

    pub async fn readiness_check(client: &Client) {
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .expect("DB connection should work");
    }

    pub async fn create_client(mongo_host: &str) -> Client {
        let options = ClientOptions::parse(mongo_host)
            .await
            .expect("Option parsing should work");

        let client = Client::with_options(options).expect("DB connection should work");

        readiness_check(&client).await;

        return client;
    }

    pub async fn insert_event(
        client: &Client,
        event_data: &str,
    ) -> Result<InsertOneResult, error::Error> {
        let account = client
            .database("events")
            .collection::<AccountEvent>("account");

        let new_event = AccountEvent {
            id: None,
            data: String::from(event_data),
            inserted_at: chrono::Utc::now(),
        };

        account.insert_one(new_event, None).await
    }
}

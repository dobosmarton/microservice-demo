pub use self::authentication_service::*;
use super::*;
use actix_web::{error::ErrorUnauthorized, HttpRequest};
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};

pub mod authentication_service {

    use super::*;

    pub async fn verify_jwt(
        _req: &HttpRequest,
    ) -> Result<TokenData<models::Claims>, actix_web::Error> {
        let _auth = _req.headers().get("Authorization");

        match _auth {
            Some(_) => {
                let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
                let token = _split[1].trim();

                let _var = config::Config::get_config().jwt_secret_key;
                let key = _var.as_bytes();

                match decode::<models::Claims>(
                    token,
                    &DecodingKey::from_secret(key),
                    &Validation::new(Algorithm::HS256),
                ) {
                    Ok(_token) => Ok(_token),
                    Err(_e) => Err(ErrorUnauthorized(_e)),
                }
            }
            None => Err(ErrorUnauthorized("blocked!")),
        }
    }
}

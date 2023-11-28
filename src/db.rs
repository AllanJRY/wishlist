use serde::Deserialize;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::{Jwt, Scope},
    Surreal,
};

use crate::auth::{AuthError, SigninCredentials, SignupCredentials};

#[derive(Debug, Clone, Deserialize)]
pub struct SurrealConfig {
    pub db_ns: String,
    pub db_name: String,
}

impl SurrealConfig {
    pub const PREFIX: &'static str = "SURREAL_";
}

#[derive(Debug, Clone)]
pub struct Db {
    endpoint: String,
    ns: String,
    name: String,
}

impl Db {
    pub fn new(endpoint: String, ns: String, name: String) -> Self {
        Db { endpoint, ns, name }
    }

    pub async fn connect(&self) -> Surreal<Client> {
        let db = Surreal::new::<Ws>("wishlist-db-prod:8000").await.unwrap();
        db.use_ns("prod").use_db("wishlist").await.unwrap();
        db
    }

    // TODO: a wrapper around the connectionsm this way a can have a GlobalConn and ScopedConn
    pub async fn connect_scope(&self, token: &str) -> Surreal<Client> {
        let db = Surreal::new::<Ws>("wishlist-db-prod:8000").await.unwrap();
        db.use_ns("prod").use_db("wishlist").await.unwrap();
        db.authenticate(token).await.unwrap();
        db
    }

    pub async fn signup(&self, signup_credentials: SignupCredentials) -> Result<(), AuthError> {
        let db = self.connect().await;
        match db
            .signup(Scope {
                namespace: self.ns.as_str(),
                database: self.name.as_str(),
                scope: "user_scope",
                params: signup_credentials,
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(AuthError::SignupFailed),
        }
    }

    pub async fn signin(&self, signin_credentials: SigninCredentials) -> Result<Jwt, AuthError> {
        let db = self.connect().await;
        match db
            .signin(Scope {
                namespace: self.ns.as_str(),
                database: self.name.as_str(),
                scope: "user_scope",
                params: signin_credentials,
            })
            .await
        {
            Ok(access_token) => Ok(access_token),
            Err(_) => Err(AuthError::InvalidCredentials),
        }
    }
}

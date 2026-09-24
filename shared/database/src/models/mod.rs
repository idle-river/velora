use std::env;

use axum_login::AuthnBackend;
use diesel::{
    ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
    r2d2::{ConnectionManager, Pool},
};
use password_auth::verify_password;
use dotenvy::dotenv;
use uuid::Uuid;

use crate::{models::user::User, schema::users::dsl::*};

pub mod user;

#[derive(Clone)]
pub struct Backend {
    pub db: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Clone)]
pub struct Credentials {
    pub user_id: String,
}

impl Backend {
    pub fn new() -> Self {
        Self {
            db: Self::establish_connection(),
        }
    }

    fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .build(manager)
            .expect("failed to create database connection pool")
    }
}

impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let parsed_id = Uuid::parse_str(&creds.user_id).ok();
        let mut conn = self.db.get().expect("failed to get DB connection");

        let result = if let Some(user_uuid) = parsed_id {
            users
                .filter(id.eq(user_uuid))
                .first::<User>(&mut conn)
                .optional()
                .expect("Error loading user")
        } else {
            None
        };

        Ok(result)
    }

    async fn get_user(
        &self,
        user_id: &axum_login::UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        let parsed_id = Uuid::parse_str(user_id).ok();
        let mut conn = self.db.get().expect("failed to get DB connection");

        let result = if let Some(user_uuid) = parsed_id {
            users
                .filter(id.eq(user_uuid))
                .first::<User>(&mut conn)
                .optional()
                .expect("Error loading user")
        } else {
            None
        };

        Ok(result)
}
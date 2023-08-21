//! This module contains the definition and implementation of the `User` struct.
//!
//! The `User` struct represents a user in the application. It stores information such as
//! user ID, name, email, password, wallet ID, and timestamps for creation and update.
//!
//! The module provides various methods for interacting with user data, including listing users,
//! finding users by ID or email, creating new users, updating user information, deleting users,
//! and handling user login.
//! 
//! # Examples
//! 
//! ```rust
//! use crate::models::user::User;
//!
//! // List all users in the database
//! let users = User::list(&mut connection);
//!
//! // Find a user by ID
//! if let Some(user) = User::find_by_id(&mut connection, "user_id".to_string()) {
//!     println!("Found user: {:?}", user);
//! }
//!
//! // Create a new user
//! if let Some(new_user) = User::create(&mut connection, "John Doe".to_string(), "john@example.com".to_string(), "wallet_id".to_string(), "password123".to_string()) {
//!     println!("Created new user: {:?}", new_user);
//! }
//!
//! // Update user information
//! if let Some(updated_user) = User::update(&mut connection, "user_id".to_string(), "New Name".to_string(), "newemail@example.com".to_string(), "new_wallet_id".to_string(), "new_password123".to_string()) {
//!     println!("Updated user: {:?}", updated_user);
//! }
//!
//! // Delete a user
//! if User::delete(&mut connection, "user_id".to_string()) {
//!     println!("User deleted");
//! }
//!
//! // User login
//! if let Some(jwt_token) = User::login(&mut connection, "john@example.com".to_string(), "password123".to_string()) {
//!     println!("User logged in. JWT token: {}", jwt_token);
//! }
//! ```
//! 
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;

use crate::services::jwt::create_jwt;

use super::super::schema::{*, self};
use super::super::schema::users::dsl::users as users_dsl;
use super::wallet::Wallet;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::users)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub wallet_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl User {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        users_dsl
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn find_by_id(conn: &mut SqliteConnection, id: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn find_by_email(conn: &mut SqliteConnection, email: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .filter(users::email.eq(email))
            .get_result::<User>(conn) {
            Some(record)
            } else {
                None
            }
    }

    pub fn create(conn: &mut SqliteConnection, name: String, email: String, wallet_id: String, password: String) -> (Option<Self>, Option<String>) {
        let new_id = Uuid::new_v4().as_hyphenated().to_string();

        if email.is_empty() || password.is_empty() || name.is_empty() || wallet_id.is_empty() {
            return (None, Some("Missing required fields".to_string()));
        }
        
        
        if Self::find_by_email(conn, email.clone()).is_some() {
            return (None, Some("Email already exists".to_string()));
        }
        
        
        if Wallet::find_by_id(conn, wallet_id.clone()).is_none() {
            return (None, Some("Wallet does not exist".to_string()));
        }
        
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();


        let new_user = Self::new_user_struct(new_id, name, email, wallet_id, hashed_password);

        diesel::insert_into(users_dsl)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");
        
        (Self::find_by_id(conn, new_user.id), None)
    }

    fn new_user_struct(id: String, name: String, email: String, wallet_id: String, password: String) -> Self {
        Self {
            id: id,
            name: name,
            email: email,
            password: password,
            wallet_id: wallet_id,
            created_at: chrono::Local::now().naive_local(),
            updated_at: chrono::Local::now().naive_local(),
        }
    }

    pub fn update(conn: &mut SqliteConnection, id: String, name: String, email: String, wallet: String, password: String) -> Option<Self> {
        if let Ok(record) = users_dsl
            .find(id)
            .get_result::<User>(conn) {
            let updated_user = Self::update_user_struct(record, name, email, wallet, password);
            diesel::update(users_dsl.find(updated_user.id.clone()))
                .set((schema::users::name.eq(updated_user.name.clone()),
                    schema::users::email.eq(updated_user.email.clone()),
                    schema::users::wallet_id.eq(updated_user.wallet_id.clone()),                    
                    schema::users::password.eq(bcrypt::hash(updated_user.password.clone(), bcrypt::DEFAULT_COST).unwrap()),
                    schema::users::updated_at.eq(chrono::Local::now().naive_local())))
                .execute(conn)
                .expect("Error updating user");
            Some(updated_user)
            } else {
                None
            }
    }

    fn update_user_struct(mut user: Self, name: String, email: String, wallet: String, password: String) -> Self {
        user.name = name;
        user.email = email;
        user.wallet_id = wallet;
        user.password = password;
        user.updated_at = chrono::Local::now().naive_local();
        user
    }

    pub fn delete(conn: &mut SqliteConnection, id: String) -> bool {
        if let Ok(_record) = users_dsl
            .find(id.clone())
            .get_result::<User>(conn) {
            diesel::delete(users_dsl.find(id))
                .execute(conn)
                .expect("Error deleting user");
            true
            } else {
                false
            }
    }

    pub fn login(conn: &mut SqliteConnection, email: String, password: String) -> Option<String> {
        if let Ok(record) = users_dsl
            .filter(users::email.eq(email))
            .get_result::<User>(conn) {
                if bcrypt::verify(password, &record.password).unwrap() {
                    Some(create_jwt(record.id).unwrap())
                } else {
                    None
                }
            } else {
                None
            }
    }

}


//! This module defines utility functions for JSON Web Token (JWT) creation and authentication in Actix Web applications.
//!
//! It includes functions to create JWT tokens with custom claims and to authenticate incoming requests based on JWT tokens.
//!
//! # Examples
//!
//! ```rust
//! use actix_web::{HttpRequest, Error};
//!
//! // ... imports ...
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Claims {
//!     id: String,
//!     exp: i64,
//! }
//!
//! // Create a JWT token with custom claims.
//! pub fn create_jwt(id: String) -> Result<String, jsonwebtoken::errors::Error> {
//!     // ... implementation details ...
//! }
//!
//! // Authenticate a request using a JWT token.
//! pub fn authenticate(req: HttpRequest) -> Result<(), Error> {
//!     // ... implementation details ...
//! }
//! ```
//!
//! # Note
//! Ensure that you have the necessary JWT library (e.g., `jsonwebtoken`) and the required secret set in your environment
//! variables (`JWT_SECRET`) for proper token creation and authentication. Additionally, use the `create_jwt` function to generate
//! JWT tokens and the `authenticate` function to verify and authenticate incoming requests.

use actix_web::error::ErrorUnauthorized;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{encode, Header, EncodingKey, Validation, Algorithm, decode, DecodingKey};
use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, Error};
use actix_web::http::header::AUTHORIZATION;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: String,
    exp: i64,
}

pub fn create_jwt(id: String) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(3))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims { id, exp: expiration.clone() };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = secret.as_bytes();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key),
    )?;
    
    Ok(token)
}

pub fn authenticate(req: HttpRequest) -> Result<(), Error> {
    let token = match req.headers().get(AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Ok(value) => value,
            Err(_) => return Err(ErrorUnauthorized("invalid token")),
        },
        None => return Err(ErrorUnauthorized("missing token")),
    };

    let validation = Validation::new(Algorithm::HS256);

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = secret.as_bytes();

    match decode::<Claims>(token, &DecodingKey::from_secret(key), &validation) {
        Ok(_token_data) => (),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => return Err(ErrorUnauthorized("token expired")),
            ErrorKind::InvalidToken => return Err(ErrorUnauthorized("invalid token")),
            _ => return Err(ErrorUnauthorized("invalid token")),
        },
    };

    Ok(())
}
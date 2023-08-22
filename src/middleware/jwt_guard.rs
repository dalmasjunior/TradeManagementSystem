//! This module defines a middleware for JWT-based authentication and authorization in Actix Web applications.
//!
//! The `JwtGuard` middleware is responsible for guarding routes with JSON Web Token (JWT) authentication. It verifies the JWT
//! token provided in the request and enforces access control to protected routes.
//!
//! The middleware consists of two main components:
//! - `JwtGuard`: A transformer that wraps the provided service with JWT authentication logic.
//! - `JwtGuardMiddleware`: The middleware that performs the actual JWT token verification and user authentication.
//!
//! These components are designed to integrate seamlessly into the Actix Web middleware chain, providing a secure way to protect routes.
//!
//! # Examples
//!
//! ```rust
//! use actix_service::{Service, Transform};
//! use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
//!
//! // ... imports ...
//!
//! // Define a middleware struct for JWT authentication.
//! pub struct JwtGuard;
//!
//! // ... implementation details ...
//!
//! // Define a middleware struct that wraps the JwtGuard middleware.
//! pub struct JwtGuardMiddleware<S> {
//!     // ... fields ...
//! }
//!
//! // ... implementation details ...
//!
//! // Implement the `Transform` trait for JwtGuard.
//! impl<S, B> Transform<S, ServiceRequest> for JwtGuard
//! where
//!     // ... trait bounds ...
//! {
//!     // ... associated types and methods ...
//! }
//!
//! // Implement the `Service` trait for JwtGuardMiddleware.
//! impl<S, B> Service<ServiceRequest> for JwtGuardMiddleware<S>
//! where
//!     // ... trait bounds ...
//! {
//!     // ... associated types and methods ...
//! }
//!
//! // ... other related functions ...
//! ```
//!
//! # Note
//! Ensure that you have the necessary JWT-related functions and structures (e.g., `authenticate`) available in your project for proper
//! JWT verification and user authentication. Additionally, ensure that this middleware is properly integrated into your Actix Web application's
//! middleware chain to secure the desired routes.

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use futures_util::future::LocalBoxFuture;
use crate::services::jwt::authenticate;

pub struct JwtGuard;

impl<S, B> Transform<S, ServiceRequest> for JwtGuard
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtGuardMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtGuardMiddleware { service })
    }
}

pub struct JwtGuardMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtGuardMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;

            authenticate(res.request().clone())?;

            Ok(res)
        })
    }
}
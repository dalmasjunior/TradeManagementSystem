//! This module defines the Diesel schema for the database tables used in the application.
//!
//! It includes the Diesel table definitions for the `trades`, `users`, and `wallet` tables.
//! These tables represent different aspects of the application's data, including trade activities,
//! user information, and wallet details.
//!
//! Additionally, this module establishes relationships between tables using the `joinable!` macros,
//! enabling convenient queries involving multiple tables.
//!
//! # Note
//! This code is generated automatically by the Diesel CLI and reflects the structure of your database.
//! It provides a convenient way to interact with the database using Diesel's query building and
//! schema representation.


// @generated automatically by Diesel CLI.

diesel::table! {
    trades (id) {
        id -> Text,
        user_id -> Text,
        wallet_id -> Text,
        amount -> Float,
        chain -> Text,
        trade_type -> Text,
        asset -> Text,
        before_price -> Float,
        execution_price -> Float,
        final_price -> Float,
        traded_amount -> Float,
        execution_fee -> Float,
        transaction_fee -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
        wallet_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    wallet (id) {
        id -> Text,
        hash -> Text,
        balance -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(trades -> users (user_id));
diesel::joinable!(trades -> wallet (wallet_id));
diesel::joinable!(users -> wallet (wallet_id));

diesel::allow_tables_to_appear_in_same_query!(
    trades,
    users,
    wallet,
);

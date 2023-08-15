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

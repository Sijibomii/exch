
// token
diesel::table! {
    tokens(id) {
        id -> Uuid,
        ticker -> VarChar,
        is_trading -> Bool,
        supply -> BigInt,
        user_id -> Uuid,
        ticker_id -> BigInt,
    }
}
  
// customer account
diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        is_verified -> Bool,
        verification_token -> Uuid,
        verification_token_expires_at -> Timestamptz,
        reset_token -> Nullable<Uuid>,
        reset_token_expires_at -> Nullable<Timestamptz>,
        last_login_time -> Nullable<Timestamptz>,
        last_login_ip -> Nullable<VarChar>,
        trading_client_id -> BigInt,
        last_order_id -> BigInt,
        last_seq_num -> BigInt,
    }
} 



diesel::joinable!(tokens -> users (user_id));

// wallet 
diesel::table! {
    wallet(id) {
        id -> Uuid,
        user_id -> Uuid,
        balance -> BigInt,
        last_activity_time -> Nullable<Timestamptz>,
    }
}

// // token onwership map no of tokens owned to trading wallet
diesel::table! {
    token_ownership(id) {
        id -> Uuid,
        token_id -> Uuid,
        balance -> BigInt,
        wallet_id -> Uuid,
    }
}

diesel::table! {
    client(id) {
        id -> Uuid,
        next_id -> BigInt,
        next_ticker_id -> BigInt
    }
}

// last client id given


allow_tables_to_appear_in_same_query!(
    users,
    tokens,
    wallet,
    token_ownership
);
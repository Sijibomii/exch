use diesel::sql_types::*;
// customer account
table! {
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
    }
} 


// token
table! {
    token(id) {
        id -> Uuid,
        ticker -> VarChar,
        is_trading -> Bool,
        supply -> BigInt,
        owner_id -> Uuid,
    }
}

// wallet 
table! {
    wallet(id) {
        id -> Uuid,
        user_id -> Uuid,
        balance -> Numeric,
        last_activity_time -> Nullable<Timestamptz>,
    }
}

// token onwership map no of tokens owned to trading wallet
table! {
    token_ownership(id) {
        id -> Uuid,
        token_id -> Uuid,
        balance -> Numeric,
        wallet_id -> Uuid,
    }
}


allow_tables_to_appear_in_same_query!(
    
);
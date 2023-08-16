
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

// admin accounts
table! {
    admin (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Varchar,
        salt -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        reset_token -> Nullable<Uuid>,
        reset_token_expires_at -> Nullable<Timestamptz>,
        last_login_time -> Nullable<Timestamptz>,
        last_login_ip -> Nullable<VarChar>,
        role_name -> VarChar,
        role_id -> Uuid,
        department_id -> Uuid,
    } 
}

// admin activity log
table! {
    admin_activity (id) {
        id -> Uuid,
        admin_id -> Uuid,
        operation -> VarChar,
        access_mode -> VarChar, 
        access_time -> Timestamptz,
        admin_activity_type -> Varchar,
    } 
}

// currency 
table! {
    currency(id) {
        id -> Uuid,
        name -> VarChar
    }
}

// startup 
table! {
    startup(id) {
        id -> Uuid,
        name -> VarChar,
        description -> VarChar
    }
}

// token
table! {
    token(id) {
        id -> Uuid,
        ticker -> VarChar,
        is_trading -> Bool,
        supply -> Numeric
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

// trading wallet
table! {
    trading_wallet(id) {
        id -> Uuid,
        user_id -> Uuid,
        balance -> Numeric,
        last_activity_time -> Nullable<Timestamptz>,
    }
}

// token onwership map no of tokens owned to trading wallet

allow_tables_to_appear_in_same_query!(
    
);
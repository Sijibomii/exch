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
        
    } 
}

allow_tables_to_appear_in_same_query!(
    
);
// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        img -> Nullable<Varchar>,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    booking (id) {
        id -> Int4,
        user_id -> Int4,
        provision_d_id -> Nullable<Int4>,
        provision_h_id -> Nullable<Int4>,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        st_date -> Nullable<Date>,
        en_date -> Nullable<Date>,
        st_hour -> Nullable<Timestamp>,
        en_hour -> Nullable<Timestamp>,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    provision_d (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        st_date -> Nullable<Date>,
        en_date -> Nullable<Date>,
        s_dates -> Nullable<Array<Nullable<Date>>>,
        e_dates -> Nullable<Array<Nullable<Date>>>,
        dates -> Nullable<Array<Nullable<Date>>>,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    provision_h (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        st_hour -> Nullable<Timestamp>,
        en_hour -> Nullable<Timestamp>,
        s_hours -> Nullable<Array<Nullable<Timestamp>>>,
        e_hours -> Nullable<Array<Nullable<Timestamp>>>,
        hours -> Nullable<Array<Nullable<Timestamp>>>,
        completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (session_token) {
        session_token -> Bytea,
        id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        username -> Text,
        password -> Text,
        #[max_length = 255]
        img -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(article -> users (user_id));
diesel::joinable!(booking -> provision_d (provision_d_id));
diesel::joinable!(booking -> provision_h (provision_h_id));
diesel::joinable!(booking -> users (user_id));
diesel::joinable!(provision_d -> users (user_id));
diesel::joinable!(provision_h -> users (user_id));
diesel::joinable!(sessions -> users (id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    booking,
    provision_d,
    provision_h,
    sessions,
    users,
);

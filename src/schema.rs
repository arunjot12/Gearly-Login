// @generated automatically by Diesel CLI.

diesel::table! {
    signup_shopkeepers (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 10]
        phone_number -> Nullable<Char>,
        #[max_length = 255]
        shop_name -> Nullable<Varchar>,
        #[max_length = 255]
        shop_address -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 10]
        phone_number -> Char,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(signup_shopkeepers, users,);

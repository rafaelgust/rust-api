// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 256]
        url_name -> Varchar,
        #[max_length = 512]
        description -> Varchar,
        created_at -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 256]
        url_name -> Varchar,
        #[max_length = 256]
        description -> Varchar,
        created_at -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    categories_related (parent_id, child_id) {
        parent_id -> Int4,
        child_id -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        #[max_length = 256]
        text -> Varchar,
        created_at -> Timestamp,
        product_id -> Uuid,
        user_id -> Uuid,
        published -> Bool,
    }
}

diesel::table! {
    feedback_types (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
    }
}

diesel::table! {
    feedbacks (id) {
        id -> Int4,
        product_id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    grades (id) {
        id -> Int4,
        feedback_id -> Int4,
        type_id -> Int4,
        value -> Int4,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 512]
        url_name -> Varchar,
        description -> Text,
        #[max_length = 256]
        image -> Nullable<Varchar>,
        brand_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
        created_at -> Timestamp,
        published -> Bool,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 16]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 512]
        password -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        role_id -> Int4,
        created_at -> Timestamp,
        published -> Bool,
    }
}

diesel::joinable!(comments -> products (product_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(feedbacks -> products (product_id));
diesel::joinable!(feedbacks -> users (user_id));
diesel::joinable!(grades -> feedback_types (type_id));
diesel::joinable!(products -> brands (brand_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(users -> roles (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    categories,
    categories_related,
    comments,
    feedback_types,
    feedbacks,
    grades,
    products,
    roles,
    users,
);

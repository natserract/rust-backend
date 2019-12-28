table! {
    rust_db.users (id) {
        id -> Integer,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    entries (id) {
        id -> Integer,
        sitename -> Text,
        siteurl -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
    }
}

diesel::table! {
    secrets (id) {
        id -> Integer,
        masterkey_hash -> Text,
        device_secret -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entries, secrets,);

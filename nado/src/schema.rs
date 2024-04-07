// @generated automatically by Diesel CLI.

diesel::table! {
    codepromo (Id) {
        Id -> Nullable<Integer>,
        Description -> Text,
        IdSubscibe -> Integer,
    }
}

diesel::table! {
    content (Id) {
        Id -> Nullable<Integer>,
        Name -> Text,
        Description -> Text,
        DescriptionDetails -> Text,
    }
}

diesel::table! {
    contentForPreferences (Id) {
        Id -> Nullable<Integer>,
        IdContent -> Integer,
        IdUsers -> Integer,
    }
}

diesel::table! {
    file (Id) {
        Id -> Nullable<Integer>,
        IdContent -> Integer,
        Path -> Text,
    }
}

diesel::table! {
    history (Id) {
        Id -> Nullable<Integer>,
        IdUser -> Integer,
        IdContent -> Integer,
        EndSee -> Integer,
    }
}

diesel::table! {
    role (Id) {
        Id -> Nullable<Integer>,
        Name -> Text,
    }
}

diesel::table! {
    subscribe (Id) {
        Id -> Nullable<Integer>,
        Name -> Text,
        Count_month -> Integer,
        Title -> Text,
        Description -> Text,
        Discount -> Nullable<Integer>,
    }
}

diesel::table! {
    subscribeAndUser (Id) {
        Id -> Nullable<Integer>,
        IdSubscribe -> Integer,
        IdUsers -> Integer,
        DataEnd -> Text,
    }
}

diesel::table! {
    users (Id) {
        Id -> Nullable<Integer>,
        Name -> Text,
        Surname -> Text,
        Password -> Text,
        Email -> Text,
        Role -> Integer,
    }
}

diesel::table! {
    workers (Id) {
        Id -> Nullable<Integer>,
        Name -> Text,
        Surname -> Text,
        IdContent -> Integer,
        Role -> Integer,
    }
}

diesel::joinable!(codepromo -> subscribe (IdSubscibe));
diesel::joinable!(contentForPreferences -> content (IdContent));
diesel::joinable!(contentForPreferences -> users (IdUsers));
diesel::joinable!(file -> content (IdContent));
diesel::joinable!(history -> content (IdContent));
diesel::joinable!(history -> users (IdUser));
diesel::joinable!(subscribeAndUser -> subscribe (IdSubscribe));
diesel::joinable!(subscribeAndUser -> users (IdUsers));
diesel::joinable!(users -> role (Role));
diesel::joinable!(workers -> content (IdContent));
diesel::joinable!(workers -> role (Role));

diesel::allow_tables_to_appear_in_same_query!(
    codepromo,
    content,
    contentForPreferences,
    file,
    history,
    role,
    subscribe,
    subscribeAndUser,
    users,
    workers,
);

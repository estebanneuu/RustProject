// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    commands (id) {
        id -> Int4,
        command -> Varchar,
        agent_id -> Int4,
        created_at -> Timestamp,
        done -> Bool,
    }
}

diesel::table! {
    results (id) {
        id -> Int4,
        command_id -> Int4,
        agent_id -> Int4,
        result_content -> Varchar,
        done_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    commands,
    results,
);

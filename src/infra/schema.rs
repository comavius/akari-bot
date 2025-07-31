use diesel::prelude::*;

table! {
    scores (user_id) {
        user_id -> Varchar,
        precision_type -> Varchar,
        precision_percentage -> Nullable<Int8>,
        time_sec -> Int8,
    }
}

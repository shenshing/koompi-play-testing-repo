table! {
    facebook_log_users (_id) {
        _id -> Int4,
        facebook_user_id -> Varchar,
        facebook_user_name -> Varchar,
        facebook_user_gender -> Varchar,
        facebook_user_email -> Varchar,
        facebook_user_created -> Timestamp,
        facebook_user_profile -> Varchar,
        facebook_user_role -> Nullable<Varchar>,
        facebook_user_phone -> Nullable<Varchar>,
    }
}

table! {
    players (id) {
        id -> Int4,
        playername -> Varchar,
        score -> Int4,
        playdate -> Timestamp,
        email -> Varchar,
    }
}

table! {
    questions (question_id) {
        question_id -> Int4,
        question -> Varchar,
        correct_answer -> Varchar,
        incorrect_answer1 -> Varchar,
        incorrect_answer2 -> Varchar,
        incorrect_answer3 -> Varchar,
        incorrect_answer4 -> Varchar,
        incorrect_answer5 -> Varchar,
    }
}

table! {
    users (user_id) {
        user_id -> Int4,
        user_name -> Varchar,
        user_gender -> Varchar,
        user_email -> Varchar,
        user_password -> Varchar,
        create_date -> Timestamp,
        user_profile -> Nullable<Varchar>,
        user_role -> Nullable<Varchar>,
        phone_number -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    facebook_log_users,
    players,
    questions,
    users,
);

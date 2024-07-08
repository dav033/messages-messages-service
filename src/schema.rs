diesel::table! {
    messages (id) {
        id -> Integer,
        body -> Text,
        #[max_length = 50]
        typeM -> Varchar,
        #[max_length = 50]
        datetime -> Varchar,
        #[max_length = 100]
        sender -> Varchar,
        #[max_length = 100]
        sender_name -> Varchar,
        #[max_length = 100]
        receiver -> Varchar,
        #[max_length = 2000]
        readed -> Varchar,
    }
}

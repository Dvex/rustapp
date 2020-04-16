table! {
    currency (id) {
        id -> Int4,
        iso_name -> Varchar,
        description -> Varchar,
    }
}

table! {
    product (id) {
        id -> Int4,
        description -> Varchar,
    }
}

table! {
    sale (id) {
        id_product -> Int4,
        id_currency -> Int4,
        amount -> Numeric,
        date_sale -> Timestamp,
        id -> Int4,
    }
}

joinable!(sale -> currency (id_currency));
joinable!(sale -> product (id_product));

allow_tables_to_appear_in_same_query!(
    currency,
    product,
    sale,
);

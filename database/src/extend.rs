use diesel::{sql_function, sql_types::Text};

sql_function! { fn lower(s: Text) -> Text; }

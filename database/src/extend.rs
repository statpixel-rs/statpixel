use diesel::{
	sql_function,
	sql_types::{Bytea, Text},
};

sql_function! { fn lower(s: Text) -> Text; }
sql_function! { fn length(s: Text) -> Int2; }
sql_function! { fn md5(s: Bytea) -> Text; }

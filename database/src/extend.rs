use diesel::{
	sql_function,
	sql_types::{Integer, Text},
};

sql_function! { fn lower(s: Text) -> Text; }
sql_function! {
	#[sql_name = "mod"]
	fn modulo(a: Integer, b: Integer) -> Integer;
}

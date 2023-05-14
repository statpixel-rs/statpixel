// @generated automatically by Diesel CLI.

diesel::table! {
	autocomplete (uuid) {
		uuid -> Uuid,
		name -> Varchar,
	}
}

diesel::table! {
	snapshot (id) {
		uuid -> Uuid,
		created_at -> Timestamptz,
		updated_at -> Timestamptz,
		id -> Int4,
		data -> Bytea,
	}
}

diesel::table! {
	users (id) {
		id -> Int8,
		text -> Bool,
		uuid -> Nullable<Uuid>,
	}
}

diesel::allow_tables_to_appear_in_same_query!(autocomplete, snapshot, users,);

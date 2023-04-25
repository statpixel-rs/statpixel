// @generated automatically by Diesel CLI.

diesel::table! {
	history (uuid, game_id, game_mode) {
			uuid -> Uuid,
			game_id -> Int2,
			game_mode -> Int2,
			update_frequency -> Nullable<Int2>,
			created_at -> Timestamptz,
			updated_at -> Timestamptz,
	}
}

diesel::table! {
	users (id) {
			id -> Int8,
			text -> Bool,
			uuid -> Nullable<Uuid>,
	}
}

diesel::allow_tables_to_appear_in_same_query!(history, users,);

// @generated automatically by Diesel CLI.

diesel::table! {
	autocomplete (uuid) {
		uuid -> Uuid,
		name -> Varchar,
		searches -> Int4,
	}
}

diesel::table! {
	guild_autocomplete (uuid) {
		uuid -> Uuid,
		name -> Varchar,
		xp -> Int4,
		searches -> Int4,
	}
}

diesel::table! {
	guild_schedule (uuid) {
		uuid -> Uuid,
		snapshots -> Int4,
		hash -> Int8,
		prev_hash -> Nullable<Int8>,
		update_at -> Timestamptz,
		created_at -> Timestamptz,
	}
}

diesel::table! {
	guild_snapshot (id) {
		id -> Int4,
		uuid -> Uuid,
		hash -> Int8,
		did_update -> Bool,
		data -> Bytea,
		created_at -> Timestamptz,
		updated_at -> Timestamptz,
		days_since_epoch -> Int4,
	}
}

diesel::table! {
	schedule (uuid) {
		uuid -> Uuid,
		update_at -> Timestamptz,
		created_at -> Timestamptz,
		snapshots -> Int4,
		hash -> Int8,
		prev_hash -> Nullable<Int8>,
		weekly_schedule -> Int4,
	}
}

diesel::table! {
	snapshot (id) {
		uuid -> Uuid,
		created_at -> Timestamptz,
		updated_at -> Timestamptz,
		id -> Int4,
		data -> Bytea,
		did_update -> Bool,
		hash -> Int8,
	}
}

diesel::table! {
	user (id) {
		id -> Int8,
		text -> Bool,
		uuid -> Nullable<Uuid>,
		updated_at -> Timestamptz,
		created_at -> Timestamptz,
	}
}

diesel::allow_tables_to_appear_in_same_query!(
	autocomplete,
	guild_autocomplete,
	guild_schedule,
	guild_snapshot,
	schedule,
	snapshot,
	user,
);

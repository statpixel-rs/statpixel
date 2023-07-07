// @generated automatically by Diesel CLI.

diesel::table! {
	autocomplete (uuid) {
		uuid -> Uuid,
		#[max_length = 16]
		name -> Varchar,
		searches -> Int4,
	}
}

diesel::table! {
	bazaar (id) {
		id -> Int4,
		item_id -> Int2,
		sell_price -> Float8,
		sell_volume -> Int4,
		sell_orders -> Int4,
		buy_price -> Float8,
		buy_volume -> Int4,
		buy_orders -> Int4,
		created_at -> Timestamptz,
	}
}

diesel::table! {
	bazaar_item (id) {
		id -> Int2,
		name -> Text,
	}
}

diesel::table! {
	guild_autocomplete (uuid) {
		uuid -> Uuid,
		#[max_length = 32]
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
		version -> Int2,
	}
}

diesel::table! {
	metric (id) {
		id -> Int4,
		discord_id -> Int8,
		kind -> Int2,
		created_at -> Timestamptz,
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
		version -> Int2,
	}
}

diesel::table! {
	usage (user_id, command_name) {
		user_id -> Int8,
		command_name -> Text,
		count -> Int4,
	}
}

diesel::table! {
	user (id) {
		id -> Int8,
		uuid -> Nullable<Uuid>,
		updated_at -> Timestamptz,
		created_at -> Timestamptz,
		display -> Int2,
		#[max_length = 4]
		suffix -> Nullable<Varchar>,
		colour -> Nullable<Int4>,
		votes -> Int2,
	}
}

-- Your SQL goes here

CREATE TABLE bazaar_items (
	id SMALLSERIAL PRIMARY KEY NOT NULL,
	name TEXT NOT NULL
);

CREATE TABLE bazaar (
	id SERIAL PRIMARY KEY NOT NULL,
	item_id SMALLINT NOT NULL,
	sell_price DOUBLE PRECISION NOT NULL,
	sell_volume INT NOT NULL,
	sell_orders INT NOT NULL,
	buy_price DOUBLE PRECISION NOT NULL,
	buy_volume INT NOT NULL,
	buy_orders INT NOT NULL,

	FOREIGN KEY (item_id) REFERENCES bazaar_items(id)
);

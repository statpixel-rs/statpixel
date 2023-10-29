use api::canvas;
use database::schema;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use poise::serenity_prelude as serenity;
use translate::{context, tr, tr_fmt};

use crate::{util::error_embed, Context, Error};
const COLOURS: &[[&str; 2]] = &[
	["Reset", "reset"],
	["Transparent", "transparent"],
	["Alice Blue", "alice blue"],
	["Amaranth", "amaranth"],
	["Amber", "amber"],
	["Amethyst", "amethyst"],
	["Apple Green", "apple green"],
	["Apple Red", "apple red"],
	["Apricot", "apricot"],
	["Aqua", "aqua"],
	["Aquamarine", "aquamarine"],
	["Azure", "azure"],
	["Baby Blue", "baby blue"],
	["Beige", "beige"],
	["Black", "black"],
	["Blue", "blue"],
	["Blue Green", "blue green"],
	["Blue Violet", "blue violet"],
	["Blush", "blush"],
	["Brick Red", "brick red"],
	["Bronze", "bronze"],
	["Burgundy", "burgundy"],
	["Byzantium", "byzantium"],
	["Carmine", "carmine"],
	["Cerise", "cerise"],
	["Cerulean", "cerulean"],
	["Champagne", "champagne"],
	["Chartreuse Green", "chartreuse green"],
	["Chocolate", "chocolate"],
	["Cobalt Blue", "cobalt blue"],
	["Coffee", "coffee"],
	["Copper", "copper"],
	["Coral", "coral"],
	["Crimson", "crimson"],
	["Desert Sand", "desert sand"],
	["Electric Blue", "electric blue"],
	["Emerald", "emerald"],
	["Erin", "erin"],
	["Gold", "gold"],
	["Gray", "gray"],
	["Green", "green"],
	["Harlequin", "harlequin"],
	["Indigo", "indigo"],
	["Ivory", "ivory"],
	["Jade", "jade"],
	["Jungle Green", "jungle green"],
	["Lavender", "lavender"],
	["Lemon", "lemon"],
	["Lilac", "lilac"],
	["Lime", "lime"],
	["Magenta", "magenta"],
	["Magenta Rose", "magenta rose"],
	["Maroon", "maroon"],
	["Mauve", "mauve"],
	["Navy Blue", "navy blue"],
	["Ochre", "ochre"],
	["Olive", "olive"],
	["Orange", "orange"],
	["Orange Red", "orange red"],
	["Orchid", "orchid"],
	["Peach", "peach"],
	["Pear", "pear"],
	["Periwinkle", "periwinkle"],
	["Persian Blue", "persian blue"],
	["Pink", "pink"],
	["Plum", "plum"],
	["Prussian Blue", "prussian blue"],
	["Puce", "puce"],
	["Purple", "purple"],
	["Raspberry", "raspberry"],
	["Red", "red"],
	["Red Violet", "red violet"],
	["Rose", "rose"],
	["Ruby", "ruby"],
	["Salmon", "salmon"],
	["Sangria", "sangria"],
	["Sapphire", "sapphire"],
	["Scarlet", "scarlet"],
	["Silver", "silver"],
	["Slate Gray", "slate gray"],
	["Spring Bud", "spring bud"],
	["Spring Green", "spring green"],
	["Tan", "tan"],
	["Taupe", "taupe"],
	["Teal", "teal"],
	["Turquoise", "turquoise"],
	["Ultramarine", "ultramarine"],
	["Violet", "violet"],
	["Viridian", "viridian"],
	["White", "white"],
	["Yellow", "yellow"],
];

#[allow(clippy::unused_async)]
async fn autocomplete_colour(
	_ctx: Context<'_>,
	partial: &str,
) -> Box<dyn Iterator<Item = String> + Send> {
	let lower = partial.to_ascii_lowercase();

	Box::new(
		COLOURS
			.iter()
			.filter_map(|c| {
				if c[1].starts_with(&lower) {
					Some(c[0].to_string())
				} else {
					None
				}
			})
			.take(10)
			.collect::<Vec<_>>()
			.into_iter(),
	)
}

/// Changes the background of images.
#[poise::command(
	on_error = "crate::util::error_handler",
	slash_command,
	required_bot_permissions = "EMBED_LINKS"
)]
pub async fn background(
	ctx: Context<'_>,
	#[autocomplete = "autocomplete_colour"] colour: Option<String>,
) -> Result<(), Error> {
	let Some(colour) = colour.map_or(Some(canvas::Canvas::BACKGROUND_U32), |mut c| {
		c.make_ascii_lowercase();
		colour_from_str(&c)
	}) else {
		ctx.send(error_embed(
			tr(&ctx, "error-invalid-colour"),
			tr(&ctx, "error-invalid-colour-description"),
		))
		.await?;

		return Ok(());
	};

	let u = ctx.author();
	let ctx = &context::Context::from_poise(&ctx);

	diesel::insert_into(schema::user::table)
		.values((
			schema::user::id.eq(u.id.get() as i64),
			schema::user::colour.eq(colour as i32),
		))
		.on_conflict(schema::user::id)
		.do_update()
		.set((
			schema::user::colour.eq(colour as i32),
			schema::user::updated_at.eq(chrono::Utc::now()),
		))
		.execute(&mut ctx.connection().await?)
		.await?;

	ctx.send(
		poise::CreateReply::new().embed(
			serenity::CreateEmbed::new()
				.title(tr(&ctx, "colour-changed"))
				.description(
					tr_fmt!(&ctx, "colour-changed-description", colour: format!("#{:0>8x}", colour)),
				)
				.colour(colour & 0x00_ffffff),
		),
	)
	.await?;

	Ok(())
}

#[allow(clippy::too_many_lines)]
fn colour_from_str(colour: &str) -> Option<u32> {
	Some(match colour {
		"reset" | "normal" | "regular" | "statpixel" => canvas::Canvas::BACKGROUND_U32,
		"transparent" | "none" => 0x00_000000,
		"black" => 0xff_000000,
		"gold" => 0xff_ffaa00,
		"gray" => 0xff_aaaaaa,
		"blue" => 0xff_5555ff,
		"green" => 0xff_55ff55,
		"aqua" => 0xff_55ffff,
		"red" => 0xff_ff5555,
		"yellow" => 0xff_ffff55,
		"white" => 0xff_ffffff,
		"alice blue" => 0xff_f0f8ff,
		"amaranth" => 0xff_e52b50,
		"amber" => 0xff_ffbf00,
		"amethyst" => 0xff_9966cc,
		"apple green" => 0xff_8db600,
		"apple red" => 0xff_ed2939,
		"apricot" => 0xff_fbceb1,
		"aquamarine" => 0xff_7fffd4,
		"azure" => 0xff_007fff,
		"baby blue" => 0xff_89cff0,
		"beige" => 0xff_f5f5dc,
		"bric kred" => 0xff_cb4154,
		"blue green" => 0xff_0d98ba,
		"blue violet" => 0xff_8a2be2,
		"blush" => 0xff_de5d83,
		"bronze" => 0xff_cd7f32,
		"burgundy" => 0xff_800020,
		"byzantium" => 0xff_702963,
		"carmine" => 0xff_960018,
		"cerise" => 0xff_de3163,
		"cerulean" => 0xff_007ba7,
		"champagne" => 0xff_f7e7ce,
		"chartreuse green" => 0xff_7fff00,
		"chocolate" => 0xff_d2691e,
		"cobalt blue" => 0xff_0047ab,
		"coffee" => 0xff_6f4e37,
		"copper" => 0xff_b87333,
		"coral" => 0xff_ff7f50,
		"crimson" => 0xff_dc143c,
		"desert sand" => 0xff_edc9af,
		"electric blue" => 0xff_7df9ff,
		"emerald" => 0xff_50c878,
		"erin" => 0xff_00ff3f,
		"harlequin" => 0xff_3fff00,
		"indigo" => 0xff_4b0082,
		"ivory" => 0xff_fffff0,
		"jade" => 0xff_00a86b,
		"jungle green" => 0xff_29ab87,
		"lavender" => 0xff_b57edc,
		"lemon" => 0xff_fff700,
		"lilac" => 0xff_c8a2c8,
		"lime" => 0xff_32cd32,
		"magenta" => 0xff_ff00ff,
		"magenta rose" => 0xff_ff00af,
		"maroon" => 0xff_800000,
		"mauve" => 0xff_e0b0ff,
		"navy blue" => 0xff_000080,
		"ochre" => 0xff_cc7722,
		"olive" => 0xff_808000,
		"orange" => 0xff_ffa500,
		"orange red" => 0xff_ff4500,
		"orchid" => 0xff_da70d6,
		"peach" => 0xff_ffcba4,
		"pear" => 0xff_d1e231,
		"periwinkle" => 0xff_ccccff,
		"persian blue" => 0xff_1c39bb,
		"pink" => 0xff_ffc0cb,
		"plum" => 0xff_dda0dd,
		"prussian blue" => 0xff_003153,
		"puce" => 0xff_cc8899,
		"purple" => 0xff_800080,
		"raspberry" => 0xff_e30b5d,
		"red violet" => 0xff_c71585,
		"rose" => 0xff_ff007f,
		"ruby" => 0xff_e0115f,
		"salmon" => 0xff_fa8072,
		"sangria" => 0xff_92000a,
		"sapphire" => 0xff_0f52ba,
		"scarlet" => 0xff_ff2400,
		"silver" => 0xff_c0c0c0,
		"slateg ray" => 0xff_708090,
		"spring bud" => 0xff_a7fc00,
		"spring green" => 0xff_00ff7f,
		"tan" => 0xff_d2b48c,
		"taupe" => 0xff_483c32,
		"teal" => 0xff_008080,
		"turquoise" => 0xff_40e0d0,
		"ultramarine" => 0xff_120a8f,
		"violet" => 0xff_7f00ff,
		"viridian" => 0xff_40826d,
		hex if colour.starts_with('#') && colour.len() == 4 => {
			let colour = u32::from_str_radix(&hex[1..4], 16).ok()?;

			0xff_000000
				| ((colour & 0xf00) << 12)
				| ((colour & 0xf00) << 8)
				| ((colour & 0xf0) << 8)
				| ((colour & 0xf0) << 4)
				| ((colour & 0xf) << 4)
				| (colour & 0xf)
		}
		hex if colour.starts_with('#') && colour.len() == 5 => {
			let colour = u32::from_str_radix(&hex[1..5], 16).ok()?;

			((colour & 0xf000) << 16)
				| ((colour & 0xf000) << 12)
				| ((colour & 0xf00) << 12)
				| ((colour & 0xf00) << 8)
				| ((colour & 0xf0) << 8)
				| ((colour & 0xf0) << 4)
				| ((colour & 0xf) << 4)
				| (colour & 0xf)
		}
		hex if colour.starts_with('#') && colour.len() == 7 => {
			0xff_000000 | u32::from_str_radix(&hex[1..7], 16).ok()?
		}
		hex if colour.starts_with('#') && colour.len() == 9 => {
			u32::from_str_radix(&hex[1..9], 16).ok()?
		}
		_ => return None,
	})
}

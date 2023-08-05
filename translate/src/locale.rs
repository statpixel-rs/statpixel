//! Portions of this implementation are taken from the Poise + Fluent example.
//! https://github.com/serenity-rs/poise/blob/current/examples/fluent_localization/translation.rs

use core::panic;
use std::{borrow::Cow, fmt::Debug, str::FromStr};

use crate::{prelude::GetLocale, Data, Error};
use once_cell::sync::Lazy;
use tracing::warn;

type Bundle = fluent::bundle::FluentBundle<
	fluent::FluentResource,
	intl_memoizer::concurrent::IntlLangMemoizer,
>;

pub struct English<'c>(&'c Data);

static NAME_REGEX: Lazy<regex::Regex> =
	Lazy::new(|| regex::Regex::new(r"^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$").unwrap());

impl GetLocale for English<'_> {
	fn data(&self) -> &crate::Data {
		self.0
	}

	fn locale(&self) -> Option<&crate::context::Locale> {
		Some(&crate::context::Locale::en_US)
	}
}

#[macro_export]
macro_rules! tr_fmt {
	($ctx: expr, $id: expr $(, $argname:ident: $argvalue:expr )* $(,)?) => {{
		let mut args = $crate::fluent::FluentArgs::new();
		$( args.set(stringify!($argname), $argvalue); )*

		let result = $crate::get($ctx, $id, None, Some(&args));
		drop(args);

		result
	}};
}

#[macro_export]
macro_rules! tr {
	($ctx: expr, $id: expr) => {{
		$crate::get_str($ctx, $id)
	}};
}

pub struct Locale {
	main: Bundle,
	other: std::collections::HashMap<super::context::Locale, Bundle>,
}

impl Debug for Locale {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Locale").finish()
	}
}

/// Given a language file and message identifier, returns the translation
pub fn format(
	bundle: &Bundle,
	id: &str,
	attr: Option<&str>,
	args: Option<&fluent::FluentArgs<'_>>,
) -> Option<String> {
	let message = bundle.get_message(id)?;
	let pattern = match attr {
		Some(attribute) => message.get_attribute(attribute)?.value(),
		None => message.value()?,
	};

	Some(
		bundle
			.format_pattern(pattern, args, &mut vec![])
			.into_owned(),
	)
}

pub fn get_str<'t, 'c: 't, 'i: 't>(ctx: &'c impl GetLocale, id: &'i str) -> Cow<'t, str> {
	let locale = &ctx.data().locale;

	ctx.locale()
		.and_then(|l| get_locale_str(locale.other.get(l)?, id))
		.or_else(|| get_locale_str(&locale.main, id))
		.unwrap_or_else(|| {
			warn!("unknown fluent message identifier `{}`", id);
			Cow::Borrowed(id)
		})
}

fn get_locale_str<'t, 'i: 't>(bundle: &'t Bundle, id: &'i str) -> Option<Cow<'t, str>> {
	let message = bundle.get_message(id)?;
	let pattern = message.value()?;

	Some(bundle.format_pattern(pattern, None, &mut vec![]))
}

/// Retrieves the appropriate language file depending on user locale and calls [`format`]
pub fn get<'i>(
	ctx: &impl GetLocale,
	id: &'i str,
	attr: Option<&str>,
	args: Option<&fluent::FluentArgs<'_>>,
) -> Cow<'i, str> {
	let locale = &ctx.data().locale;

	ctx.locale()
		.and_then(|l| format(locale.other.get(l)?, id, attr, args).map(Cow::Owned))
		.or_else(|| format(&locale.main, id, attr, args).map(Cow::Owned))
		.unwrap_or_else(|| {
			warn!("unknown fluent message identifier `{}`", id);
			Cow::Borrowed(id)
		})
}

/// Parses the `locale/` folder into a set of language files (Bundle)
pub fn read_ftl() -> Result<Locale, Box<dyn std::error::Error>> {
	fn read_single_ftl(
		path: &std::path::Path,
	) -> Result<(crate::context::Locale, Bundle), Box<dyn std::error::Error>> {
		// Extract locale from filename
		let locale = path.file_stem().ok_or("invalid .ftl filename")?;
		let locale = locale.to_str().ok_or("invalid filename UTF-8")?;

		// Load .ftl resource
		let file_contents =
			std::fs::read_to_string(path).map_err(|e| format!("could not read file: {e:?}"))?;

		let resource = fluent::FluentResource::try_new(file_contents)
			.map_err(|(_, e)| format!("failed to parse {path:?}: {e:?}"))?;

		// Associate .ftl resource with locale and bundle it
		let mut bundle = Bundle::new_concurrent(vec![locale
			.parse()
			.map_err(|e| format!("invalid locale `{locale}`: {e}"))?]);

		bundle.set_use_isolating(false);
		bundle
			// disable the Directional Isolate formatting characters since they
			// can mess up with copying text from Discord, like identifiers, correctly
			.add_resource(resource)
			.map_err(|e| format!("failed to add resource to bundle: {e:?}"))?;

		Ok((crate::context::Locale::from_str(locale).unwrap(), bundle))
	}

	Ok(Locale {
		main: read_single_ftl("locale/en-US.ftl".as_ref())?.1,
		other: std::fs::read_dir("locale")?
			.map(|file| read_single_ftl(&file?.path()))
			.collect::<Result<_, _>>()
			.map_err(|e| format!("could not read directory {e:?}"))?,
	})
}

impl Locale {
	/// Given a set of language files, fills in command strings and their localizations accordingly
	pub fn apply_translations(
		&self,
		commands: &mut [poise::Command<Data, Error>],
		parent: Option<&str>,
	) {
		for command in commands {
			let command_name = match parent {
				Some(parent) => format!("{}-{}", parent, command.name),
				None => command.name.clone(),
			};

			if !command.subcommands.is_empty() {
				self.apply_translations(command.subcommands.as_mut(), Some(&command_name));

				continue;
			}

			// Add localizations
			for (locale, bundle) in &self.other {
				if locale == &crate::context::Locale::en_US {
					continue;
				}

				// Insert localized command name and description
				let localized_command_name = match format(bundle, &command_name, None, None) {
					Some(x) => x,
					None => {
						warn!(
							name = command_name,
							locale = locale.as_str(),
							"missing localization for command"
						);

						continue;
					}
				};

				if !NAME_REGEX.is_match(&localized_command_name) {
					panic!(
						"invalid localized command name `{}` in {}",
						localized_command_name,
						locale.as_str(),
					);
				}

				command
					.name_localizations
					.insert(locale.as_str().to_string(), localized_command_name);

				let description = format(bundle, &command_name, Some("description"), None);

				if let Some(description) = description {
					command
						.description_localizations
						.insert(locale.as_str().to_string(), description);
				} else {
					warn!(
						"missing command description localization for `{}` in {}",
						command_name,
						locale.as_str()
					);
				}

				for parameter in &mut command.parameters {
					let name = format(bundle, &command_name, Some(&parameter.name), None);

					if let Some(name) = name {
						if !NAME_REGEX.is_match(&name) {
							panic!(
								"invalid localized parameter name `{}` in {}",
								name,
								locale.as_str(),
							);
						}

						parameter
							.name_localizations
							.insert(locale.as_str().to_string(), name);
					} else {
						warn!(
							"missing parameter name localization for `{}` in {}",
							parameter.name,
							locale.as_str()
						);

						continue;
					}

					let description = format(
						bundle,
						&command_name,
						Some(&format!("{}-description", parameter.name)),
						None,
					);

					if let Some(description) = description {
						parameter
							.description_localizations
							.insert(locale.as_str().to_string(), description);
					} else {
						warn!(
							"missing parameter description localization for `{}` in {}",
							parameter.name,
							locale.as_str()
						);
					}

					// If this is a choice parameter, insert its localized variants
					for choice in &mut parameter.choices {
						let name = format(bundle, &choice.name, None, None);

						if let Some(name) = name {
							choice
								.localizations
								.insert(locale.as_str().to_string(), name);
						} else {
							warn!(
								"missing choice name localization for `{}` in {}",
								choice.name,
								locale.as_str()
							);
						}
					}
				}
			}

			// Set fallback command name and description to en-US
			let bundle = &self.main;
			let description = format(bundle, &command_name, Some("description"), None);

			if let Some(description) = description {
				command.description = Some(description);
			} else {
				panic!(
					"missing command description localization for `{}` in en-US",
					command_name
				);
			}

			for parameter in &mut command.parameters {
				let Some(name) = format(bundle, &command_name, Some(&parameter.name), None) else {
					panic!(
						"missing parameter localization for `{}` in en-US",
						parameter.name
					);
				};

				let description = format(
					bundle,
					&command_name,
					Some(&format!("{}-description", parameter.name)),
					None,
				);

				if let Some(description) = description {
					parameter.description = Some(description);
				} else {
					panic!(
						"missing parameter description localization for `{}` (command `{}`) in en-US",
						parameter.name, command_name
					);
				}

				// If this is a choice parameter, set the choice names to en-US
				for choice in &mut parameter.choices {
					let name = format(bundle, &choice.name.replace('_', "-"), None, None);

					if let Some(name) = name {
						choice.name = name;
					} else {
						panic!("missing choice localization for `{}` in en-US", choice.name);
					}
				}

				parameter.name = name;
			}
		}
	}
}

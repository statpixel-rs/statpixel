use darling::{ast, FromDeriveInput, FromVariant};
use proc_macro_crate::FoundCrate;
use quote::{quote, ToTokens};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(mode), supports(enum_unit))]
pub(crate) struct ModeTraitInputReceiver {
	pub ident: syn::Ident,
	pub generics: syn::Generics,
	pub data: ast::Data<FieldReceiver, ()>,
	pub kind: syn::Ident,
	pub rename: Option<syn::Ident>,
}

impl ToTokens for ModeTraitInputReceiver {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let ModeTraitInputReceiver {
			ident,
			generics,
			data,
			kind,
			rename,
		} = self;

		let fields = data.as_ref().take_enum().expect("should be a unit enum");

		let mode_ty = rename.clone().unwrap_or(ident.clone());

		let api = match proc_macro_crate::crate_name("api").unwrap() {
			FoundCrate::Itself => quote!(crate),
			FoundCrate::Name(name) => quote!(::#name),
		};

		let options_root = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #ty_str), #api::id::encode(#api::id::Id::Root {
					kind: #api::id::Mode::#mode_ty (Self::#ty),
					uuid,
				}))
			}
		});

		let options_snapshot = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #ty_str), #api::id::encode(#api::id::Id::Snapshot {
					kind: #api::id::Mode::#mode_ty (Self::#ty),
					uuid,
					past,
				}))
			}
		});

		let options_history = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #ty_str), #api::id::encode(#api::id::Id::History {
					kind: #api::id::Mode::#mode_ty (Self::#ty),
					uuid,
				}))
			}
		});

		let options_project = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				poise::serenity_prelude::CreateSelectMenuOption::new(::translate::tr(ctx, #ty_str), #api::id::encode(#api::id::Id::Project {
					kind: #api:id::ProjectMode::#mode_ty (Self::#ty, kind),
					uuid,
				}))
			}
		});

		let get_tr = fields.iter().map(|f| {
			let ty = &f.ident;
			let ty_str = quote!(#ty).to_string();
			let ty_str = ty_str.as_str();

			quote! {
				Self::#ty => #ty_str,
			}
		});

		tokens.extend(quote! {
			impl #generics #ident #generics {
				pub fn get_tr(&self) -> &'static str {
					match self {
						#(#get_tr)*
					}
				}
			}

			impl #api::canvas::prelude::Mode for #ident #generics {
				type Kind = #kind;

				fn as_root(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid, selected: Option<Self>
				) -> ::poise::serenity_prelude::CreateActionRow {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						ctx.id().to_string(),
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#(#options_root),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					::poise::serenity_prelude::CreateActionRow::SelectMenu(menu)
				}

				fn as_snapshot(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					past: i64,
					selected: Option<Self>
				) -> ::poise::serenity_prelude::CreateActionRow {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						ctx.id().to_string(),
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#(#options_snapshot),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					::poise::serenity_prelude::CreateActionRow::SelectMenu(menu)
				}

				fn as_history(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					selected: Option<Self>
				) -> ::poise::serenity_prelude::CreateActionRow {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						ctx.id().to_string(),
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#(#options_history),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					::poise::serenity_prelude::CreateActionRow::SelectMenu(menu)
				}

				fn as_project(
					ctx: &::translate::context::Context<'_>,
					uuid: ::uuid::Uuid,
					kind: Self::Kind,
					selected: Option<Self>
				) -> ::poise::serenity_prelude::CreateActionRow {
					let mut menu = ::poise::serenity_prelude::CreateSelectMenu::new(
						ctx.id().to_string(),
						::poise::serenity_prelude::CreateSelectMenuKind::String {
							options: ::std::vec![
								#(#options_project),*
							]
						}
					);

					if let Some(selected) = selected {
						menu = menu.placeholder(::translate::tr(ctx, selected.get_tr()));
					}

					menu = menu.max_values(1).min_values(1);

					::poise::serenity_prelude::CreateActionRow::SelectMenu(menu)
				}
			}
		});
	}
}

#[derive(Debug, FromVariant)]
#[darling(attributes(mode))]
pub(crate) struct FieldReceiver {
	pub ident: syn::Ident,
}

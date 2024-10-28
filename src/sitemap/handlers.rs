use axum::{
    response::{Html, IntoResponse},
    Extension,
};

use tera::Context;

use crate::{
	sitemap::views::{generate_links},
	common::Templates
};


pub async fn get_sitemap(
	Extension(templates): Extension<Templates>
) -> impl IntoResponse {

	let arg = generate_links().await;
	println!(" arg..{:#?}", arg);

	let mut context = Context::new();
	context.insert("arg", &arg);
	Html(templates.render("sitemap", &context).unwrap())
}
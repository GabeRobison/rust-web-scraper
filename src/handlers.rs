use actix_web::{get, web, HttpResponse, Result};
use crate::utils::scrape_wikipedia;

#[get("/web-scraper/{title}")]
pub async fn scrape(title: web::Path<String>) -> Result<HttpResponse> {
    // Fetch and parse Wikipedia content based on the given title
    let parsed_data = scrape_wikipedia(&title.into_inner()).await?;

    // Return the full legible text as a plain text HTTP response
    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(parsed_data.legible_text))
}

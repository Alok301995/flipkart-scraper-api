use flipkart_scraper::ProductSearch;
use serde::Serialize;
use std::env;

pub async fn search_product(query: String) -> Result<SearchResultResponse, String> {
    // Early return pattern for error handling
    let search = ProductSearch::search(query).await.map_err(|err| err.to_string())?;
    
    let ProductSearch {
        query,
        query_url,
        results,
    } = search;
    
    // Get deployment URL at runtime instead of compile time
    let host = env::var("DEPLOYMENT_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    // Transform search results
    let result = results
        .into_iter()
        .map(|p| {
            // Check if product is discounted
            let discounted = match (p.current_price, p.original_price) {
                (Some(current), Some(original)) => current < original,
                _ => false,
            };
            
            // Clean up query URL once
            let query_url = clean_product_url(&p.product_link);
            
            // Extract clean link
            let link = p.product_link
                .split_once("&q=")
                .map_or(p.product_link.clone(), |(link, _)| link.to_string());
            
            SearchResultProduct {
                name: p.product_name,
                link,
                current_price: p.current_price,
                original_price: p.original_price,
                discounted,
                thumbnail: p.thumbnail,
                query_url: format!("{}/product{}", host, query_url),
            }
        })
        .collect();
    
    Ok(SearchResultResponse {
        total_result: result.len(),
        query,
        fetch_from: query_url,
        result,
    })
}

// Helper function to clean product URLs
fn clean_product_url(url: &str) -> &str {
    let url = url.strip_prefix("https://").unwrap_or(url);
    let url = url.strip_prefix("http://").unwrap_or(url);
    let url = url.strip_prefix("dl.flipkart.com").unwrap_or(url);
    let url = url.strip_prefix("flipkart.com").unwrap_or(url);
    
    // Remove query parameters
    url.split_once('?').map_or(url, |(link, _)| link)
}

#[derive(Serialize)]
pub struct SearchResultProduct {
    name: String,
    link: String,
    current_price: Option<i32>,
    original_price: Option<i32>,
    discounted: bool,
    thumbnail: String,
    query_url: String,
}

#[derive(Serialize)]
pub struct SearchResultResponse {
    total_result: usize,
    query: String,
    fetch_from: String,
    result: Vec<SearchResultProduct>,
}
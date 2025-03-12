use flipkart_scraper::{product_details::*, Url};
use serde::Serialize;

pub async fn product_details(url: Url) -> Result<ProductDetailsResponse, String> {
    // Fetch product details
    let result = ProductDetails::fetch(url)
        .await
        .map_err(|e| e.to_string())?;
    
    // Calculate discount percentage once
    let discount_percent = match (result.current_price, result.original_price) {
        (Some(current), Some(original)) if current < original => 
            Some(((original - current) * 100) / original),
        _ => None,
    };
    
    // Determine if product is discounted
    let discounted = discount_percent.is_some();
    
    // Transform offers with single iterator
    let offers = result.offers
        .into_iter()
        .map(|o| OfferResponse {
            offer_type: o.category,
            description: o.description,
        })
        .collect();
    
    // Transform specifications with single iterator
    let specs = result.specifications
        .into_iter()
        .map(|s| SpecsResponse {
            title: s.category,
            details: s.specifications
                .into_iter()
                .map(|d| SpecResponse {
                    property: d.name,
                    value: d.value,
                })
                .collect(),
        })
        .collect();
    
    // Create seller response if seller exists
    let seller = result.seller.map(|s| SellerResponse {
        seller_name: s.name,
        seller_rating: s.rating,
    });
    
    // Construct response with all fields
    Ok(ProductDetailsResponse {
        name: result.name,
        current_price: result.current_price,
        original_price: result.original_price,
        discounted,
        discount_percent,
        rating: result.rating,
        in_stock: result.in_stock,
        f_assured: result.f_assured,
        share_url: result.share_url,
        seller,
        thumbnails: result.thumbnails,
        highlights: result.highlights,
        product_id: result.product_id,
        offers,
        specs,
    })
}

#[derive(Serialize)]
pub struct SellerResponse {
    seller_name: String,
    seller_rating: Option<f32>,
}

#[derive(Serialize)]
pub struct OfferResponse {
    offer_type: Option<String>,
    description: String,
}

#[derive(Serialize)]
pub struct SpecsResponse {
    title: String,
    details: Vec<SpecResponse>,
}

#[derive(Serialize)]
pub struct SpecResponse {
    property: String,
    value: String,
}

#[derive(Serialize)]
pub struct ProductDetailsResponse {
    name: Option<String>,
    current_price: Option<i32>,
    original_price: Option<i32>,
    discounted: bool,
    discount_percent: Option<i32>,
    rating: Option<f32>,
    in_stock: bool,
    f_assured: bool,
    share_url: String,
    seller: Option<SellerResponse>,
    thumbnails: Vec<String>,
    highlights: Vec<String>,
    product_id: Option<String>,
    offers: Vec<OfferResponse>,
    specs: Vec<SpecsResponse>,
}
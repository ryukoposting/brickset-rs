//! Response parsers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::util;

/// Response to a failed request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub message: String,
}

/// Response to a successful `checkKey` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckKeyResponse {}

/// Response to a successful `login` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub hash: String
}

/// Response to a successful `checkUserHash` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CheckUserHashResponse {}

/// Response to a successful `getKeyUsageStats` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetKeyUsageStatsResponse {
    pub matches: usize,
    pub api_key_usage: Vec<ApiKeyUsage>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyUsage {
    pub date_stamp: DateTime<Utc>,
    pub count: usize
}

/// Response to a successful `getSets` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSetsResponse {
    pub matches: usize,
    pub sets: Vec<Set>
}

/// Response to a successful `getAdditionalImages` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetAdditionalImagesResponse {
    pub matches: usize,
    pub additional_images: Vec<Image>
}

/// Response to a successful `getInstructions` or `getInstructions2` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetInstructionsResponse {
    pub matches: usize,
    pub instructions: Vec<Instructions>
}

/// Response to a successful `getReviews` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetReviewsResponse {
    pub matches: usize,
    pub reviews: Vec<Review>
}

/// Response to a successful `getThemes` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetThemesResponse {
    pub matches: usize,
    pub themes: Vec<Theme>
}

/// Response to a successful `getSubthemes` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSubthemesResponse {
    pub matches: usize,
    pub subthemes: Vec<Subtheme>
}

/// Response to a successful `getYears` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetYearsResponse {
    pub matches: usize,
    pub years: Vec<Year>
}

/// Response to a successful `setCollection` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCollectionResponse {}

/// Response to a successful `getUserNotes` request.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetUserNotesResponse {
    pub matches: usize,
    pub user_notes: Vec<UserNote>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMinifigCollectionResponse {
    pub matches: usize,
    pub minifigs: Vec<MinifigCollection>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMinifigCollectionResponse {}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMinifigUserNotesResponse {
    pub matches: usize,
    pub user_minifig_notes: Vec<UserMinifigNote>
}



#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Set {
    #[serde(rename = "setID")]
    pub set_id: u64,
    pub number: String,
    pub number_variant: usize,
    #[serde(default)]
    #[serde(with = "util::not_specified_optional_string")]
    pub name: Option<String>,
    pub year: i32,
    #[serde(default)]
    #[serde(with = "util::not_specified_optional_string")]
    pub theme: Option<String>,
    #[serde(default)]
    #[serde(with = "util::not_specified_optional_string")]
    pub theme_group: Option<String>,
    #[serde(default)]
    #[serde(with = "util::not_specified_optional_string")]
    pub subtheme: Option<String>,
    #[serde(default)]
    #[serde(with = "util::not_specified_optional_string")]
    pub category: Option<String>,
    pub released: bool,
    pub pieces: Option<usize>,
    pub minifigs: Option<usize>,
    pub image: Image,
    #[serde(rename = "bricksetURL")]
    pub brickset_url: String,
    pub collection: Collection,
    pub collections: Collections,
    #[serde(rename = "LEGOCom")]
    pub lego_com: LegoCom,
    pub rating: f64,
    pub review_count: usize,
    #[serde(with = "util::not_specified_optional_string")]
    pub packaging_type: Option<String>,
    #[serde(with = "util::not_specified_optional_string")]
    pub availability: Option<String>,
    pub instructions_count: usize,
    pub additional_image_count: usize,
    pub age_range: AgeRange,
    pub dimensions: Dimensions,
    pub barcode: Barcode,
    pub extended_data: ExtendedData,
    #[serde(default)]
    pub last_updated: Option<DateTime<Utc>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Image {
    #[serde(rename = "thumbnailURL")]
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "imageURL")]
    #[serde(default)]
    pub image_url: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(default)]
    #[serde(rename = "owned")]
    pub is_owned: Option<bool>,
    #[serde(default)]
    #[serde(rename = "wanted")]
    pub is_wanted: Option<bool>,
    #[serde(default)]
    pub qty_owned: Option<usize>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collections {
    #[serde(default)]
    pub owned_by: Option<usize>,
    #[serde(default)]
    pub wanted_by: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LegoCom {
    #[serde(rename = "US")]
    pub united_states: LegoComDetails,
    #[serde(rename = "UK")]
    pub united_kingdom: LegoComDetails,
    #[serde(rename = "CA")]
    pub canada: LegoComDetails,
    #[serde(rename = "DE")]
    pub germany: LegoComDetails,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LegoComDetails {
    #[serde(default)]
    pub retail_price: Option<f64>,
    #[serde(default)]
    pub date_first_available: Option<DateTime<Utc>>,
    #[serde(default)]
    pub date_last_available: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgeRange {
    #[serde(default)]
    pub min: Option<f64>,
    #[serde(default)]
    pub max: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dimensions {
    #[serde(default)]
    pub height: Option<f64>,
    #[serde(default)]
    pub width: Option<f64>,
    #[serde(default)]
    pub depth: Option<f64>,
    #[serde(default)]
    pub weight: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Barcode {
    #[serde(default)]
    #[serde(rename = "UPC")]
    pub upc: Option<String>,
    #[serde(default)]
    #[serde(rename = "EAN")]
    pub ean: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedData {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instructions {
    #[serde(rename = "URL")]
    pub url: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub author: String,
    pub date_posted: DateTime<Utc>,
    pub rating: Rating,
    pub title: String,
    pub review: String,
    #[serde(rename = "HTML")]
    pub html: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    #[serde(rename = "theme")]
    pub name: String,
    pub set_count: usize,
    pub subtheme_count: usize,
    pub year_from: i32,
    pub year_to: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subtheme {
    pub theme: String,
    #[serde(rename = "subtheme")]
    pub name: String,
    pub set_count: usize,
    pub year_from: i32,
    pub year_to: i32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Year {
    pub theme: String,
    pub year: i32,
    pub set_count: usize
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    pub overall: i32,
    #[serde(with = "util::zero_none")]
    pub parts: Option<i32>,
    #[serde(with = "util::zero_none")]
    pub building_experience: Option<i32>,
    #[serde(with = "util::zero_none")]
    pub playability: Option<i32>,
    #[serde(with = "util::zero_none")]
    pub value_for_money: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserNote {
    #[serde(rename = "setID")]
    pub set_id: u64,
    pub notes: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MinifigCollection {
    pub minifig_number: String,
    pub name: String,
    pub category: String,
    pub owned_in_sets: usize,
    pub owned_loose: usize,
    pub owned_total: usize,
    pub wanted: bool
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserMinifigNote {
    pub minifig_number: String,
    pub notes: String
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

//! Request builders.

use super::util::{self, Flag};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

#[cfg(feature = "log")]
use log::warn;

lazy_static! {
    pub static ref ENDPOINT: url::Url = Url::parse("https://brickset.com/api/v3.asmx/").unwrap();
}

#[derive(Debug)]
pub enum Error {
    UrlParseError(url::ParseError),
    SerdeJson(serde_json::Error),
    Message(String),
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),
}

#[derive(Debug, Clone)]
pub struct CheckKey<'s> {
    api_key: &'s str,
}

#[derive(Debug, Clone)]
pub struct Login<'s> {
    api_key: &'s str,
    username: &'s str,
    password: &'s str,
}

#[derive(Debug, Clone)]
pub struct CheckUserHash<'s> {
    api_key: &'s str,
    user_hash: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetKeyUsageStats<'s> {
    api_key: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetSets<'s> {
    api_key: &'s str,
    user_hash: Option<&'s str>,
    params: GetSetsParameters<'s>,
}

#[derive(Debug, Clone)]
pub struct GetAdditionalImages<'s> {
    api_key: &'s str,
    set_id: u64,
}

#[derive(Debug, Clone)]
pub struct GetInstructions<'s> {
    api_key: &'s str,
    set_id: u64,
}

#[derive(Debug, Clone)]
pub struct GetInstructions2<'s> {
    api_key: &'s str,
    set_number: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetReviews<'s> {
    api_key: &'s str,
    set_id: u64,
}

#[derive(Debug, Clone)]
pub struct GetThemes<'s> {
    api_key: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetSubthemes<'s> {
    api_key: &'s str,
    theme: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetYears<'s> {
    api_key: &'s str,
    theme: &'s str,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum OrderBy {
    Number,
    YearFrom,
    Pieces,
    Minifigs,
    Rating,
    USRetailPrice,
    UKRetailPrice,
    CARetailPrice,
    DERetailPrice,
    FRRetailPrice,
    USPricePerPiece,
    UKPricePerPiece,
    CAPricePerPiece,
    DEPricePerPiece,
    FRPricePerPiece,
    Theme,
    Subtheme,
    Name,
    Random,
    QtyOwned,
    OwnCount,
    WantCount,
    UserRating,
    CollectionID,
    NumberDESC,
    YearFromDESC,
    PiecesDESC,
    MinifigsDESC,
    RatingDESC,
    USRetailPriceDESC,
    UKRetailPriceDESC,
    CARetailPriceDESC,
    DERetailPriceDESC,
    FRRetailPriceDESC,
    USPricePerPieceDESC,
    UKPricePerPieceDESC,
    CAPricePerPieceDESC,
    DEPricePerPieceDESC,
    FRPricePerPieceDESC,
    ThemeDESC,
    SubthemeDESC,
    NameDESC,
    RandomDESC,
    QtyOwnedDESC,
    OwnCountDESC,
    WantCountDESC,
    UserRatingDESC,
    CollectionIDDESC,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetSetsParameters<'s> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    set_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    query: Option<&'s str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    theme: Option<&'s str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    subtheme: Option<&'s str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(rename = "setNumber")]
    full_set_number: Option<&'s str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(with = "util::int_vec_as_commastr")]
    year: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    tag: Option<&'s str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    owned: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    wanted: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "util::updated_since_format")]
    #[serde(default)]
    updated_since: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    order_by: Option<OrderBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    page_size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    page_number: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    extended_data: Option<Flag>,
}

#[derive(Debug, Clone)]
pub struct SetCollection<'s> {
    api_key: &'s str,
    user_hash: &'s str,
    set_id: u64,
    params: SetCollectionParameters<'s>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetCollectionParameters<'s> {
    #[serde(skip_serializing_if = "Option::is_none")]
    own: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    want: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qty_owned: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<&'s str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct GetUserNotes<'s> {
    api_key: &'s str,
    user_hash: &'s str,
}

#[derive(Debug, Clone)]
pub struct GetMinifigCollection<'s> {
    api_key: &'s str,
    user_hash: &'s str,
    params: GetMinifigCollectionParameters<'s>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetMinifigCollectionParameters<'s> {
    #[serde(skip_serializing_if = "Option::is_none")]
    owned: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wanted: Option<Flag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    query: Option<&'s str>,
}

#[derive(Debug, Clone)]
pub struct SetMinifigCollection<'s> {
    api_key: &'s str,
    user_hash: &'s str,
    minifig_number: &'s str,
    params: SetMinifigCollectionParameters<'s>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetMinifigCollectionParameters<'s> {
    #[serde(skip_serializing_if = "Option::is_none")]
    own: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    want: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    qty_owned: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notes: Option<&'s str>,
}

#[derive(Debug, Clone)]
pub struct GetUserMinifigNotes<'s> {
    api_key: &'s str,
    user_hash: &'s str
}

impl<'s> CheckKey<'s> {
    pub fn new(api_key: &'s str) -> Self {
        CheckKey { api_key }
    }
}

impl<'s> Login<'s> {
    pub fn new(api_key: &'s str, username: &'s str, password: &'s str) -> Self {
        Login {
            api_key,
            username,
            password,
        }
    }
}

impl<'s> CheckUserHash<'s> {
    pub fn new(api_key: &'s str, user_hash: &'s str) -> Self {
        CheckUserHash { api_key, user_hash }
    }
}

impl<'s> GetKeyUsageStats<'s> {
    pub fn new(api_key: &'s str) -> Self {
        GetKeyUsageStats { api_key }
    }
}

impl<'s> GetSets<'s> {
    pub fn new(
        api_key: &'s str,
        user_hash: Option<&'s str>,
        params: GetSetsParameters<'s>,
    ) -> Self {
        Self {
            api_key,
            user_hash,
            params,
        }
    }
}

impl<'s> GetAdditionalImages<'s> {
    pub fn new(api_key: &'s str, set_id: u64) -> Self {
        Self { api_key, set_id }
    }
}

impl<'s> GetInstructions<'s> {
    pub fn new(api_key: &'s str, set_id: u64) -> Self {
        Self { api_key, set_id }
    }
}

impl<'s> GetInstructions2<'s> {
    pub fn new(api_key: &'s str, set_number: &'s str) -> Self {
        Self {
            api_key,
            set_number,
        }
    }
}

impl<'s> GetReviews<'s> {
    pub fn new(api_key: &'s str, set_id: u64) -> Self {
        Self { api_key, set_id }
    }
}

impl<'s> GetThemes<'s> {
    pub fn new(api_key: &'s str) -> Self {
        Self { api_key }
    }
}

impl<'s> GetSubthemes<'s> {
    pub fn new(api_key: &'s str, theme: &'s str) -> Self {
        Self { api_key, theme }
    }
}

impl<'s> GetYears<'s> {
    pub fn new(api_key: &'s str, theme: &'s str) -> Self {
        Self { api_key, theme }
    }
}

impl<'s> SetCollection<'s> {
    pub fn new(
        api_key: &'s str,
        user_hash: &'s str,
        set_id: u64,
        params: SetCollectionParameters<'s>,
    ) -> Self {
        Self {
            api_key,
            user_hash,
            set_id,
            params,
        }
    }
}

impl<'s> GetSetsParameters<'s> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter results to show only a single set ID.
    #[inline]
    pub fn set_id(mut self, set_id: u64) -> Self {
        self.set_id = Some(set_id);
        self
    }

    /// Filter results to show sets from a single year.
    #[inline]
    pub fn year(mut self, year: i32) -> Self {
        self.year.clear();
        self.year.push(year);
        self
    }

    /// Filter results to show sets from multiple years.
    #[inline]
    pub fn years(mut self, year: Vec<i32>) -> Self {
        self.year = year;
        self
    }

    /// Query text. Searches set number, name, theme, and subtheme.
    #[inline]
    pub fn query(mut self, query: &'s str) -> Self {
        self.query = Some(query);
        self
    }

    /// Filter results to show sets from a single theme.
    #[inline]
    pub fn theme(mut self, theme: &'s str) -> Self {
        self.theme = Some(theme);
        self
    }

    /// Filter results to show sets from a single sub-theme.
    #[inline]
    pub fn subtheme(mut self, subtheme: &'s str) -> Self {
        self.subtheme = Some(subtheme);
        self
    }

    /// Full set number, including the variant number. ex: "6876-1"
    #[inline]
    pub fn full_set_number(mut self, full_set_number: &'s str) -> Self {
        self.full_set_number = Some(full_set_number);
        self
    }

    /// Filter results to show sets with a particular tag.
    #[inline]
    pub fn tag(mut self, tag: &'s str) -> Self {
        self.tag = Some(tag);
        self
    }

    /// Only show sets owned by the user.
    #[inline]
    pub fn owned_by_user(mut self, owned: bool) -> Self {
        self.owned = if owned { Some(Flag) } else { None };
        self
    }

    /// Only show sets wanted by the user.
    #[inline]
    pub fn wanted_by_user(mut self, wanted: bool) -> Self {
        self.wanted = if wanted { Some(Flag) } else { None };
        self
    }

    /// Retrieve extended information about sets, including tags, description, and notes.
    #[inline]
    pub fn extended_data(mut self, extended_data: bool) -> Self {
        self.extended_data = if extended_data { Some(Flag) } else { None };
        self
    }

    /// Filter results to only show sets updated since the given date.
    #[inline]
    pub fn updated_since<D: Into<NaiveDate>>(mut self, date: D) -> Self {
        self.updated_since = Some(date.into());
        self
    }

    /// Sort the results.
    #[inline]
    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Specify the number of sets to retrieve. Maximum = 500, default = 20.
    #[inline]
    pub fn page_size(mut self, page_size: usize) -> Self {
        if page_size > 500 {
            #[cfg(feature = "log")]
            warn!("Given page_size was {page_size}, but the maximum is 500");
        } else if page_size == 0 {
            #[cfg(feature = "log")]
            warn!("Zero page size is not valid");
        }

        self.page_size = Some(page_size);
        self
    }

    /// Specify which page of sets to retrieve. Should be used in conjunction with 
    /// [`Self::page_size`]. Default = 1
    #[inline]
    pub fn page_number(mut self, page_number: usize) -> Self {
        self.page_number = Some(page_number);
        self
    }
}

impl<'s> SetCollectionParameters<'s> {
    /// new, empty SetCollectionParameters. These parameters will not alter BrickSet's database.
    /// To alter a database entry, use these functions:
    /// - [`Self::owned`]
    /// - [`Self::wanted`]
    /// - [`Self::notes`]
    /// - [`Self::rating`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or remove this set from the list of sets owned by the user. The user owns `qty_owned`
    /// of this set. Remove the set from the user's owned list by setting `qty_owned` to zero.
    pub fn owned(mut self, qty_owned: usize) -> Self {
        if qty_owned == 0 {
            self.own = None;
        } else {
            self.own = Some(1);
        }
        self.qty_owned = Some(qty_owned);
        self
    }

    /// Add or remove this set from the user's wanted list.
    pub fn wanted(mut self, wanted: bool) -> Self {
        if wanted {
            self.want = Some(1);
        } else {
            self.want = Some(0);
        }
        self
    }

    /// Alter the user's notes for the set.
    pub fn notes(mut self, notes: &'s str) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Alter the user's rating of the set.
    pub fn rating(mut self, rating: i32) -> Self {
        self.rating = Some(rating);
        self
    }
}

impl<'s> GetUserNotes<'s> {
    pub fn new(api_key: &'s str, user_hash: &'s str) -> Self {
        Self { api_key, user_hash }
    }
}

impl<'s> GetMinifigCollection<'s> {
    pub fn new(api_key: &'s str, user_hash: &'s str, params: GetMinifigCollectionParameters<'s>) -> Self {
        Self { api_key, user_hash, params }
    }
}

impl<'s> GetMinifigCollectionParameters<'s> {
    /// Get minifigs owned by the user.
    pub fn owned() -> Self {
        Self { owned: Some(Flag), ..Default::default() }
    }

    /// Get minifigs wanted by the user.
    pub fn wanted() -> Self {
        Self { wanted: Some(Flag), ..Default::default() }
    }

    /// Add query text to the request. The query will search by minifig ID number, and name.
    pub fn query(mut self, query: &'s str) -> Self {
        self.query = Some(query);
        self
    }
}

impl<'s> SetMinifigCollection<'s> {
    pub fn new(api_key: &'s str, user_hash: &'s str, minifig_number: &'s str, params: SetMinifigCollectionParameters<'s>) -> Self {
        Self { api_key, user_hash, minifig_number, params }
    }
}

impl<'s> SetMinifigCollectionParameters<'s> {
    /// new, empty SetMinifigCollectionParameters. These parameters will not alter BrickSet's database.
    /// /// To alter a database entry, use these functions:
    /// - [`Self::owned`]
    /// - [`Self::wanted`]
    /// - [`Self::notes`]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or remove this minifig from the list of minifigs owned by the user. The user owns `qty_owned`
    /// of this minifig. Remove the minifig from the user's owned list by setting `qty_owned` to zero.
    pub fn owned(mut self, qty_owned: usize) -> Self {
        if qty_owned == 0 {
            self.own = Some(0);
            self.qty_owned = None;
        } else {
            self.own = None;
            self.qty_owned = Some(qty_owned);
        }
        self
    }

    /// Add or remove this minifig from the user's wanted list.
    pub fn wanted(mut self, wanted: bool) -> Self {
        if wanted {
            self.want = Some(1);
        } else {
            self.want = Some(0);
        }
        self
    }

    /// Alter the user's notes for the minifig.
    pub fn notes(mut self, notes: &'s str) -> Self {
        self.notes = Some(notes);
        self
    }
}

impl<'s> GetUserMinifigNotes<'s> {
    pub fn new(api_key: &'s str, user_hash: &'s str) -> Self {
        Self { api_key, user_hash }
    }
}

impl OrderBy {
    /// Get the reversed version of an [`OrderBy`]. For example,
    /// `OrderBy::Number.reversed()` will return `OrderBy::NumberDESC`,
    /// and vice versa.
    pub fn reversed(self) -> Self {
        match self {
            OrderBy::Number => OrderBy::NumberDESC,
            OrderBy::YearFrom => OrderBy::YearFromDESC,
            OrderBy::Pieces => OrderBy::PiecesDESC,
            OrderBy::Minifigs => OrderBy::MinifigsDESC,
            OrderBy::Rating => OrderBy::RatingDESC,
            OrderBy::USRetailPrice => OrderBy::USRetailPriceDESC,
            OrderBy::UKRetailPrice => OrderBy::UKRetailPriceDESC,
            OrderBy::CARetailPrice => OrderBy::CARetailPriceDESC,
            OrderBy::DERetailPrice => OrderBy::DERetailPriceDESC,
            OrderBy::FRRetailPrice => OrderBy::FRRetailPriceDESC,
            OrderBy::USPricePerPiece => OrderBy::USPricePerPieceDESC,
            OrderBy::UKPricePerPiece => OrderBy::UKPricePerPieceDESC,
            OrderBy::CAPricePerPiece => OrderBy::CAPricePerPieceDESC,
            OrderBy::DEPricePerPiece => OrderBy::DEPricePerPieceDESC,
            OrderBy::FRPricePerPiece => OrderBy::FRPricePerPieceDESC,
            OrderBy::Theme => OrderBy::ThemeDESC,
            OrderBy::Subtheme => OrderBy::SubthemeDESC,
            OrderBy::Name => OrderBy::NameDESC,
            OrderBy::Random => OrderBy::RandomDESC,
            OrderBy::QtyOwned => OrderBy::QtyOwnedDESC,
            OrderBy::OwnCount => OrderBy::OwnCountDESC,
            OrderBy::WantCount => OrderBy::WantCountDESC,
            OrderBy::UserRating => OrderBy::UserRatingDESC,
            OrderBy::CollectionID => OrderBy::CollectionIDDESC,
            OrderBy::NumberDESC => OrderBy::Number,
            OrderBy::YearFromDESC => OrderBy::YearFrom,
            OrderBy::PiecesDESC => OrderBy::Pieces,
            OrderBy::MinifigsDESC => OrderBy::Minifigs,
            OrderBy::RatingDESC => OrderBy::Rating,
            OrderBy::USRetailPriceDESC => OrderBy::USRetailPrice,
            OrderBy::UKRetailPriceDESC => OrderBy::UKRetailPrice,
            OrderBy::CARetailPriceDESC => OrderBy::CARetailPrice,
            OrderBy::DERetailPriceDESC => OrderBy::DERetailPrice,
            OrderBy::FRRetailPriceDESC => OrderBy::FRRetailPrice,
            OrderBy::USPricePerPieceDESC => OrderBy::USPricePerPiece,
            OrderBy::UKPricePerPieceDESC => OrderBy::UKPricePerPiece,
            OrderBy::CAPricePerPieceDESC => OrderBy::CAPricePerPiece,
            OrderBy::DEPricePerPieceDESC => OrderBy::DEPricePerPiece,
            OrderBy::FRPricePerPieceDESC => OrderBy::FRPricePerPiece,
            OrderBy::ThemeDESC => OrderBy::Theme,
            OrderBy::SubthemeDESC => OrderBy::Subtheme,
            OrderBy::NameDESC => OrderBy::Name,
            OrderBy::RandomDESC => OrderBy::Random,
            OrderBy::QtyOwnedDESC => OrderBy::QtyOwned,
            OrderBy::OwnCountDESC => OrderBy::OwnCount,
            OrderBy::WantCountDESC => OrderBy::WantCount,
            OrderBy::UserRatingDESC => OrderBy::UserRating,
            OrderBy::CollectionIDDESC => OrderBy::CollectionID,
        }
    }
}

/// Implemented by any type that can be turned into a BrickSet API request.
///
/// - [`BricksetRequest::to_request_url`] creates a URL containing the request method and query parameters.
/// - [`BricksetRequest::to_reqwest`] creates a POST [`reqwest::Request`] with the query paramters url-encoded in the body.
pub trait BricksetRequest {
    /// Encode method parameters via a URL serializer.
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target;

    /// The request's method name.
    fn method_name(&self) -> &'static str;

    /// Create a URL representing the request. All request parameters will appear in the URL.
    /// 
    /// NOTE: It is better practice to only put the method name in the request URL, and use
    /// [`Self::encode_query`] to put the parameters in the request's body.
    fn to_request_url(&self) -> Result<Url, Error> {
        let mut result = ENDPOINT.join(self.method_name())?;
        self.encode_query(&mut result.query_pairs_mut())?;
        Ok(result)
    }

    /// Build a [`reqwest::Request`] from `self`. The resulting [`reqwest::Request`] will
    /// always be a POST request with all method parameters encoded into the body, using
    /// content type `application/x-www-form-urlencoded`.
    #[cfg(feature = "reqwest")]
    fn to_reqwest(&self, client: &reqwest::Client) -> Result<reqwest::Request, Error> {
        let url = ENDPOINT.join(self.method_name())?;

        let mut body = url::form_urlencoded::Serializer::new(String::new());

        self.encode_query(&mut body)?;

        let body = body.finish();

        Ok(client
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .header(reqwest::header::CONTENT_LENGTH, body.as_bytes().len())
            .body(body)
            .build()?)
    }
}

impl<'s> BricksetRequest for CheckKey<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query.append_pair("apiKey", self.api_key);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "checkKey"
    }
}

impl<'s> BricksetRequest for Login<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("username", self.username)
            .append_pair("password", self.password);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "login"
    }
}

impl<'s> BricksetRequest for CheckUserHash<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "checkUserHash"
    }
}

impl<'s> BricksetRequest for GetKeyUsageStats<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query.append_pair("apiKey", self.api_key);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getKeyUsageStats"
    }
}

impl<'s> BricksetRequest for GetSets<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        let params = serde_json::to_string(&self.params)?;

        if (self.params.wanted.is_some() || self.params.owned.is_some()) && self.user_hash.is_none() {
            #[cfg(feature = "log")]
            warn!("User hash is required when wanted/owned parameters are used in GetSets");
        }

        query
            .append_pair("apiKey", self.api_key)
            .append_pair("params", &params)
            .append_pair("userHash", self.user_hash.unwrap_or(""));
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getSets"
    }
}

impl<'s> BricksetRequest for GetAdditionalImages<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("setID", self.set_id.to_string().as_str());
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getAdditionalImages"
    }
}

impl<'s> BricksetRequest for GetInstructions<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("setID", self.set_id.to_string().as_str());
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getInstructions"
    }
}

impl<'s> BricksetRequest for GetInstructions2<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("setNumber", self.set_number);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getInstructions2"
    }
}

impl<'s> BricksetRequest for GetReviews<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("setID", self.set_id.to_string().as_str());
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getReviews"
    }
}

impl<'s> BricksetRequest for GetThemes<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query.append_pair("apiKey", self.api_key);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getThemes"
    }
}

impl<'s> BricksetRequest for GetSubthemes<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("theme", self.theme);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getSubthemes"
    }
}

impl<'s> BricksetRequest for GetYears<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("theme", self.theme);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getYears"
    }
}

impl<'s> BricksetRequest for SetCollection<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        let params = serde_json::to_string(&self.params)?;

        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash)
            .append_pair("setID", self.set_id.to_string().as_str())
            .append_pair("params", &params);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "setCollection"
    }
}

impl<'s> BricksetRequest for GetUserNotes<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getUserNotes"
    }
}

impl<'s> BricksetRequest for GetMinifigCollection<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        let params = serde_json::to_string(&self.params)?;

        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash)
            .append_pair("params", params.as_str());
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getMinifigCollection"
    }
}

impl<'s> BricksetRequest for SetMinifigCollection<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target,
    {
        let params = serde_json::to_string(&self.params)?;

        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash)
            .append_pair("minifigNumber", self.minifig_number.to_string().as_str())
            .append_pair("params", params.as_str());
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "setMinifigCollection"
    }
}

impl<'s> BricksetRequest for GetUserMinifigNotes<'s> {
    fn encode_query<T>(&self, query: &mut url::form_urlencoded::Serializer<T>) -> Result<(), Error>
    where
        T: url::form_urlencoded::Target {
        query
            .append_pair("apiKey", self.api_key)
            .append_pair("userHash", self.user_hash);
        Ok(())
    }

    fn method_name(&self) -> &'static str {
        "getUserMinifigNotes"
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson(value)
    }
}

#[cfg(feature = "reqwest")]
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::Reqwest(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UrlParseError(err) => err.fmt(f),
            Error::Message(err) => f.write_str(&err),
            Error::SerdeJson(err) => err.fmt(f),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

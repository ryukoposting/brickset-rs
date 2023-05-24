//! [`ClientWrapper`] is a high-level wrapper for [`reqwest`].
//! 
//! # Sample
//! 
//! This sample program logs into BrickSet using a username and password, then
//! retrieves a list of all fire trucks owned by the logged-in user.
//! 
//! ```no_run
//! use brickset::{reqwest_api::ClientWrapper, request::GetSetsParameters};
//! use std::error::Error;
//! 
//! const API_KEY: &str = "<your API key>";
//! const USERNAME: &str = "<your BrickSet username>";
//! const PASSWORD: &str = "<your BrickSet password>";
//! 
//! #[tokio::main]
//! async fn main() {
//!     let reqwest_client = reqwest::Client::default();
//!     let mut client = ClientWrapper::new(API_KEY, &reqwest_client);
//!
//!     client.log_in(USERNAME, PASSWORD).await.expect("log_in");
//! 
//!     let params = GetSetsParameters::new()
//!         .owned_by_user(true)
//!         .query("fire truck");
//!     
//!     let sets = client.get_sets(params).await.expect("get_sets");
//!     
//!     println!("Found {} matching sets", sets.matches);
//!     for set in sets.sets.iter() {
//!         println!("{} {}", set.number, set.name.as_deref().unwrap_or("(Unknown)"));
//!     }
//! }
//! ```
//! 

use reqwest::Client;
use serde_json;

#[cfg(feature = "log")]
use log::debug;

use super::{Response, response, request::{self, BricksetRequest, GetMinifigCollectionParameters, SetMinifigCollectionParameters}};

type Result<T, E = Error> = std::result::Result<T, E>;

type RespResult<T> = std::result::Result<T, response::Error>;

/// Wraps a [`reqwest::Client`] with convenient functions for accessing the
/// BrickSet API, including rudimentary session management.
pub struct ClientWrapper<'a> {
    client: &'a Client,
    api_key: &'a str,
    user_hash: Option<String>
}

/// Errors that can be returned by [`ClientWrapper`] API calls.
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Request(request::Error),
    Response(response::Error),
    Json(serde_json::Error),
    Http {
        response: reqwest::Response
    },
    /// Tried to call a [`ClientWrapper`] function that requires a logged-in user,
    /// but the client is not logged in.
    NotLoggedIn
}

impl<'a> ClientWrapper<'a> {
    /// Create a new [`ClientWrapper`] that will use the given [`reqwest::Client`] and API key.
    pub fn new(api_key: &'a str, client: &'a Client) -> ClientWrapper<'a> {
        ClientWrapper { client, api_key, user_hash: None }
    }

    /// Check if the [`ClientWrapper`]'s API key is valid.
    /// 
    /// This function can be used even when the [`ClientWrapper`] is not logged in.
    pub async fn check_key(&self) -> Result<response::CheckKeyResponse> {
        let request = request::CheckKey::new(self.api_key);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }
    
    /// Get information about API key usage.
    /// 
    /// This function can be used even when the [`ClientWrapper`] is not logged in.
    pub async fn get_key_usage_stats(&self) -> Result<response::GetKeyUsageStatsResponse> {
        let request = request::GetKeyUsageStats::new(self.api_key);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Log into Brickset with the given username and password. The resulting user hash token
    /// will be used for subsequent requests until [`Self::log_out`] is called.
    pub async fn log_in(&mut self, username: &str, password: &str) -> Result<response::LoginResponse> {
        let request = request::Login::new(self.api_key, username, password);
        let response = self.execute(request).await?;
        let result: response::LoginResponse = RespResult::from(response)?;

        self.user_hash = Some(result.hash.clone());
        Ok(result)
    }

    /// Try to log in using an existing user hash token. This function will return
    /// an error if the token has expired.
    pub async fn reuse_login(&mut self, user_hash: &str) -> Result<response::CheckUserHashResponse> {
        let result = self.check_user_hash(user_hash).await?;
        self.force_reuse_login(user_hash);
        Ok(result)
    }

    /// Log in by reusing an existing user hash token. Be careful! This function
    /// will succeed even if the token is invalid. Consider using [`Self::reuse_login`]
    /// instead.
    pub fn force_reuse_login(&mut self, user_hash: &str) {
        self.user_hash = Some(user_hash.to_string())
    }

    /// Validate a user hash token.
    ///
    /// This function can be used even when the [`ClientWrapper`] is not logged in.
    pub async fn check_user_hash(&self, user_hash: &str) -> Result<response::CheckUserHashResponse> {
        let request = request::CheckUserHash::new(self.api_key, user_hash);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Same as [`Self::check_user_hash`], but validates the user token currently being used
    /// by the [`ClientWrapper`].
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn validate_login(&self) -> Result<response::CheckUserHashResponse> {
        match &self.user_hash {
            Some(user_hash) => self.check_user_hash(user_hash).await,
            None => Err(Error::NotLoggedIn)
        }
    }

    /// Forget the saved user hash token. If the [`ClientWrapper`] isn't logged in,
    /// this function has no effect.
    pub fn log_out(&mut self) {
        self.user_hash = None;
    }

    /// Returns true if the [`ClientWrapper`] is currently logged in
    #[inline]
    pub fn is_logged_in(&self) -> bool {
        self.user_hash.is_some()
    }

    /// Retrieve a paginated list of sets, or more information about a particular set. You may
    /// find these functions convenient for some use cases:
    /// 
    /// - [`Self::get_wanted_sets`]
    /// - [`Self::get_owned_sets`]
    pub async fn get_sets<'s>(&self, params: request::GetSetsParameters<'s>) -> Result<response::GetSetsResponse> {
        let request = request::GetSets::new(self.api_key, self.user_hash.as_deref(), params);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get the user's wanted sets. For additional filtering options, use [`Self::get_sets`].
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn get_wanted_sets(&self, order_by: Option<request::OrderBy>, page_size: Option<usize>, page_number: Option<usize>, extended_data: bool) -> Result<response::GetSetsResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn)
        }
        let mut params = request::GetSetsParameters::new()
            .wanted_by_user(true)
            .extended_data(extended_data);
        if let Some(order_by) = order_by {
            params = params.order_by(order_by);
        }
        if let Some(page_size) = page_size {
            params = params.page_size(page_size);
        }
        if let Some(page_number) = page_number {
            params = params.page_number(page_number);
        }
        self.get_sets(params).await
    }

    /// Get the user's owned sets. For additional filtering options, use [`Self::get_sets`].
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn get_owned_sets(&self, order_by: Option<request::OrderBy>, page_size: Option<usize>, page_number: Option<usize>, extended_data: bool) -> Result<response::GetSetsResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn)
        }
        let mut params = request::GetSetsParameters::new()
            .owned_by_user(true)
            .extended_data(extended_data);
        if let Some(order_by) = order_by {
            params = params.order_by(order_by);
        }
        if let Some(page_size) = page_size {
            params = params.page_size(page_size);
        }
        if let Some(page_number) = page_number {
            params = params.page_number(page_number);
        }
        self.get_sets(params).await
    }

    /// Get instructions for a particular set.
    pub async fn get_instructions(&self, set_id: u64) -> Result<response::GetInstructionsResponse> {
        let request = request::GetInstructions::new(self.api_key, set_id);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get instructions for a particular set.
    pub async fn get_instructions_2(&self, set_number: &str) -> Result<response::GetInstructionsResponse> {
        let request = request::GetInstructions2::new(self.api_key, set_number);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get reviews for a particular set.
    pub async fn get_reviews(&self, set_id: u64) -> Result<response::GetReviewsResponse> {
        let request = request::GetReviews::new(self.api_key, set_id);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get all themes, with the total number of sets in each theme.
    pub async fn get_themes(&self) -> Result<response::GetThemesResponse> {
        let request = request::GetThemes::new(self.api_key);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get subthemes for the given theme, with the total number of sets in each subtheme.
    pub async fn get_subthemes(&self, theme: &str) -> Result<response::GetSubthemesResponse> {
        let request = request::GetSubthemes::new(self.api_key, theme);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get a list of years for a given theme, with the total number of sets in each year.
    pub async fn get_years(&self, theme: &str) -> Result<response::GetYearsResponse> {
        let request = request::GetYears::new(self.api_key, theme);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Alter the user's collection. You may find these functions more convenient:
    /// - [`Self::set_wanted`]
    /// - [`Self::set_owned`]
    /// - [`Self::set_notes`]
    /// - [`Self::set_rating`]
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn set_collection<'s>(&self, set_id: u64, params: request::SetCollectionParameters<'s>) -> Result<response::SetCollectionResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn);
        }
        let request = request::SetCollection::new(self.api_key, self.user_hash.as_deref().unwrap(), set_id, params);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Add or remove a set from the user's wanted list.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_wanted(&self, set_id: u64, wanted: bool) -> Result<response::SetCollectionResponse> {
        let params = request::SetCollectionParameters::new()
            .wanted(wanted);
        self.set_collection(set_id, params).await
    }

    /// Add or remove a set from the user's owned list.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_owned(&self, set_id: u64, qty_owned: usize) -> Result<response::SetCollectionResponse> {
        let params = request::SetCollectionParameters::new()
            .owned(qty_owned);
        self.set_collection(set_id, params).await
    }

    /// Modify the user's notes for a set.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_notes(&self, set_id: u64, notes: &str) -> Result<response::SetCollectionResponse> {
        let params = request::SetCollectionParameters::new()
            .notes(notes);
        self.set_collection(set_id, params).await
    }

    /// Modify the user's rating of a set.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_rating(&self, set_id: u64, rating: i32) -> Result<response::SetCollectionResponse> {
        let params = request::SetCollectionParameters::new()
            .rating(rating);
        self.set_collection(set_id, params).await
    }

    /// Get the user's set notes.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn get_notes(&self) -> Result<response::GetUserNotesResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn);
        }
        let request = request::GetUserNotes::new(self.api_key, self.user_hash.as_deref().unwrap());
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get the user's minifig collection. You may find these functions more convenient:
    /// 
    /// - [`Self::get_owned_minifigs`]
    /// - [`Self::get_wanted_minifigs`]
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn get_minifig_collection<'s>(&self, params: GetMinifigCollectionParameters<'s>) -> Result<response::GetMinifigCollectionResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn);
        }
        let request = request::GetMinifigCollection::new(self.api_key, self.user_hash.as_deref().unwrap(), params);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Get a list of minifigs owned by the user. If not None, `query` is used to filter the
    /// results by name and ID.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn get_owned_minifigs(&self, query: Option<&str>) -> Result<response::GetMinifigCollectionResponse> {
        let mut params = request::GetMinifigCollectionParameters::owned();
        if let Some(query) = query {
            params = params.query(query);
        }
        self.get_minifig_collection(params).await
    }

    /// Get a list of minifigs owned by the user. If not None, `query` is used to filter the
    /// results by name and ID.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn get_wanted_minifigs(&self, query: Option<&str>) -> Result<response::GetMinifigCollectionResponse> {
        let mut params = request::GetMinifigCollectionParameters::wanted();
        if let Some(query) = query {
            params = params.query(query);
        }
        self.get_minifig_collection(params).await
    }

    /// Modify the user's minifig collection. You may find these functions more convenient:
    /// 
    /// - [`Self::set_minifig_owned`]
    /// - [`Self::set_minifig_wanted`]
    /// - [`Self::set_minifig_notes`]
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn set_minifig_collection<'s>(&self, minifig_number: &str, params: SetMinifigCollectionParameters<'s>) -> Result<response::SetMinifigCollectionResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn);
        }
        let request = request::SetMinifigCollection::new(self.api_key, self.user_hash.as_deref().unwrap(), minifig_number, params);
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    /// Add or remove a minifig from the user's owned list.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_minifig_owned(&self, minifig_number: &str, qty_owned: usize) -> Result<response::SetMinifigCollectionResponse> {
        let params = request::SetMinifigCollectionParameters::new()
            .owned(qty_owned);
        self.set_minifig_collection(minifig_number, params).await
    }

    /// Add or remove a minifig from the user's wanted list.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_minifig_wanted(&self, minifig_number: &str, wanted: bool) -> Result<response::SetMinifigCollectionResponse> {
        let params = request::SetMinifigCollectionParameters::new()
            .wanted(wanted);
        self.set_minifig_collection(minifig_number, params).await
    }

    /// Modify the user's notes for a minifig.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    #[inline]
    pub async fn set_minifig_notes(&self, minifig_number: &str, notes: &str) -> Result<response::SetMinifigCollectionResponse> {
        let params = request::SetMinifigCollectionParameters::new()
            .notes(notes);
        self.set_minifig_collection(minifig_number, params).await
    }

    /// Get the user's minifig notes.
    /// 
    /// If the [`ClientWrapper`] is not logged in, this function will return an error.
    pub async fn get_minifig_notes(&self) -> Result<response::GetMinifigUserNotesResponse> {
        if !self.is_logged_in() {
            return Err(Error::NotLoggedIn);
        }
        let request = request::GetUserMinifigNotes::new(self.api_key, self.user_hash.as_deref().unwrap());
        let response = self.execute(request).await?;
        Ok(RespResult::from(response)?)
    }

    async fn execute<E, T>(&self, request: E) -> Result<Response<T>>
    where
        T: serde::de::DeserializeOwned,
        E: BricksetRequest
    {
        #[cfg(feature = "log")]
        debug!("Executing Brickset API request: {}", request.method_name());

        let request = request.to_reqwest(&self.client)?;

        let response = self.client.execute(request).await?;

        if !response.status().is_success() {
            return Err(Error::Http { response })
        }

        let text = response.text().await?;

        Ok(serde_json::from_str(&text)?)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Error {
        Error::Reqwest(value)
    }
}

impl From<request::Error> for Error {
    fn from(value: request::Error) -> Error {
        Error::Request(value)
    }
}

impl From<response::Error> for Error {
    fn from(value: response::Error) -> Error {
        Error::Response(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Error {
        Error::Json(value)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(e) => e.fmt(f),
            Error::Request(e) => e.fmt(f),
            Error::Response(e) => e.fmt(f),
            Error::Json(e) => e.fmt(f),
            Error::Http { response } => write!(f, "HTTP request failed with status code {}", response.status()),
            Error::NotLoggedIn => write!(f, "Not logged in")
        }
    }
}

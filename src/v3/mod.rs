//! Implementation of the Brickset V3 API.

use serde::{Deserialize, Serialize};

pub mod response;
pub mod request;
pub(crate) mod util;

#[cfg(feature = "reqwest")]
pub mod reqwest_api;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "status")]
pub enum Response<T> {
    #[serde(rename = "success")]
    Ok(T),
    #[serde(rename = "error")]
    Err(response::Error),
}

impl<T> Response<T> {
    #[inline]
    pub fn get(&self) -> Option<&T> {
        match self {
            Response::Ok(ok) => Some(ok),
            Response::Err(_) => None,
        }
    }

    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        match self {
            Response::Ok(ok) => Some(ok),
            Response::Err(_) => None,
        }
    }

    #[inline]
    pub fn get_err(&self) -> Option<&response::Error> {
        match self {
            Response::Ok(_) => None,
            Response::Err(err) => Some(err),
        }
    }

    #[inline]
    pub fn get_err_mut(&mut self) -> Option<&mut response::Error> {
        match self {
            Response::Ok(_) => None,
            Response::Err(err) => Some(err),
        }
    }

    #[inline]
    pub fn unwrap(self) -> T {
        match self {
            Response::Ok(ok) => ok,
            Response::Err(err) => panic!("unwrap on Response::Err({err:?})"),
        }
    }
}

impl<T: std::fmt::Debug> Response<T> {
    #[inline]
    pub fn unwrap_err(self) -> response::Error {
        match self {
            Response::Ok(ok) => panic!("unwrap on Response::Ok({ok:?})"),
            Response::Err(err) => err,
        }
    }
}

impl<T> From<Response<T>> for std::result::Result<T,response::Error> {
    fn from(value: Response<T>) -> Self {
        match value {
            Response::Ok(ok) => Ok(ok),
            Response::Err(err) => Err(err),
        }
    }
}


#[cfg(test)]
mod response_tests {
    use super::response::*;
    use super::Response;
    use serde::Deserialize;
    use serde_json::Deserializer;

    #[test]
    fn check_key_success() {
        let input = r#" {"status":"success"} "#;
        let mut des = Deserializer::from_str(input);
        let resp = Response::<CheckKeyResponse>::deserialize(&mut des).expect("deserialize");
        assert!(matches!(resp, Response::Ok(_)));
    }

    #[test]
    fn check_key_error() {
        let input = r#" {"status":"error", "message":"Invalid API key"} "#;
        let mut des = Deserializer::from_str(input);
        let resp = Response::<CheckKeyResponse>::deserialize(&mut des).expect("deserialize");
        assert!(matches!(resp, Response::Err(_)));
        let err = resp.unwrap_err();
        assert_eq!(err.message, "Invalid API key");
    }
}

#[cfg(test)]
mod request_tests {
    use super::request::*;

    #[test]
    fn check_key() {
        let input = CheckKey::new("12345678");
        let url = input.to_request_url().expect("try_from");
        assert_eq!(url.as_str(), "https://brickset.com/api/v3.asmx/checkKey?apiKey=12345678")
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Flag;

impl Serialize for Flag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        1.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Flag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let i = i32::deserialize(deserializer)?;
        if i == 1 {
            Ok(Flag)
        } else {
            Err(D::Error::custom(format!("flag must be 1, was {i}")))
        }
    }
}

/// Serializes a valid "updatedSince" field for the getSets method
pub(crate) mod updated_since_format {
    use chrono::prelude::*;
    use serde::{self, Deserialize, Serializer, Deserializer, Serialize};

    const FMT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        date.unwrap().format(FMT).to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>
    {
        use serde::de::Error;

        let text = String::deserialize(deserializer)?;
        match NaiveDate::parse_from_str(&text, FMT) {
            Ok(date) => Ok(Some(date)),
            Err(err) => Err(D::Error::custom(format!("{err}"))),
        }
    }
}

/// Converts a string value to an `Option<String>`, mapping the value `"{Not specified}"
/// to `None`.
pub(crate) mod not_specified_optional_string {
    use serde::{self, Deserialize, Serializer, Deserializer, Serialize};

    pub fn serialize<S>(s: &Option<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match s {
            None => "{Not specified}".serialize(serializer),
            Some(s) => s.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(s) => if s == "{Not specified}" {
                Ok(None)
            } else {
                Ok(Some(s))
            },
            None => Ok(None),
        }
    }
}

/// Converts a [`Vec<i32>`] to a comma-delimited string of numbers, and vice versa.
pub(crate) mod int_vec_as_commastr {
    use serde::{self, Deserialize, Serializer, Deserializer, Serialize};

    pub fn serialize<S>(years: &Vec<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        use itertools::Itertools;

        format!("{}", years.iter().format(", ")).serialize(serializer)
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i32)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: Deserializer<'de>
    {
        use serde::de::Error;

        let mut result = vec![];

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(str) => {
                for i in str.split(",").map(|s| i32::from_str_radix(s.trim(), 10)) {
                    match i {
                        Ok(i) => result.push(i),
                        Err(err) => return Err(D::Error::custom(format!("{err}")))
                    }
                }
            },
            StringOrInt::Int(int) => {
                result.push(int)
            }
        }

        Ok(result)
    }
}

/// Deserializes a nullable `i32` normally, except zero is mapped to None.
pub(crate) mod zero_none {
    use serde::{self, Deserialize, Serializer, Deserializer, Serialize};

    pub fn serialize<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match value {
            None => 0.serialize(serializer),
            Some(val) => val.serialize(serializer),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = i32::deserialize(deserializer)?;

        if value == 0 {
            Ok(None)
        } else {
            Ok(Some(value))
        }
    }
}

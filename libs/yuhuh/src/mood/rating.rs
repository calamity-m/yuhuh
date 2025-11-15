use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};
use utoipa::PartialSchema;

use crate::error::RatingError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rating(u32);

impl Rating {
    pub fn new(value: u32) -> Option<Rating> {
        if value <= 10 { Some(Self(value)) } else { None }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<i16> for Rating {
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        match value {
            0..=10 => Ok(Self(value as u32)),
            11.. => Err(RatingError::new(format!(
                "must 0 and 10, but got {} instead",
                value
            ))),
            _ => Err(RatingError::new(format!(
                "ratings cannot be negative, but got {} instead",
                value
            ))),
        }
    }

    type Error = RatingError;
}

impl TryFrom<i64> for Rating {
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0..=10 => Ok(Self(value as u32)),
            11.. => Err(RatingError::new(format!(
                "must 0 and 10, but got {} instead",
                value
            ))),
            _ => Err(RatingError::new(format!(
                "ratings cannot be negative, but got {} instead",
                value
            ))),
        }
    }

    type Error = RatingError;
}

impl TryFrom<u32> for Rating {
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0..=10 => Ok(Self(value)),
            _ => Err(RatingError::new(format!(
                "must 0 and 10, but got {} instead",
                value
            ))),
        }
    }

    type Error = RatingError;
}

impl TryFrom<u64> for Rating {
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0..=10 => Ok(Self(value as u32)),
            _ => Err(RatingError::new(format!(
                "must 0 and 10, but got {} instead",
                value
            ))),
        }
    }

    type Error = RatingError;
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RatingVisitor;

        impl<'de> Visitor<'de> for RatingVisitor {
            type Value = Rating;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an i16 or u32 integer between 0 and 10")
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Rating::try_from(value).map_err(serde::de::Error::custom)
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Rating::try_from(value).map_err(serde::de::Error::custom)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Rating::try_from(value).map_err(serde::de::Error::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Rating::try_from(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(RatingVisitor)
    }
}

// Serialize as u32
impl Serialize for Rating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

// openapi schema
impl PartialSchema for Rating {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::Type(
                utoipa::openapi::schema::Type::Number,
            ))
            .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                utoipa::openapi::KnownFormat::Int32,
            )))
            .minimum(Some(0))
            .maximum(Some(10))
            .description(Some("A rating value"))
            .examples(vec![0, 1, 5, 7, 10])
            .into()
    }
}

impl utoipa::ToSchema for Rating {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("Rating")
    }
}

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub year: u32,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.year == other.year
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Document {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub event: ObjectId,
    pub url: String,
    pub notified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordGuild {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "string")]
    pub discord_id: u64,
    pub name: String,
    #[serde(with = "u64_option", skip_serializing_if = "Option::is_none")]
    pub channel: Option<u64>,
    #[serde(with = "u64_option", skip_serializing_if = "Option::is_none")]
    pub role: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thread {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub guild: ObjectId,
    pub event: ObjectId,
    #[serde(with = "u64")]
    pub discord_id: u64,
}

mod u64 {

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(*value as i64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i64::deserialize(deserializer) {
            Ok(val) => Ok(val as u64),
            Err(why) => Err(why),
        }
    }
}

mod u64_option {

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(value.unwrap_or_default() as i64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match i64::deserialize(deserializer) {
            Ok(val) => Ok(Some(val as u64)),
            Err(why) => Err(why),
        }
    }
}

mod string {
    use std::{fmt::Display, str::FromStr};

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

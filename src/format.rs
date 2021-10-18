pub mod datetime {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y/%m/%d,%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Utc.datetime_from_str(&String::deserialize(deserializer)?, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod date {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%d %B %Y";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        NaiveDate::parse_from_str(&String::deserialize(deserializer)?, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

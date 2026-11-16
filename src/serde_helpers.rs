use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, de::Error};

pub(crate) fn date<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error> {
    match Option::<String>::deserialize(deserializer)? {
        Some(text) if !text.is_empty() => NaiveDate::parse_from_str(&text, "%Y-%m-%d")
            .map(Some)
            .map_err(D::Error::custom),
        _ => Ok(None),
    }
}

pub(crate) fn string_or_number<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        Number(u64),
        Text(String),
    }

    Ok(match Id::deserialize(deserializer)? {
        Id::Number(number) => number.to_string(),
        Id::Text(text) => text,
    })
}

pub(crate) fn comma_list<T: Serialize>(items: &[T]) -> String {
    let names: Vec<String> = items.iter().filter_map(name).collect();
    names.join(",")
}

fn name<T: Serialize>(item: &T) -> Option<String> {
    serde_json::to_value(item).ok()?.as_str().map(str::to_owned)
}

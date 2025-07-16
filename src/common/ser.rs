pub mod i64_to_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        i64::from_str(&s).map_err(serde::de::Error::custom)
    }
}

pub mod option_string_or_i64 {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        s.map(|s| i64::from_str(&s).map_err(serde::de::Error::custom))
            .transpose()
    }
}

pub mod veci64_to_vecstring {
    use serde::{self, Deserialize, Deserializer, Serializer};
    #[warn(clippy::ptr_arg)]
    pub fn serialize<S>(values: &[i64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = values.iter().map(|v| v.to_string()).collect::<Vec<_>>();
        serializer.collect_seq(serialized)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Vec<String> = Vec::deserialize(deserializer)?;
        s.into_iter()
            .map(|item| item.parse().map_err(serde::de::Error::custom))
            .collect()
    }
}

pub mod option_veci64_to_vecstring {
    use serde::{self, Deserialize, Deserializer, Serializer};
    #[warn(clippy::ptr_arg)]
    pub fn serialize<S>(values: &Option<Vec<i64>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match values {
            Some(val) => {
                let serialized = val.iter().map(|v| v.to_string()).collect::<Vec<_>>();
                serializer.collect_seq(serialized)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
    where
        D: Deserializer<'de>,
    { 
        let s: Option<Vec<String>> = Option::<Vec<String>>::deserialize(deserializer)?;
        match s {
            Some(strings) => {
                let mut int_vec = Vec::with_capacity(strings.len());
                for string in strings {
                    if let Ok(num) = string.parse::<i64>() {
                        int_vec.push(num);
                    } else {
                        return Err(serde::de::Error::custom(format!("Failed to parse string to i64: {}", string)));
                    }
                }
                Ok(Some(int_vec))
            }
            None => Ok(None),
        }
    }
}

pub mod string_to_bool {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            "0" => serializer.serialize_bool(false),
            _ => serializer.serialize_bool(true),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = bool::deserialize(deserializer)?;
        if s {
            Ok("1".to_string())
        } else {
            Ok("0".to_string())
        }
    }
}

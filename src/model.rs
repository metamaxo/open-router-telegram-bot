use serde::{Deserialize, Serialize};

/// Model enum
/// Capture the different models that can be used
/// The default model is OpenAi
#[derive(Default, Clone, Copy, Debug)]
pub enum Model {
    Weaver,
    Unslopnemo,
    Gemini,
    Deepseek,
    Claude,
    Llama,
    #[default]
    OpenAi,
}

/// Implement Display for Model
/// This gives the `Model` enum the `to_string` and `as_str` methods
/// Also allows for the `format!` macro to be used
impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", <&str>::from(*self))
    }
}

/// Implement From<Model> for &str
/// This allows for us to cast a `Model` to a `&str`
impl From<Model> for &str {
    fn from(model: Model) -> Self {
        match model {
            Model::Unslopnemo => "thedrummer/unslopnemo-12b",
            Model::Gemini => "google/gemini-2.0-flash-001",
            Model::Deepseek => "deepseek/deepseek-r1-distill-llama-8b",
            Model::Claude => "anthropic/claude-3.5-sonnet",
            Model::Llama => "sao10k/13.1-70b-hanami-x1",
            Model::OpenAi => "openai/gpt-4o",
            Model::Weaver => "mancer/weaver",
        }
    }
}

/// Implement From<&str> for Model
/// This allows for us to cast a `&str` to a `Model`
impl TryFrom<&str> for Model {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            _ if value.contains("weaver") => Self::Weaver,
            _ if value.contains("unslopnemo") => Self::Unslopnemo,
            _ if value.contains("gemini") => Self::Gemini,
            _ if value.contains("deepseek") => Self::Deepseek,
            _ if value.contains("claude") => Self::Claude,
            _ if value.contains("llama") => Self::Llama,
            _ if value.contains("openai") => Self::OpenAi,
            _ if value.contains("gpt") => Self::OpenAi,
            _ => return Err(()),
        })
    }
}

/// Custom serialization implementation to use the Display format
impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(<&str>::from(*self))
    }
}

/// Custom deserialization implementation
impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Model::try_from(s.as_str()).map_err(|_| {
            serde::de::Error::custom(format!("unknown model type: {}", s))
        })
    }
}

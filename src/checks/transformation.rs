use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use svalue::SValue;
use utoipa::ToSchema;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "action")]
pub enum Transformation {
    TrimLeftRight,
    ToUppercase,
    ExtractSingleNumber,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TransformationError {
    #[error("output doesn't contain mandatory number")]
    DidntFindNumber,
    #[error("output contains multiple numbers, choice is ambiguous")]
    AmbiguousChoiceForNumber,
}

static NUMBER_REGEX: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new("[+-]?[0-9]+([.][0-9]+)?").unwrap());

impl Transformation {
    pub(crate) fn transform(&self, value: SValue) -> Result<SValue, TransformationError> {
        Ok(match self {
            Self::ToUppercase => SValue::Text(value.to_string().to_uppercase().into()),
            Self::TrimLeftRight => SValue::Text(value.to_string().trim().into()),
            Self::ExtractSingleNumber => match value {
                SValue::Int(i) => SValue::Int(i),
                SValue::Float(f) => SValue::Float(f),
                SValue::Bool(true) => SValue::Int(1),
                SValue::Bool(false) => SValue::Int(0),
                SValue::Text(text) => {
                    let mut iter = NUMBER_REGEX.find_iter(&text);
                    if let Some(num) = iter.next() {
                        if iter.next().is_some() {
                            return Err(TransformationError::AmbiguousChoiceForNumber);
                        } else {
                            num.as_str().to_string().into()
                        }
                    } else {
                        return Err(TransformationError::DidntFindNumber);
                    }
                }
            },
        })
    }
}

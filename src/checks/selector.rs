use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Selector {
    #[schema(title = "last-line")]
    LastLine,
    #[schema(title = "first-line")]
    FirstLine,
    #[schema(title = "nth-line-from-start")]
    /// First line is `n = 0`, second `n = 1`, ...
    NthLineFromStart { n: u16 },
    #[schema(title = "nth-line-from-end")]
    /// Last line is `n = 0`, second to last `n = 1`, ...
    NthLineFromEnd { n: u16 },
}

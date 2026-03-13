use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Selector {
    LastLine,
    FirstLine,
    /// First line is n = 0, second n = 1, ...
    NthLineFromStart {
        n: u16,
    },
    /// Last line is n = 0, second to last n = 1, ...
    NthLineFromEnd {
        n: u16,
    },
}

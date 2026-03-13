use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum CaseCheckSeverity {
    /// if this check fails, only an info is shown that a nice-to-have criteria is not fulfilled
    NiceToHave,
    /// if this check fails, a warning is produced
    Warning,
    /// if this check fails, an error is produced
    Error,
}

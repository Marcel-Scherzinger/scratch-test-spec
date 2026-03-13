mod checks;
mod conditions;
mod helper;
mod lints;
mod parts;

pub(crate) use helper::impl_modifiers;

pub use conditions::ExplainableFailure;

pub mod error {
    pub use crate::checks::{criterion::CriterionError, transformation::TransformationError};
    pub use crate::conditions::ConditionError;
    pub use crate::parts::specification::SpecError;
}
pub mod report {
    pub use crate::checks::case_check::CheckReport;
    pub use crate::lints::LintReport;
    pub use crate::parts::category::CategoryReport;
}
pub mod spec {
    pub use crate::checks::{
        CaseCheckSeverity, Check, Criterion, Selector, SingleCaseCheckCondition, TestCaseCheck,
        Transformation,
    };
    pub use crate::conditions::{CompoundCheckCondition, Condition};
    pub use crate::lints::{Lint, LintCondition};
    pub use crate::parts::{StaticTestCategory, TestCase, TestCategory, TestSpecification};
}

use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, ToSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckResultMessages {
    human_msg: Option<String>,
    tools_msg: Option<String>,
    help_url: Option<String>,
}

crate::impl_modifiers!(CheckResultMessages {
    human_msg: {optinto} String,
    tools_msg: {optinto} String,
    help_url: {optinto} String,
});

impl CheckResultMessages {
    pub fn new() -> Self {
        Self {
            human_msg: None,
            tools_msg: None,
            help_url: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
pub struct RandomsCfg {
    generate: RandomsGenerate,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "status")]
pub enum RandomsGenerate {
    /// no program is allowed to request random numbers, doing so will lead to
    /// termination and an error
    Deny,

    /// random numbers are generated based on an optional seed value,
    /// the same library version on the same target will produce the same sequence
    /// for the same seed but there are no further guarantees
    #[serde(rename_all = "kebab-case")]
    Allow { seed: Option<u64> },
}

#[cfg(test)]
mod test_output;

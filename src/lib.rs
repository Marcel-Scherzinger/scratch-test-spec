#![allow(unused)]
mod checks;
mod helper;
mod parts;

pub(crate) use helper::impl_modifiers;

pub mod error {
    pub use crate::checks::{
        condition::ConditionError, criterion::CriterionError, transformation::TransformationError,
    };
    pub use crate::parts::specification::SpecError;
}
pub mod report {
    pub use crate::checks::case_check::CheckReport;
    pub use crate::parts::category::CategoryReport;
}
pub mod spec {
    pub use crate::checks::{
        CaseCheckSeverity, Check, CompoundCheckCondition, Criterion, Selector,
        SingleCheckCondition, TestCaseCheck, TestCaseCheckCondition, Transformation,
    };
    pub use crate::parts::{StaticTestCategory, TestCase, TestCategory, TestSpecification};
}

use std::num::NonZeroUsize;

use derive_getters::Getters;
use derive_more::From;
use schemars::{JsonSchema, transform};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct CheckResultMessages {
    human_msg: Option<String>,
    tools_msg: Option<String>,
    help_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
pub struct RandomsCfg {
    generate: RandomsGenerate,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

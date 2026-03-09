#![allow(unused)]

use std::num::NonZeroUsize;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TestSpecification {
    /// list of test categories
    categories: Vec<TestCategory>,
    /// Ignored but preserved by application, can be used to add extra information
    misc: Option<serde_json::Value>,
    /// Optional solution file that can be used to check the test specification
    solution: Option<serde_json::Value>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum TestCategory {
    Static(StaticTestCategory),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StaticTestCategory {
    /// optional description about kind of contained tests and tested criteria
    description: Option<String>,
    randoms: Option<RandomsCfg>,
    cases: Vec<TestCase>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TestCase {
    inputs: Vec<String>,
    // randoms: Option<RandomsCfg>,
    checks: Vec<TestCaseCheck>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TestCaseCheck {
    severity: TestCaseSeverity,
    on_success: Option<CheckResultMessages>,
    on_failure: Option<CheckResultMessages>,
    condition: TestCaseCheckCondition,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum TestCaseCheckCondition {
    Compound(CompoundCheckCondition),
    Single(SingleCheckCondition),
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CompoundCheckCondition {
    All(Vec<TestCaseCheckCondition>),
    Any(Vec<TestCaseCheckCondition>),
    Not(Box<TestCaseCheckCondition>),
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SingleCheckCondition {
    select: Selector,
    transformations: Vec<Transformation>,
    criterion: Criterion,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Criterion {
    EqualTexts {
        other: String,
    },
    #[serde(rename_all = "kebab-case")]
    EqualNumbers {
        other: serde_json::Number,
        float_tolerance: Option<f64>,
    },
    ContainedIn {
        text: String,
    },
    Contains {
        text: String,
    },
    OneOf {
        options: Vec<String>,
    },
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Selector {
    LastLine,
    FirstLine,
    /// First line is n = 1, second n = 2, ..., n = 0 is invalid
    NthLineFromStart {
        n: NonZeroUsize,
    },
    /// Last line is n = 1, second to last n = 2, ..., n = 0 is invalid
    NthLineFromEnd {
        n: NonZeroUsize,
    },
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "action")]
pub enum Transformation {
    TrimLeftRight,
    ToUppercase,
    ExtractSingleNumber,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum TestCaseSeverity {
    /// if this check fails, only an info is shown that a nice-to-have criteria is not fulfilled
    NiceToHave,
    /// if this check fails, a warning is produced
    Warning,
    /// if this check fails, an error is produced
    Error,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckResultMessages {
    human_msg: Option<String>,
    tools_msg: Option<String>,
    help_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
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

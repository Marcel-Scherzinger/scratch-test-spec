use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smodel::{ProjectDoc, blocks::BlockKindUnit};
use utoipa::ToSchema;

use crate::parts::CheckResultMessages;
use crate::{
    conditions::{AnySingleCondition, ExplainableFailure},
    error::ConditionError,
    spec::Condition,
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub struct Lint {
    severity: LintSeverity,
    condition: Condition<LintCondition>,
    on_success: Option<CheckResultMessages>,
    on_failure: Option<CheckResultMessages>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, Copy, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum LintSeverity {
    /// if this check fails, only an info is shown that a nice-to-have criteria is not fulfilled
    NiceToHave,
    /// if this check fails, a warning is produced
    Warning,
    /// if this check fails, an error is produced
    Error,
}

#[derive(Debug, PartialEq)]
pub struct LintReport<'s> {
    severity: LintSeverity,
    failure: Option<ConditionError<'s, LintCondition>>,
    message: Option<&'s CheckResultMessages>,
}
impl<'s> LintReport<'s> {
    pub fn severity(&self) -> &LintSeverity {
        &self.severity
    }
    pub fn failure(&self) -> Option<&ConditionError<'s, LintCondition>> {
        self.failure.as_ref()
    }
    pub fn message(&self) -> Option<&CheckResultMessages> {
        self.message
    }
}

crate::impl_modifiers!(Lint {
    on_failure: Option<CheckResultMessages>,
    on_success: Option<CheckResultMessages>,
    severity: LintSeverity,
});

impl Lint {
    pub fn block_count_limit(opcode: impl Into<BlockKindUnit>, max: u16) -> Self {
        Self {
            severity: LintSeverity::Warning,
            condition: LintCondition::BlockCountLimit {
                opcode: opcode.into(),
                max,
            }
            .into(),
            on_failure: None,
            on_success: None,
        }
    }

    pub fn run_on<'s>(&'s self, doc: &ProjectDoc) -> LintReport<'s> {
        let result = self.condition.check(doc);

        let message = if result.is_ok() {
            self.on_success.as_ref()
        } else {
            self.on_failure.as_ref()
        };
        LintReport {
            severity: self.severity,
            failure: result.err(),
            message,
        }
    }

    pub fn make_error(mut self) -> Self {
        self.severity = LintSeverity::Error;
        self
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum LintCondition {
    BlockCountLimit { opcode: BlockKindUnit, max: u16 },
}
impl LintCondition {
    pub fn make_lint(self) -> Lint {
        Lint {
            condition: Condition::Single(self),
            on_success: None,
            on_failure: None,
            severity: LintSeverity::Warning,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LintConditionError {
    ExceededAllowedBlockCount {
        opcode: BlockKindUnit,
        max: u16,
        used: usize,
    },
}

impl ExplainableFailure for LintConditionError {
    fn explain(&self) -> String {
        match self {
            Self::ExceededAllowedBlockCount { opcode, max, used } => format!(
                "program uses more instances of {opcode} blocks than allowed ({used} > {max})"
            ),
        }
    }
}

impl AnySingleCondition for LintCondition {
    type Input<'i> = &'i ProjectDoc;
    type Error<'s>
        = LintConditionError
    where
        Self: 's;

    fn check<'s>(&'s self, doc: Self::Input<'_>) -> Result<(), Self::Error<'s>>
    where
        Self: Sized,
    {
        match self {
            LintCondition::BlockCountLimit { opcode, max } => {
                let used_count = doc
                    .su_ids_with_blocks()
                    .map(|(_, o)| o)
                    .filter(|o| o == opcode)
                    .count();
                if used_count > (*max as usize) {
                    Err(LintConditionError::ExceededAllowedBlockCount {
                        opcode: *opcode,
                        max: *max,
                        used: used_count,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

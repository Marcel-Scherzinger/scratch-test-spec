use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{Report, default_state::DefaultState};
use utoipa::ToSchema;

use super::CaseCheckSeverity;

use crate::parts::{CheckResultMessages, RandomsCfg};
use crate::{error::ConditionError, spec::Condition, spec::SingleCaseCheckCondition};

#[derive(Debug, PartialEq)]
pub struct CheckReport<'s> {
    severity: CaseCheckSeverity,
    failure: Option<ConditionError<'s, SingleCaseCheckCondition>>,
    message: Option<&'s CheckResultMessages>,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TestCaseCheck {
    severity: CaseCheckSeverity,
    on_success: Option<CheckResultMessages>,
    on_failure: Option<CheckResultMessages>,
    condition: Condition<SingleCaseCheckCondition>,
}

impl TestCaseCheck {
    pub(crate) fn run_on<'a>(
        &'a self,
        randoms: Option<&RandomsCfg>,
        report: &Report<'_, DefaultState>,
    ) -> CheckReport<'a> {
        let result = self.condition.check((randoms, report));

        let message = if result.is_ok() {
            self.on_success.as_ref()
        } else {
            self.on_failure.as_ref()
        };
        CheckReport {
            severity: self.severity,
            failure: result.err(),
            message,
        }
    }

    pub fn new_error(condition: impl Into<Condition<SingleCaseCheckCondition>>) -> Self {
        Self::new(CaseCheckSeverity::Error, condition)
    }
    pub fn new_warning(condition: impl Into<Condition<SingleCaseCheckCondition>>) -> Self {
        Self::new(CaseCheckSeverity::Warning, condition)
    }
    pub fn new_nice_to_have(condition: impl Into<Condition<SingleCaseCheckCondition>>) -> Self {
        Self::new(CaseCheckSeverity::NiceToHave, condition)
    }

    pub fn new(
        severity: CaseCheckSeverity,
        condition: impl Into<Condition<SingleCaseCheckCondition>>,
    ) -> Self {
        Self {
            severity,
            on_success: None,
            on_failure: None,
            condition: condition.into(),
        }
    }
}

impl<'a> CheckReport<'a> {
    pub fn success(&self) -> bool {
        self.failure.is_none()
    }
    pub fn failure(&self) -> Option<&ConditionError<'a, SingleCaseCheckCondition>> {
        self.failure.as_ref()
    }
    pub fn severity(&self) -> &CaseCheckSeverity {
        &self.severity
    }
    pub fn message(&self) -> Option<&'a CheckResultMessages> {
        self.message
    }
}

crate::helper::impl_modifiers!(TestCaseCheck {
    severity: CaseCheckSeverity,
    on_success: Option<CheckResultMessages>,
    on_failure: Option<CheckResultMessages>,
    condition: {into} Condition<SingleCaseCheckCondition>
});

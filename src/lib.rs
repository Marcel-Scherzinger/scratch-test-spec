mod checks;
mod conditions;
mod helper;
mod lints;
mod parts;

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
pub use conditions::ExplainableFailure;
pub(crate) use helper::impl_modifiers;

use parts::Number;

#[cfg(test)]
mod test_output;

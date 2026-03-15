//! # Abstract
//!
//! This is part of the `scratch-test` project and specifies how tests are written
//! down. This includes linting and especially testing.
//!
//! Test specifications can be serialized to JSON with [`serde_json`].
//!
//! Every specification ([`TestSpecification`]) consists of tests which are grouped
//! in categories and lints that typically apply globally/statically, depending on
//! the lint at hand.
//!
//! When executed on a program the different parts of a specification generate reports.
//!
//! # Categories
//!
//! Categories can be one of a list of different types. By now there are only
//! [static categories](parts::StaticTestCategory) but the format is open for
//! future additions as the type still needs to be specified.
//!
//! ## Static test categories
//!
//! A static category has three properties:
//!
//! - An optional `"description"` of type `String`
//! - An optional configuration deciding if [`"randoms"`](parts::RandomsCfg) are enabled
//! - A list of tests ([`TestCase`]) belonging to this category, all of those
//!   tests will be executed with the configured random numbers policy.
//!   If some of them fail this _may_ be displayed as a partially failed category
//!
//! Categories should group similar tests together, test cases with similar properties
//! like negative numbers or special edge cases.
//! The description can hint about the kind of test inputs.
//!
//! # Test cases
//!
//! A test case is a list of `"inputs"` and `"checks"`.
//! Those inputs will be provided to the program whenever it asks "questions".
//! This is the Scratch version of reading from `stdin`.
//! The test case will run the program with the inputs and collect all of
//! its interesting actions, typically output.
//! Afterwards, all checks are executed having the output available.
//!
//! ```
//! # use scratch_test_spec::TestCase;
//! let case = TestCase::new(vec!["First Input", "Second Input"]);
//! ```
//!
//! This case has no checks, you can add them with [`TestCase::and_check`] like this:
//!
//! ```
//! # use scratch_test_spec::{TestCase, Check};
//! let case = TestCase::new(vec!["First Input", "Second Input"])
//!     .and_check(Check::last_line().c_equal_texts("Expected Output").make_error())
//!     .and_check(Check::first_line().c_equal_texts("Expected first line").make_error());
//! ```
//!
//! # Conditions
//!
//! The condition system allows to combine multiple atomic conditions with logical
//! connectives. Currently, there are two kinds of atomic conditions, one for test
//! case checks and one for lints.
//!
//! Logical connectives are represented by [`CompoundCondition`](conditions::CompoundCondition).
//! As convenience shortcuts the crate also provides the macros [`all_conditions_of`],
//! [`any_condition_of`] and [`not_condition`] to create bigger constructs.
//!
//! ```
//! # use scratch_test_spec::{*, spec::*};
//! let condition: CompoundCondition<SingleCaseCheckCondition> = all_conditions_of![
//!     Check::last_line()
//!         .t_to_uppercase()
//!         .t_trim_left_right()
//!         .c_equal_texts("0"),
//!     Check::nth_line_from_end(3)
//!         .t_extract_single_number()
//!         .c_tolerance_equal_numbers(3, 1e-9),
//! ];
//! // Convert condition into a check with severity error
//! let check = condition.make_error();
//! ```
//!
//! # [Check]s
//!
//! A check has a condition which can be either atomic or compound.
//! Depending on the success/failure of the condition other properties get relevant:
//!
//! - `on_success` is an optional [message collection](parts::ResultMessages)
//!   that should be emitted if the condition was satisfied
//! - `on_failure` is an optional [message collection](parts::ResultMessages)
//!   that should be emitted if the condition was **not** satisfied
//! - `severity` decides how important the check was, so if a warning or an error
//!   should be produced if the condition is not satisfied
//!
//! An [atomic condition for checks](checks::SingleCaseCheckCondition)
//! consists of a [selector](checks::Selector) that decides which part of e. g.
//! the output should be considered and multiple [transformations](checks::Transformation)
//! that are executed in order. Each of them gets an input and produces an output.
//! The first one gets the selected value, every following transformation
//! receives the output of the last one and the last output is passed to
//! the [criterion](checks::Criterion).
//! The criterion is a comparison or other boolean function that can hold for
//! a value -- or not.
//!
//! A simple way to construct atomic conditions, and if needed checks of only those,
//! is the [`Check`] type. See it for details.
//!
//! # Example
//!
//! You can use the Rust code to generate the JSON format:
//!
//! ```
//! # use scratch_test_spec::*;
//! let spec = TestSpecification::new(vec![
//!     StaticTestCategory::new(vec![
//!         TestCase::new(vec!["1", "10"]).and_check(
//!             Check::last_line()
//!                 .t_extract_single_number()
//!                 .c_equal_texts("45")
//!                 .make_error()
//!                 .with_on_failure(Some(ResultMessages::new().with_human_msg(Some(
//!                     "The last line should contain only one number, namely 45",
//!                 )))),
//!         ),
//!     ])
//!     .with_description(Some("Description of category")),
//! ]);
//!
//! let generated_json = serde_json::to_value(&spec).expect("Use in practice better error handling");
//! let expected_json = serde_json::json!({
//!     "categories": [
//!         {
//!             "description": "Description of category",
//!             "type": "static",
//!             "cases": [
//!                 {
//!                     "inputs": ["1", "10"],
//!                     "checks": [
//!                         {
//!                             "severity": "error",
//!                             "condition": {
//!                                 "select": { "type": "last-line" },
//!                                 "transformations": [
//!                                     { "action": "extract-single-number" }
//!                                 ],
//!                                 "criterion": {
//!                                     "type": "equal-texts",
//!                                     "other": "45"
//!                                 }
//!                             },
//!                             "on-failure": {
//!                                 "human-msg": "The last line should contain only one number, namely 45"
//!                             }
//!                         }
//!                     ]
//!                 }
//!             ]
//!         }
//!     ],
//!     "lints": []
//! });
//!
//! assert_eq!(expected_json, generated_json);
//! ```
//!

mod checks;
mod conditions;
mod helper;
mod lints;
mod parts;

pub use spec::{Check, ResultMessages, StaticTestCategory, TestCase, TestSpecification};

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
        Transformation, builder::CheckConditionBuilder,
    };
    pub use crate::conditions::{CompoundCondition, Condition};
    pub use crate::lints::{Lint, LintCondition};
    pub use crate::parts::{RandomsCfg, RandomsGenerate, ResultMessages};
    pub use crate::parts::{StaticTestCategory, TestCase, TestCategory, TestSpecification};
}
pub use conditions::ExplainableFailure;
pub(crate) use helper::impl_modifiers;

use parts::Number;

#[cfg(test)]
mod test_output;

#[macro_export]
macro_rules! all_conditions_of {
    ($($c: expr),* $(,)?) => {
        $crate::spec::CompoundCondition::All(vec![$($c.into()),*])
    };
}
#[macro_export]
macro_rules! any_condition_of {
    ($($c: expr),* $(,)?) => {
        $crate::spec::CompoundCondition::Any(vec![$($c.into()),*])
    };
}
#[macro_export]
macro_rules! not_condition {
    ($c: expr) => {
        $crate::spec::CompoundCondition::Not($c.into())
    };
}

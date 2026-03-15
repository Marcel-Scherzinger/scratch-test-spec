use crate::{
    parts::Number,
    spec::{CaseCheckSeverity, Condition, TestCaseCheck},
};

use super::{Criterion, Selector, SingleCaseCheckCondition, Transformation};

/// Constructs an atomic check condition
///
/// The procedure is always: select, transform, criterion
///
/// You can create a new `Check` with [`Check::new`] where you have to
/// specify what should be selected.
/// Additionally, there are constructors for common selectors:
///
/// - [`Check::last_line`]
/// - [`Check::first_line`]
/// - [`Check::nth_line_from_end`]
/// - [`Check::nth_line_from_start`]
///
/// Aftwerwards, any number of transformations can be applied (even none).
/// To add a [`Transformation`] [`Check::transform`] can be used or one
/// of the helper methods (all start with `t_`):
///
/// - [`Check::t_to_uppercase`]
/// - [`Check::t_trim_left_right`]
/// - [`Check::t_extract_single_number`]
///
/// Finally, you have to choose one [`Criterion`] with [`Check::criterion`]
/// or one of the helper methods that will convert the `Check` instance
/// to another type (all start with `c_`).
///
/// - [`Check::c_equal_texts`]
/// - [`Check::c_exactly_equal_numbers`]
/// - [`Check::c_tolerance_equal_numbers`]
/// - [`Check::c_one_of`]
/// - [`Check::c_contains`]
/// - [`Check::c_is_contained_in`]
///
/// If you want this (one selector, some transformations and one criterion)
/// as the only condition of a check you can call
/// [`with_severity`](CheckConditionBuilder::with_severity) on the result to convert
/// this directly into a [`TestCaseCheck`].
/// You can also use [`make_error`](CheckConditionBuilder::make_error) and
/// [`make_warning`](CheckConditionBuilder::make_warning).
///
///
///
///
/// ## Example
///
/// ```
/// # use scratch_test_spec::Check;
/// let check = Check::last_line()
///    .t_to_uppercase()
///    .t_trim_left_right()
///    .c_equal_texts("0");
/// ```
pub struct Check {
    select: Selector,
    transformations: Vec<Transformation>,
}
impl Check {
    pub fn new(select: Selector) -> Self {
        Self {
            select,
            transformations: vec![],
        }
    }
    /// Shortcut for [`Selector::LastLine`]
    pub fn last_line() -> Self {
        Self::new(Selector::LastLine)
    }
    /// Shortcut for [`Selector::FirstLine`]
    pub fn first_line() -> Self {
        Self::new(Selector::FirstLine)
    }
    /// `n` begins with `0`, so `n = 1` is the second to last line
    ///
    /// Shortcut for [`Selector::NthLineFromEnd`]
    pub fn nth_line_from_end(n: u16) -> Self {
        Self::new(Selector::NthLineFromEnd { n })
    }
    /// `n` begins with `0`, so `n = 1` is the second line
    ///
    /// Shortcut for [`Selector::NthLineFromStart`]
    pub fn nth_line_from_start(n: u16) -> Self {
        Self::new(Selector::NthLineFromStart { n })
    }

    pub fn transform(mut self, t: impl Into<Transformation>) -> Self {
        self.transformations.push(t.into());
        self
    }
    pub fn criterion(self, criterion: Criterion) -> CheckConditionBuilder {
        CheckConditionBuilder(
            SingleCaseCheckCondition {
                select: self.select,
                transformations: self.transformations,
                criterion,
            }
            .into(),
        )
    }
    /// Shortcut for [`Transformation::ToUppercase`]
    pub fn t_to_uppercase(self) -> Self {
        self.transform(Transformation::ToUppercase {})
    }
    /// Shortcut for [`Transformation::TrimLeftRight`]
    pub fn t_trim_left_right(self) -> Self {
        self.transform(Transformation::TrimLeftRight {})
    }
    /// Shortcut for [`Transformation::ExtractSingleNumber`]
    pub fn t_extract_single_number(self) -> Self {
        self.transform(Transformation::ExtractSingleNumber {})
    }

    /// Shortcut for [`Criterion::EqualNumbers`] without a tolerance
    pub fn c_exactly_equal_numbers(self, number: impl Into<Number>) -> CheckConditionBuilder {
        self.criterion(Criterion::EqualNumbers {
            other: number.into(),
            float_tolerance: None,
        })
    }
    /// Shortcut for [`Criterion::EqualNumbers`] with a tolerance
    pub fn c_tolerance_equal_numbers(
        self,
        number: impl Into<Number>,
        tolerance: f64,
    ) -> CheckConditionBuilder {
        self.criterion(Criterion::EqualNumbers {
            other: number.into(),
            float_tolerance: Some(tolerance),
        })
    }
    /// Shortcut for [`Criterion::OneOf`]
    pub fn c_one_of(
        self,
        options: impl IntoIterator<Item = impl Into<String>>,
    ) -> CheckConditionBuilder {
        self.criterion(Criterion::OneOf {
            options: options.into_iter().map(|s| s.into()).collect(),
        })
    }
    /// Shortcut for [`Criterion::Contains`]
    pub fn c_contains(self, text: impl Into<String>) -> CheckConditionBuilder {
        self.criterion(Criterion::Contains { text: text.into() })
    }
    /// Shortcut for [`Criterion::ContainedIn`]
    pub fn c_is_contained_in(self, text: impl Into<String>) -> CheckConditionBuilder {
        self.criterion(Criterion::ContainedIn { text: text.into() })
    }
    /// Shortcut for [`Criterion::EqualTexts`]
    pub fn c_equal_texts(self, text: impl Into<String>) -> CheckConditionBuilder {
        self.criterion(Criterion::EqualTexts { other: text.into() })
    }
}

pub struct CheckConditionBuilder(Condition<SingleCaseCheckCondition>);

impl From<CheckConditionBuilder> for Condition<SingleCaseCheckCondition> {
    fn from(value: CheckConditionBuilder) -> Self {
        value.0
    }
}

impl CheckConditionBuilder {
    pub fn with_severity(self, severity: CaseCheckSeverity) -> TestCaseCheck {
        TestCaseCheck::new(severity, self.0)
    }
    pub fn make_warning(self) -> TestCaseCheck {
        self.with_severity(CaseCheckSeverity::Warning)
    }
    pub fn make_error(self) -> TestCaseCheck {
        self.with_severity(CaseCheckSeverity::Error)
    }
}

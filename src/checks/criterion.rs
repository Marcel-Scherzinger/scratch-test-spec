use std::num::NonZeroUsize;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{Report, default_state::DefaultState};
use svalue::{SNumber, SValue};

use crate::{Number, RandomsCfg};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum Criterion {
    EqualTexts {
        other: String,
    },
    #[serde(rename_all = "kebab-case")]
    EqualNumbers {
        other: Number,
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

#[derive(Debug, PartialEq)]
pub enum CriterionError<'s> {
    ExpectedNumberAsOutput,

    NotOneOfAllowedOptions(&'s Vec<String>),
    DoesntContainExpectedText(&'s str),
    IsNotContainedIn(&'s str),
    UnequalTexts(&'s str),
    NumbersNotEqual {
        other: Number,
        float_tolerance: Option<f64>,
    },
}
impl CriterionError<'_> {
    pub fn explain(&self, value: &SValue) -> String {
        match self {
            Self::ExpectedNumberAsOutput => format!("{value:?} is not a number as required"),
            Self::NotOneOfAllowedOptions(options) => {
                format!("program output {value:?} should be one of the following: {options:?}")
            }
            Self::DoesntContainExpectedText(t) => {
                format!("program output {value:?} should contain {t:?}")
            }
            Self::IsNotContainedIn(t) => {
                format!("program output {value:?} should be contained in {t:?}")
            }
            Self::UnequalTexts(other) => {
                format!("program output {value:?} is expected to be exactly equal to {other:?}")
            }
            Self::NumbersNotEqual {
                other,
                float_tolerance: Some(tolerance),
            } => {
                format!(
                    "program output {value:?} should differ at most {tolerance} from number {other:?}"
                )
            }
            Self::NumbersNotEqual {
                other,
                float_tolerance: None,
            } => {
                format!("program output {value:?} should be exactly the same number as {other:?}")
            }
        }
    }
}

trait TrueOrErr<T> {
    fn error(self, v: T) -> Result<(), T>;
}
impl<T> TrueOrErr<T> for bool {
    fn error(self, v: T) -> Result<(), T> {
        self.then_some(()).ok_or(v)
    }
}

impl Criterion {
    pub(crate) fn is_satisfied(
        &self,
        randoms: Option<&RandomsCfg>,
        report: &Report<'_, DefaultState>,
        value: &SValue,
    ) -> Result<(), CriterionError<'_>> {
        let stringified = value.to_string();
        match self {
            Criterion::OneOf { options } => {
                return options
                    .contains(&stringified)
                    .error(CriterionError::NotOneOfAllowedOptions(options));
            }
            Criterion::Contains { text } => {
                return stringified
                    .contains(text)
                    .error(CriterionError::DoesntContainExpectedText(text));
            }
            Criterion::ContainedIn { text } => {
                return text
                    .contains(&stringified)
                    .error(CriterionError::IsNotContainedIn(text));
            }
            Criterion::EqualTexts { other } => {
                return (stringified == *other).error(CriterionError::UnequalTexts(other));
            }
            Criterion::EqualNumbers {
                other,
                float_tolerance,
            } => equal_numbers(value, other, float_tolerance)?.error(
                CriterionError::NumbersNotEqual {
                    other: other.clone(),
                    float_tolerance: *float_tolerance,
                },
            )?,
        }
        Ok(())
    }
}

fn equal_numbers<'a>(
    value: &SValue,
    other: &Number,
    float_tolerance: &Option<f64>,
) -> Result<bool, CriterionError<'a>> {
    if value.is_int() {
        let val = value.q_as_number(&mut ());
        let val = val.int_or_border(&mut ());
        Ok(match other {
            crate::Number::Int(i) => val == *i,
            crate::Number::Float(f) => (val as f64) == *f,
        })
    } else if value.is_float() {
        let val = value.q_as_float(&mut ());
        Ok(if let Some(tol) = float_tolerance {
            let delta = match other {
                crate::Number::Int(i) => ((*i as f64) - val).abs(),
                crate::Number::Float(f) => (f - val).abs(),
            };
            delta < *tol
        } else {
            match other {
                crate::Number::Int(i) => (*i as f64) == val,
                crate::Number::Float(f) => *f == val,
            }
        })
    } else {
        Err(CriterionError::ExpectedNumberAsOutput)
    }
}

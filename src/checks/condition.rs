use derive_getters::Getters;
use derive_more::From;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{Report, default_state::DefaultState};
use svalue::SValue;

use super::{Criterion, Selector, Transformation};
use crate::{
    RandomsCfg,
    checks::SingleCheckCondition,
    error::{CriterionError, TransformationError},
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, From)]
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

impl ConditionError<'_> {
    pub fn explain(&self) -> String {
        match self {
            Self::ArgumentOfNotSucceeded(inner) => {
                format!("the condition passed to `Not` didn't fail as required: {inner:?}")
            }
            Self::NotAll(one) => format!(
                "at least one condition passed to `All` failed: {}",
                one.explain()
            ),
            Self::NoneOfAny(total) => {
                let total = total.iter().map(ConditionError::explain).collect_vec();
                format!("none of the conditions passed to `Any` succeeded: {total:?}")
            }
            Self::SelectorHasNoTarget(selector) => {
                format!("the program output didn't contain a target for the selector: {selector:?}")
            }
            Self::Transformation {
                error,
                input,
                idx,
                context,
            } => {
                format!(
                    "the transformation at index {idx} ({:?}) failed on input {input:?} with {error:?}",
                    context.transformations[*idx]
                )
            }
            Self::Criterion {
                criterion_err,
                context,
                program_value,
            } => criterion_err.explain(program_value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConditionError<'s> {
    /// failure when selecting from the output
    SelectorHasNoTarget(&'s Selector),
    Transformation {
        error: TransformationError,
        input: SValue,
        idx: usize,
        context: &'s SingleCheckCondition,
    },
    /// a criterion rejected the output
    Criterion {
        /// the cause the produced value is invalid
        criterion_err: CriterionError<'s>,
        /// collection of selector, transformations and applied criterion
        context: &'s SingleCheckCondition,
        /// the selected value after transformations that was seen by the criterion
        program_value: SValue,
    },
    /// `Any` was called without any successfull conditions
    NoneOfAny(Vec<ConditionError<'s>>),
    /// at least one conditions of `All` failed
    NotAll(Box<ConditionError<'s>>),
    /// the part inside `Not` succeeded
    ArgumentOfNotSucceeded(&'s TestCaseCheckCondition),
}

impl TestCaseCheckCondition {
    pub(crate) fn is_satisfied<'s>(
        &'s self,
        randoms: Option<&RandomsCfg>,
        report: &Report<'_, DefaultState>,
    ) -> Result<(), ConditionError<'s>> {
        use crate::checks::CompoundCheckCondition as C;
        use TestCaseCheckCondition as T;

        match self {
            T::Compound(C::All(v)) => v
                .iter()
                .map(|t| t.is_satisfied(randoms, report))
                .collect::<Result<Vec<_>, _>>()
                .map(|_| ())
                .map_err(|c| ConditionError::NotAll(c.into())),
            T::Compound(C::Any(v)) => {
                let (ok, err): (Vec<_>, Vec<_>) = v
                    .iter()
                    .map(|t| t.is_satisfied(randoms, report))
                    .partition(Result::is_ok);
                if !ok.is_empty() {
                    return Ok(());
                }
                let err = err.into_iter().flat_map(Result::err).collect();
                Err(ConditionError::NoneOfAny(err))
            }
            T::Compound(C::Not(v)) => match v.is_satisfied(randoms, report) {
                Ok(()) => Err(ConditionError::ArgumentOfNotSucceeded(v)),
                Err(_) => Ok(()),
            },
            T::Single(single) => single.is_satisfied(randoms, report),
        }
    }
}

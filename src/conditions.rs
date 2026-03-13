use derive_more::From;
use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

pub trait AnySingleCondition {
    type Error<'s>: ExplainableFailure
    where
        Self: 's;
    type Input<'i>: Copy;
    fn check<'s>(&'s self, input: Self::Input<'_>) -> Result<(), Self::Error<'s>>
    where
        Self: Sized;
}

pub trait ExplainableFailure {
    fn explain(&self) -> String;
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, From, ToSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Condition<Single> {
    Compound(CompoundCheckCondition<Single>),
    Single(Single),
}
#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum CompoundCheckCondition<Single> {
    All(Vec<Condition<Single>>),
    Any(Vec<Condition<Single>>),
    Not(Box<Condition<Single>>),
}

impl<S: AnySingleCondition + std::fmt::Debug> ExplainableFailure for ConditionError<'_, S> {
    fn explain(&self) -> String {
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
            Self::Single(single) => single.explain(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ConditionError<'s, Single: AnySingleCondition> {
    Single(Single::Error<'s>),

    /// `Any` was called without any successfull conditions
    NoneOfAny(Vec<ConditionError<'s, Single>>),
    /// at least one conditions of `All` failed
    NotAll(Box<ConditionError<'s, Single>>),
    /// the part inside `Not` succeeded
    ArgumentOfNotSucceeded(&'s Condition<Single>),
}

impl<S: AnySingleCondition> Condition<S> {
    pub(crate) fn check<'s>(&'s self, input: S::Input<'_>) -> Result<(), ConditionError<'s, S>> {
        use crate::spec::CompoundCheckCondition as C;
        use Condition as T;

        match self {
            T::Compound(C::All(v)) => v
                .iter()
                .map(|t| t.check(input))
                .collect::<Result<Vec<_>, _>>()
                .map(|_| ())
                .map_err(|c| ConditionError::NotAll(c.into())),
            T::Compound(C::Any(v)) => {
                let (ok, err): (Vec<_>, Vec<_>) =
                    v.iter().map(|t| t.check(input)).partition(Result::is_ok);
                if !ok.is_empty() {
                    return Ok(());
                }
                let err = err.into_iter().flat_map(Result::err).collect();
                Err(ConditionError::NoneOfAny(err))
            }
            T::Compound(C::Not(v)) => match v.check(input) {
                Ok(()) => Err(ConditionError::ArgumentOfNotSucceeded(v)),
                Err(_) => Ok(()),
            },
            T::Single(single) => single.check(input).map_err(ConditionError::Single),
        }
    }
}

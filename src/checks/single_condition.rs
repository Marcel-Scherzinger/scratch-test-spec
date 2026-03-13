use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{Report, default_state::DefaultState};
use svalue::SValue;

use crate::{
    RandomsCfg,
    checks::{Criterion, Selector, Transformation},
    conditions::AnySingleCondition,
    error::{CriterionError, TransformationError},
};

impl AnySingleCondition for SingleCaseCheckCondition {
    type Error<'s> = SingleConditionError<'s>;

    type Input<'i> = (Option<&'i RandomsCfg>, &'i Report<'i, DefaultState>);

    fn check<'s>(&'s self, (randoms, report): Self::Input<'_>) -> Result<(), Self::Error<'s>>
    where
        Self: Sized,
    {
        self.is_satisfied(randoms, report)
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct SingleCaseCheckCondition {
    pub(super) select: Selector,
    pub(super) transformations: Vec<Transformation>,
    pub(super) criterion: Criterion,
}
impl SingleCaseCheckCondition {
    pub fn new(
        select: Selector,
        transformations: Vec<Transformation>,
        criterion: Criterion,
    ) -> Self {
        Self {
            select,
            transformations,
            criterion,
        }
    }

    pub(crate) fn is_satisfied(
        &self,
        randoms: Option<&RandomsCfg>,
        report: &Report<'_, DefaultState>,
    ) -> Result<(), <Self as AnySingleCondition>::Error<'_>> {
        let selected = match self.select {
            Selector::NthLineFromEnd { n } => report.state().output_actions().rev().nth(n.into()),
            Selector::NthLineFromStart { n } => report.state().output_actions().nth(n.into()),
            Selector::LastLine => report.state().output_actions().last(),
            Selector::FirstLine => report.state().output_actions().next(),
        };
        let Some((_, value)) = selected else {
            return Err(SingleConditionError::SelectorHasNoTarget(&self.select));
        };
        let mut value = value.clone();

        for (idx, t) in self.transformations.iter().enumerate() {
            value =
                t.transform(value.clone())
                    .map_err(|err| SingleConditionError::Transformation {
                        error: err,
                        input: value,
                        idx,
                        context: self,
                    })?;
        }

        self.criterion
            .is_satisfied(randoms, report, &value)
            .map_err(|c| SingleConditionError::Criterion {
                criterion_err: c,
                program_value: value,
                context: self,
            })
    }
}

#[derive(Debug, PartialEq)]
pub enum SingleConditionError<'s> {
    /// failure when selecting from the output
    SelectorHasNoTarget(&'s Selector),
    Transformation {
        error: TransformationError,
        input: SValue,
        idx: usize,
        context: &'s SingleCaseCheckCondition,
    },
    /// a criterion rejected the output
    Criterion {
        /// the cause the produced value is invalid
        criterion_err: CriterionError<'s>,
        /// collection of selector, transformations and applied criterion
        context: &'s SingleCaseCheckCondition,
        /// the selected value after transformations that was seen by the criterion
        program_value: SValue,
    },
}

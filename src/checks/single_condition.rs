use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{Report, default_state::DefaultState};

use crate::{
    RandomsCfg,
    checks::{Criterion, Selector, Transformation},
    error::ConditionError,
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
#[serde(rename_all = "kebab-case")]
pub struct SingleCheckCondition {
    pub(super) select: Selector,
    pub(super) transformations: Vec<Transformation>,
    pub(super) criterion: Criterion,
}
impl SingleCheckCondition {
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
    ) -> Result<(), ConditionError<'_>> {
        let selected = match self.select {
            Selector::NthLineFromEnd { n } => report.state().output_actions().rev().nth(n.into()),
            Selector::NthLineFromStart { n } => report.state().output_actions().nth(n.into()),
            Selector::LastLine => report.state().output_actions().last(),
            Selector::FirstLine => report.state().output_actions().next(),
        };
        let Some((_, value)) = selected else {
            return Err(ConditionError::SelectorHasNoTarget(&self.select));
        };
        let mut value = value.clone();

        for (idx, t) in self.transformations.iter().enumerate() {
            value = t
                .transform(value.clone())
                .map_err(|err| ConditionError::Transformation {
                    error: err,
                    input: value,
                    idx,
                    context: self,
                })?;
        }

        self.criterion
            .is_satisfied(randoms, report, &value)
            .map_err(|c| ConditionError::Criterion {
                criterion_err: c,
                program_value: value,
                context: self,
            })
    }
}

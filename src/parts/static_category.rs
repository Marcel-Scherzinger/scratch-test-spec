use crate::{
    RandomsCfg,
    parts::test_case::{TestCase, TestCaseReport},
};
use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smodel::ProjectDoc;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, derive_getters::Getters)]
pub struct StaticCategoryReport<'s> {
    description: Option<&'s String>,
    randoms: Option<&'s RandomsCfg>,
    cases: Vec<TestCaseReport<'s>>,
}

impl StaticTestCategory {
    pub(crate) fn run_on(
        &self,
        doc: &ProjectDoc,
        initial_block: &smodel::Id,
    ) -> StaticCategoryReport<'_> {
        StaticCategoryReport {
            cases: self
                .cases
                .iter()
                .map(|case| case.run_on(self.randoms.as_ref(), doc, initial_block))
                .collect(),
            description: self.description.as_ref(),
            randoms: self.randoms.as_ref(),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
pub struct StaticTestCategory {
    /// optional description about kind of contained tests and tested criteria
    description: Option<String>,
    randoms: Option<RandomsCfg>,
    cases: Vec<TestCase>,
}

crate::impl_modifiers!(StaticTestCategory {
    description: {optinto} String,
    randoms: Option<RandomsCfg>,
});

impl StaticTestCategory {
    pub fn new(cases: Vec<TestCase>) -> Self {
        Self {
            description: None,
            randoms: None,
            cases,
        }
    }

    pub fn cases_mut(&mut self) -> &mut Vec<TestCase> {
        &mut self.cases
    }
}

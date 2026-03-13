use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smodel::ProjectDoc;

use crate::parts::{TestCategory, category::CategoryReport};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
pub struct TestSpecification {
    /// list of test categories
    categories: Vec<TestCategory>,
    /// Ignored but preserved by application, can be used to add extra information
    misc: Option<serde_json::Value>,
    /// Optional solution file that can be used to check the test specification
    solution: Option<serde_json::Value>,
}

impl TestSpecification {
    pub fn new(categories: Vec<impl Into<TestCategory>>) -> Self {
        Self {
            categories: categories.into_iter().map(|s| s.into()).collect(),
            misc: None,
            solution: None,
        }
    }
    pub fn categories_mut(&mut self) -> &mut Vec<TestCategory> {
        &mut self.categories
    }
}

crate::impl_modifiers!(
    TestSpecification {
        solution: Option<serde_json::Value>,
        misc: Option<serde_json::Value>,
    }
);

impl TestSpecification {
    pub fn run_on(&self, doc: &ProjectDoc) -> Result<SpecReport<'_>, SpecError> {
        let initial_block = super::find_initial_block(doc)?;

        let reports = self
            .categories
            .iter()
            .map(|cat| cat.run_on(doc, initial_block))
            .collect();
        Ok(SpecReport {
            categories: reports,
        })
    }
}

#[derive(Debug, PartialEq, Getters)]
pub struct SpecReport<'s> {
    categories: Vec<CategoryReport<'s>>,
}
#[derive(Debug, PartialEq)]
pub enum SpecError {
    NoInitialBlockFound,
    AmbiguousInitialBlock,
}

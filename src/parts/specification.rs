use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smodel::ProjectDoc;

use crate::{
    lints::{Lint, LintReport},
    parts::{TestCategory, category::CategoryReport},
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
pub struct TestSpecification {
    lints: Vec<Lint>,
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
            lints: vec![],
            categories: categories.into_iter().map(|s| s.into()).collect(),
            misc: None,
            solution: None,
        }
    }
    pub fn categories_mut(&mut self) -> &mut Vec<TestCategory> {
        &mut self.categories
    }
    pub fn lints_mut(&mut self) -> &mut Vec<Lint> {
        &mut self.lints
    }
    pub fn and_lint(mut self, l: Lint) -> Self {
        self.lints.push(l);
        self
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

        let lints = self.lints.iter().map(|lint| lint.run_on(doc)).collect();

        let reports = self
            .categories
            .iter()
            .map(|cat| cat.run_on(doc, initial_block))
            .collect();
        Ok(SpecReport {
            lints,
            categories: reports,
        })
    }
}

#[derive(Debug, PartialEq, Getters)]
pub struct SpecReport<'s> {
    lints: Vec<LintReport<'s>>,
    categories: Vec<CategoryReport<'s>>,
}
#[derive(Debug, PartialEq)]
pub enum SpecError {
    NoInitialBlockFound,
    AmbiguousInitialBlock,
}

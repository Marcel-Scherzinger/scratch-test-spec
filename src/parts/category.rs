use derive_more::From;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smodel::ProjectDoc;

use crate::parts::{static_category::StaticCategoryReport, static_category::StaticTestCategory};

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, From)]
#[serde(rename_all = "kebab-case", tag = "type")]
pub enum TestCategory {
    Static(StaticTestCategory),
}

#[derive(Debug, PartialEq)]
pub enum CategoryReport<'s> {
    Static(StaticCategoryReport<'s>),
}

impl TestCategory {
    pub(crate) fn run_on(
        &self,
        doc: &ProjectDoc,
        initial_block: &smodel::Id,
    ) -> CategoryReport<'_> {
        match self {
            Self::Static(s) => CategoryReport::Static(s.run_on(doc, initial_block)),
        }
    }
}

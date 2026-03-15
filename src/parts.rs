use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use smodel::{
    ProjectDoc,
    blocks::{BlockKindUnit, EventBlockKindUnit},
};

pub(crate) mod category;
pub(crate) mod specification;
pub(crate) mod static_category;
pub(crate) mod test_case;

pub use category::TestCategory;
pub use specification::TestSpecification;
pub use static_category::StaticTestCategory;
pub use test_case::TestCase;
use utoipa::ToSchema;

use crate::error::SpecError;

fn find_initial_block(doc: &ProjectDoc) -> Result<&smodel::Id, SpecError> {
    const INITIAL: BlockKindUnit = BlockKindUnit::Event(EventBlockKindUnit::EventWhenflagclicked);
    let blocks: Vec<&smodel::Id> = doc
        .ids_with_opcodes()
        .filter_map(|(id, opcodes)| (opcodes == INITIAL).then_some(id))
        .collect();
    if blocks.len() > 1 {
        Err(SpecError::AmbiguousInitialBlock)
    } else if let Some(block) = blocks.first() {
        Ok(block)
    } else {
        Err(SpecError::NoInitialBlockFound)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Clone, ToSchema)]
#[serde(rename_all = "kebab-case", untagged)]
pub enum Number {
    Int(i64),
    Float(f64),
}

#[skip_serializing_none]
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub struct CheckResultMessages {
    human_msg: Option<String>,
    tools_msg: Option<String>,
    help_url: Option<String>,
}

crate::impl_modifiers!(CheckResultMessages {
    human_msg: {optinto} String,
    tools_msg: {optinto} String,
    help_url: {optinto} String,
});

impl CheckResultMessages {
    pub fn new() -> Self {
        Self {
            human_msg: None,
            tools_msg: None,
            help_url: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters, ToSchema)]
pub struct RandomsCfg {
    generate: RandomsGenerate,
}
impl RandomsCfg {
    pub fn new(generate: RandomsGenerate) -> Self {
        Self { generate }
    }
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "kebab-case", tag = "status")]
pub enum RandomsGenerate {
    /// no program is allowed to request random numbers, doing so will lead to
    /// termination and an error
    Deny,

    /// random numbers are generated based on an optional seed value,
    /// the same library version on the same target will produce the same sequence
    /// for the same seed but there are no further guarantees
    #[serde(rename_all = "kebab-case")]
    Allow { seed: Option<u64> },
}

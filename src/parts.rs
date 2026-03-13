use derive_getters::Getters;
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

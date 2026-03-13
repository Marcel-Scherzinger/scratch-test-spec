use crate::spec::TestCaseCheckCondition;

use super::{Criterion, Selector, SingleCheckCondition, Transformation};

pub struct Check {
    select: Selector,
    transformations: Vec<Transformation>,
}

impl Check {
    pub fn new(select: Selector) -> Self {
        Self {
            select,
            transformations: vec![],
        }
    }
    pub fn last_line() -> Self {
        Self::new(Selector::LastLine)
    }
    pub fn first_line() -> Self {
        Self::new(Selector::FirstLine)
    }
    pub fn nth_line_from_end(n: u16) -> Self {
        Self::new(Selector::NthLineFromEnd { n })
    }

    pub fn transform(mut self, t: impl Into<Transformation>) -> Self {
        self.transformations.push(t.into());
        self
    }
    pub fn criterion(self, criterion: Criterion) -> TestCaseCheckCondition {
        SingleCheckCondition {
            select: self.select,
            transformations: self.transformations,
            criterion,
        }
        .into()
    }
    pub fn c_equal_texts(self, text: impl Into<String>) -> TestCaseCheckCondition {
        self.criterion(Criterion::EqualTexts { other: text.into() })
    }
}

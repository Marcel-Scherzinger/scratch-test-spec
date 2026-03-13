use derive_getters::Getters;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use sinterpreter::{
    Limits, RunError,
    default_state::{DefaultState, DefaultStateError},
};
use smodel::ProjectDoc;
use svalue::SList;

use crate::{
    RandomsCfg,
    checks::{TestCaseCheck, case_check::CheckReport},
};

#[skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema, Getters)]
pub struct TestCase {
    inputs: Vec<String>,
    // randoms: Option<RandomsCfg>,
    checks: Vec<TestCaseCheck>,
}

impl TestCase {
    pub fn new(inputs: Vec<impl Into<String>>) -> Self {
        Self {
            inputs: inputs.into_iter().map(|s| s.into()).collect(),
            checks: vec![],
        }
    }
    pub fn and_check(mut self, check: TestCaseCheck) -> Self {
        self.checks.push(check);
        self
    }
    pub fn checks_mut(&mut self) -> &mut Vec<TestCaseCheck> {
        &mut self.checks
    }
    pub fn inputs_mut(&mut self) -> &mut Vec<String> {
        &mut self.inputs
    }
}

#[derive(Debug, PartialEq, derive_getters::Getters)]
pub struct TestCaseReport<'s> {
    interpreter_state: DefaultState,
    interpreter_limits: Limits,
    interpreter_error_code: Option<RunError<DefaultStateError>>,
    checks: Vec<CheckReport<'s>>,
    case: &'s TestCase,
}

impl TestCase {
    pub(crate) fn run_on(
        &self,
        randoms: Option<&RandomsCfg>,
        doc: &ProjectDoc,
        initial_block: &smodel::Id,
    ) -> TestCaseReport<'_> {
        let mut state = DefaultState::new();

        for (var, val) in doc
            .targets()
            .iter()
            .flat_map(|t| t.variables().ids().flat_map(|id| t.variables().get(id)))
        {
            state.variables_mut().insert(var.clone(), val.clone());
        }

        for (lis, el) in doc
            .targets()
            .iter()
            .flat_map(|t| t.lists().ids().flat_map(|id| t.lists().get(id)))
        {
            let mut slist = SList::new(el.len().max(1000) as i64);
            for i in el {
                #[allow(unused)]
                slist.append_item(i.clone());
            }
            state.lists_mut().insert(lis.clone(), slist);
        }

        state.set_answers(self.inputs.iter().map(|s| s.clone().into()).collect());
        let report = sinterpreter::Interpreter::new_restrictive().run(doc, state, initial_block);

        let checks = self
            .checks
            .iter()
            .map(|c| c.run_on(randoms, &report))
            .collect();

        let (interpreter_state, interpreter_error_code, interpreter_limits) = report.take_parts();

        TestCaseReport {
            interpreter_state,
            interpreter_limits,
            interpreter_error_code,
            checks,
            case: self,
        }
    }
}

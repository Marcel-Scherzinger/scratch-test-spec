use scratch_test_spec::ExplainableFailure;
use scratch_test_spec::report::CategoryReport;
use scratch_test_spec::spec::{
    Check, Lint, StaticTestCategory, TestCase, TestCaseCheck, TestSpecification, Transformation,
};
use smodel::ProjectDoc;
use smodel::blocks::StmtBlockKindUnit;

fn specification() -> TestSpecification {
    TestSpecification::new(vec![StaticTestCategory::new(vec![
        TestCase::new(vec!["1", "10"]).and_check(TestCaseCheck::new_error(
            Check::last_line()
                .transform(Transformation::ExtractSingleNumber {})
                .c_equal_texts("45"),
        )),
    ])])
    .and_lint(Lint::block_count_limit(StmtBlockKindUnit::ControlRepeat, 0).make_error())
}

#[test]
fn test_sum_from_to_with_lints_without_repeat() {
    let json = smodel::json_from_sb3_file("sb3/sum-from-to.sb3").unwrap();
    let doc = ProjectDoc::from_json(&json).unwrap();

    let spec = specification();

    let res = spec.run_on(&doc).unwrap();
    let check_report = match &res.categories()[0] {
        CategoryReport::Static(s) => &s.cases()[0].checks()[0],
    };
    assert!(check_report.success());
    assert!(check_report.failure().is_none());

    let lint = &res.lints()[0];
    assert!(lint.failure().is_none());
}

#[test]
fn test_sum_from_to_with_lints_with_repeat() {
    let json = smodel::json_from_sb3_file("sb3/sum-from-to-with-repeat-loop.sb3").unwrap();
    let doc = ProjectDoc::from_json(&json).unwrap();

    let spec = specification();

    let res = spec.run_on(&doc).unwrap();
    let check_report = match &res.categories()[0] {
        CategoryReport::Static(s) => &s.cases()[0].checks()[0],
    };
    assert!(check_report.success());
    assert!(check_report.failure().is_none());

    let lint = &res.lints()[0];
    assert_eq!(
        "program uses more instances of control_repeat blocks than allowed (1 > 0)",
        lint.failure().unwrap().explain()
    );
}

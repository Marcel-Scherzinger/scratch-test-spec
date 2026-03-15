use scratch_test_spec::report::CategoryReport;
use scratch_test_spec::spec::{
    Check, StaticTestCategory, TestCase, TestSpecification, Transformation,
};
use smodel::ProjectDoc;

#[test]
fn test_sum_from_to_only_checks() {
    let json = smodel::json_from_sb3_file("sb3/sum-from-to.sb3").unwrap();
    let doc = ProjectDoc::from_json(&json).unwrap();

    let spec1 = TestSpecification::new(vec![StaticTestCategory::new(vec![
        TestCase::new(vec!["1", "10"])
            .and_check(Check::last_line().c_equal_texts("45").make_error()),
    ])]);

    let res1 = spec1.run_on(&doc).unwrap();
    let check_report = match &res1.categories()[0] {
        CategoryReport::Static(s) => &s.cases()[0].checks()[0],
    };
    assert!(!check_report.success());

    let spec2 = TestSpecification::new(vec![StaticTestCategory::new(vec![
        TestCase::new(vec!["1", "10"]).and_check(
            Check::last_line()
                .transform(Transformation::ExtractSingleNumber {})
                .c_equal_texts("45")
                .make_error(),
        ),
    ])]);

    let res2 = spec2.run_on(&doc).unwrap();
    let check_report = match &res2.categories()[0] {
        CategoryReport::Static(s) => &s.cases()[0].checks()[0],
    };
    assert!(check_report.failure().is_none());
    assert!(check_report.success());
}

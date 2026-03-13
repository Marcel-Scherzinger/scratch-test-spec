use serde_json::Map;

use crate::spec::{
    Check, CompoundCheckCondition, Criterion, StaticTestCategory, TestCase, TestCaseCheck,
    TestCategory, TestSpecification, Transformation,
};
use crate::{RandomsCfg, RandomsGenerate};

#[test]
fn test_ser() {
    let spec = TestSpecification::new(vec![TestCategory::Static(
        StaticTestCategory::new(vec![TestCase::new(vec!["1", "2"]).and_check(
            TestCaseCheck::new_error(CompoundCheckCondition::All(vec![
                        Check::last_line()
                            .transform(Transformation::ToUppercase)
                            .transform(Transformation::TrimLeftRight)
                            .transform(Transformation::ExtractSingleNumber)
                            .criterion(Criterion::EqualNumbers {
                                other: crate::Number::Int(3),
                                float_tolerance: Some(1e-9),
                            }),
                        Check::nth_line_from_end(3)
                            .transform(Transformation::ExtractSingleNumber)
                            .c_equal_texts("0"),
                    ])),
        )])
        .with_description(Some("Both positive"))
        .with_randoms(Some(RandomsCfg {
            generate: RandomsGenerate::Allow { seed: Some(73) },
        })),
    )])
    .with_misc(Some(serde_json::Value::Object(Map::from_iter(
        vec![
            ("title", "XX – Title"),
            ("input", "Some description of input"),
        ]
        .into_iter()
        .map(|(x, y)| (x.to_string(), y.to_string().into())),
    ))));
    let _serialised = (
        "{}",
        serde_json::to_string_pretty(&schemars::schema_for!(TestSpecification).to_value()).unwrap(),
    );
    let _schema = serde_json::to_string_pretty(&spec).unwrap();
}

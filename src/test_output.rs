use serde_json::Map;

use crate::{
    Criterion, RandomsCfg, RandomsGenerate, SingleCheckCondition, StaticTestCategory, TestCase,
    TestCaseCheck, TestCategory, TestSpecification, Transformation,
};

#[test]
fn test_ser() {
    let spec = TestSpecification {
        solution: None,
        categories: vec![TestCategory::Static(StaticTestCategory {
            description: Some("Both positive".to_string()),
            randoms: Some(RandomsCfg {
                generate: RandomsGenerate::Allow { seed: Some(73) },
            }),
            cases: vec![TestCase {
                inputs: vec!["1".to_string(), "2".to_string()],
                checks: vec![TestCaseCheck {
                    severity: crate::TestCaseSeverity::Error,
                    on_success: None,
                    on_failure: None,
                    condition: crate::TestCaseCheckCondition::Compound(
                        crate::CompoundCheckCondition::All(vec![
                            crate::TestCaseCheckCondition::Single(SingleCheckCondition {
                                select: crate::Selector::LastLine,
                                transformations: vec![
                                    Transformation::ToUppercase,
                                    Transformation::TrimLeftRight,
                                    Transformation::ExtractSingleNumber,
                                ],
                                criterion: Criterion::EqualNumbers {
                                    other: 3.into(),
                                    float_tolerance: Some(1e-9),
                                },
                            }),
                            crate::TestCaseCheckCondition::Single(SingleCheckCondition {
                                select: crate::Selector::NthLineFromEnd {
                                    n: 3.try_into().unwrap(),
                                },
                                transformations: vec![Transformation::ExtractSingleNumber],
                                criterion: Criterion::EqualTexts {
                                    other: "0".to_string(),
                                },
                            }),
                        ]),
                    ),
                }],
            }],
        })],
        misc: Some(serde_json::Value::Object(Map::from_iter(
            vec![
                ("title", "XX – Title"),
                ("input", "Some description of input"),
            ]
            .into_iter()
            .map(|(x, y)| (x.to_string(), y.to_string().into())),
        ))),
    };
    let _serialised = (
        "{}",
        serde_json::to_string_pretty(&schemars::schema_for!(TestSpecification).to_value()).unwrap(),
    );
    let _schema = serde_json::to_string_pretty(&spec).unwrap();
}

//! Shared helpers for property-based tests.

use proptest::prelude::*;

/// Strategy that produces arbitrary JSON values, including nested arrays and objects.
pub(crate) fn arb_json() -> impl Strategy<Value = serde_json::Value> {
    let leaf = prop_oneof![
        Just(serde_json::Value::Null),
        any::<bool>().prop_map(serde_json::Value::from),
        any::<i64>().prop_map(serde_json::Value::from),
        any::<u64>().prop_map(serde_json::Value::from),
        any::<f64>().prop_map(|f| serde_json::json!(f)),
        "\\PC*".prop_map(serde_json::Value::from),
    ];
    leaf.prop_recursive(4, 64, 8, |inner| {
        prop_oneof![
            proptest::collection::vec(inner.clone(), 0..8).prop_map(serde_json::Value::from),
            proptest::collection::btree_map("\\PC*", inner, 0..8)
                .prop_map(|map| serde_json::Value::Object(map.into_iter().collect())),
        ]
    })
}

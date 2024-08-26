#[macro_use]
extern crate json_easy2use;

use serde_json::json;

#[test]
fn test_get_macro() {
    let mydict = json!({
        "level1": {
            "level2": {
                "level3a": "value_a"
            }
        }
    });
    
    assert!(get!(mydict, "level1.level2.level3a").is_some());
}

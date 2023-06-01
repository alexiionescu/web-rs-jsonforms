use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! stringify_nosp {
    ($($t:tt)*) => {
        stringify!($($t)*)
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect()
    };
}
pub use stringify_nosp;

#[derive(Deserialize, Serialize,Clone,Debug)]
pub struct JsonFormsRequest {
    pub name: String,
}

#[derive(Serialize,Debug)]
pub enum ButtonPos {
    Center,
    Right,
}

#[derive(Serialize,Debug)]
pub enum ButtonType {
    Submit,
    NextForm,
}
#[derive(Serialize,Debug)]
pub struct Button {
    pub name: &'static str,
    pub form: Option<JsonFormsRequest>,
    pub btype: ButtonType,
    pub bpos: ButtonPos,
}

#[derive(Serialize,Debug)]
pub struct JsonFormsResponse {
    pub schema: String,
    pub uischema: String,
    pub buttons: Vec<Button>,
    pub title: Option<String>,
}

impl JsonFormsResponse {
    pub fn add_button(&mut self, b: Button) {
        self.buttons.push(b);
    }
}

impl From<(String, String)> for JsonFormsResponse {
    fn from((schema, uischema): (String, String)) -> Self {
        Self {
            schema,
            uischema,
            buttons: Vec::new(),
            title: None,
        }
    }
}

pub trait JsonFormsSerializable {
    fn jsonforms_schema() -> (String, String);
}

pub trait JsonFormsButtons {
    fn add_buttons(form: &mut JsonFormsResponse);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use jsonforms_derive::JsonForms;
    use serde_json::{self, json};
    use std::fs;

    const SCHEMA_OUTDIR: &str = "../";

    #[derive(JsonForms, Deserialize, Debug)]
    struct ArrayItem {
        #[jsonforms(schema = r#""minLength": 5"#)]
        #[jsonforms(uischema = r#""label": "Some Item Str""#)]
        some_str: String,

        #[jsonforms(uischema = r#""label": "Some Item Enum Str""#)]
        #[jsonforms(schema = r#""enum": ["Item Enum Str 1","Item Enum Str 2","Item Enum Str 3"]"#)]
        some_enum: Option<String>,
    }

    #[derive(JsonForms, Deserialize, Debug)]
    // #[jsonforms(debug)]
    struct TestJsonForms001 {
        #[jsonforms(VerticalLayout)]
        #[jsonforms(
            schema = r#""title": "Some Str Title","minLength": 3,"description": "Please enter your name""#
        )]
        some_str: String,

        #[jsonforms(schema = r#""format":"password""#)]
        some_sec_str: String,

        some_array: Vec<ArrayItem>,

        #[jsonforms(Skip)]
        #[serde(skip_deserializing)]
        skip_this: Option<String>,

        #[jsonforms(HorizontalLayout)]
        some_int: Option<i32>,
        some_float: f64,
        #[jsonforms(EndLayout)]
        some_bool: bool,

        #[jsonforms(uischema = r#""label": "Some Enum Str""#)]
        #[jsonforms(schema = r#""enum": ["Enum Str 1","Enum Str 2","Enum Str 3"]"#)]
        some_enum: String,

        some_opt_str: Option<String>,
    }

    #[test]
    fn jsonforms001() {
        let (s, uis) = TestJsonForms001::jsonforms_schema();
        let schema: serde_json::Result<serde_json::Value> = serde_json::from_str(&s);
        match schema {
            Ok(v) => {
                assert!(matches!(v["properties"], serde_json::Value::Object { .. }));
                if let serde_json::Value::Object(props) = &v["properties"] {
                    assert!(matches!(
                        props["some_str"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_str"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("string"));
                        assert!(user_props.contains_key("title"));
                        assert_eq!(user_props["title"], json!("Some Str Title"));
                    }
                    assert!(!props.contains_key("skip_this"));
                    assert!(matches!(
                        props["some_int"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_int"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("integer"));
                    }
                    assert!(matches!(
                        props["some_bool"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_bool"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("boolean"));
                    }
                    assert!(matches!(
                        props["some_float"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_float"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("number"));
                    }
                    assert!(matches!(
                        props["some_sec_str"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_sec_str"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("string"));
                    }
                    assert!(matches!(
                        props["some_enum"],
                        serde_json::Value::Object { .. }
                    ));
                    if let serde_json::Value::Object(user_props) = &props["some_enum"] {
                        assert!(user_props.contains_key("type"));
                        assert_eq!(user_props["type"], json!("string"));
                    }
                }
                fs::write(
                    format!("{}/test_schema.json", SCHEMA_OUTDIR),
                    serde_json::to_string_pretty(&v).unwrap(),
                )
                .expect("Unable to write schema file");
            }
            Err(err) => {
                println!(
                    "Parse Json Schema Error !!! \n{}\n<Error !!!>\n{}\n{}",
                    &s[..err.column() - 1],
                    &s[err.column()..],
                    err
                );
            }
        }
        let uischema: serde_json::Result<serde_json::Value> = serde_json::from_str(&uis);
        match uischema {
            Ok(v) => {
                fs::write(
                    format!("{}/test_uischema.json", SCHEMA_OUTDIR),
                    serde_json::to_string_pretty(&v).unwrap(),
                )
                .expect("Unable to write uischema file");
            }
            Err(err) => {
                println!(
                    "Parse Json UI Schema Error !!!\n{}\n<Error !!!>\n{}\n{}",
                    &uis[..err.column() - 1],
                    &uis[err.column() - 1..],
                    err
                );
            }
        }
    }
}

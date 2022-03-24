use serde::{Deserialize, Serialize};
use struct_field::StructField;
use struct_field_names::StructFieldNames;

#[test]
fn test_field() {
    #[rustfmt::skip::attributes(derive)]
    #[allow(dead_code)]
    #[derive(Default)]
    #[derive(StructField, StructFieldNames)]
    #[derive(Debug, Clone, PartialEq,Serialize, Deserialize)]
    struct Struct {
        id: i32,
        name: String,
        debug: Option<String>,
    }
    let name = StructField::name(String::from("3"));
    let debug = StructField::debug(Some(String::from("")));
    let debug2 = StructField::debug(None);
    let json = serde_json::to_string(&[&name, &debug, &debug2]).unwrap();
    println!("{}", json);

    let fields: Vec<StructField> = serde_json::from_str(&json).unwrap();
    let mut st = Struct::default();
    fields.into_iter().for_each(|field| {
        st.update_field(field);
    });
    println!("{}", serde_json::to_string(&st).unwrap());
}

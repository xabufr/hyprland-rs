use serde::Deserialize;

use super::from_str;

#[derive(Deserialize, PartialEq, Debug)]
enum MyEnum {
    Variant1,
    Variant2(String),
    Variant3(u32, String),
    Variant4(String, String),
    Variant5(Variant5Data),
    Variant6(Variant6Data),
    Variant7(Variant7Data),
    Variant8 { id: u32, name: String },
    VariantBool(bool),
}

#[derive(Deserialize, PartialEq, Debug)]
struct Variant5Data {}

#[derive(Deserialize, PartialEq, Debug)]
struct Variant6Data {
    id: u32,
    name: String,
}

#[derive(Deserialize, PartialEq, Debug)]
struct Variant7Data;

#[test]
fn test_simple_variant_deser() {
    test_successful_deserialization(r#"Variant1>>"#, MyEnum::Variant1)
}

#[test]
fn test_variant_wrapper_deser() {
    test_successful_deserialization(r#"Variant2>>toto"#, MyEnum::Variant2("toto".into()))
}

#[test]
fn test_variant_tuple_deser() {
    test_successful_deserialization(r#"Variant3>>1,toto"#, MyEnum::Variant3(1, "toto".into()))
}

#[test]
fn test_variant_long_string_deser() {
    test_successful_deserialization(
        r#"Variant4>>titi,toto,tata"#,
        MyEnum::Variant4("titi".into(), "toto,tata".into()),
    )
}

#[test]
fn test_variant_struct_deser() {
    test_successful_deserialization(r#"Variant5>>"#, MyEnum::Variant5(Variant5Data {}))
}

#[test]
fn test_variant_simple_struct_deser() {
    test_successful_deserialization(
        r#"Variant6>>2,test"#,
        MyEnum::Variant6(Variant6Data {
            id: 2,
            name: "test".into(),
        }),
    )
}

#[test]
fn test_variant_empty_struct_deser() {
    test_successful_deserialization(r#"Variant7>>"#, MyEnum::Variant7(Variant7Data))
}

#[test]
fn test_variant_struct_variant_deser() {
    test_successful_deserialization(
        r#"Variant8>>45,test2"#,
        MyEnum::Variant8 {
            id: 45,
            name: "test2".into(),
        },
    )
}

#[test]
fn test_bool_true() {
    test_successful_deserialization(r#"VariantBool>>1"#, MyEnum::VariantBool(true))
}

#[test]
fn test_bool_false() {
    test_successful_deserialization(r#"VariantBool>>0"#, MyEnum::VariantBool(false))
}

fn test_successful_deserialization(s: &str, expected: MyEnum) {
    let d: MyEnum = from_str(s).unwrap();
    assert_eq!(expected, d);
}

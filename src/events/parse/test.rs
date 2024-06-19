use rstest::*;
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
    VariantOption(Option<bool>),
    VariantOptionFirst(Option<bool>, u32),
    VariantTuple(TupleStruct),
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

#[derive(Deserialize, PartialEq, Debug)]
struct TupleStruct(u32, String);

#[rstest]
#[case("Variant1>>", MyEnum::Variant1)]
#[case("Variant2>>test", MyEnum::Variant2("test".into()))]
#[case("Variant3>>1,test", MyEnum::Variant3(1,"test".into()))]
#[case("Variant4>>titi,toto,tata", MyEnum::Variant4("titi".into(),"toto,tata".into()))]
#[case("Variant5>>", MyEnum::Variant5(Variant5Data{}))]
#[case("Variant6>>2,test", MyEnum::Variant6(Variant6Data{id:2,name:"test".into()}))]
#[case("Variant7>>", MyEnum::Variant7(Variant7Data))]
#[case("Variant8>>45,test2", MyEnum::Variant8{id:45,name:"test2".into()})]
#[case::bool_true("VariantBool>>1", MyEnum::VariantBool(true))]
#[case::bool_false("VariantBool>>0", MyEnum::VariantBool(false))]
#[case::option_none("VariantOption>>", MyEnum::VariantOption(None))]
#[case::option_some("VariantOption>>1", MyEnum::VariantOption(Some(true)))]
#[case::option_none_start("VariantOptionFirst>>,1", MyEnum::VariantOptionFirst(None, 1))]
#[case::tuple("VariantTuple>>1,toto", MyEnum::VariantTuple(TupleStruct(1, "toto".into())))]
fn test_successful_deserialization_cases(#[case] s: &str, #[case] expected: MyEnum) {
    let d: MyEnum = from_str(s).unwrap();
    assert_eq!(expected, d);
}

use serde::Deserialize;
use test_case::test_case;

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

#[test_case("Variant1>>", MyEnum::Variant1)]
#[test_case("Variant2>>test", MyEnum::Variant2("test".into()))]
#[test_case("Variant3>>1,test", MyEnum::Variant3(1,"test".into()))]
#[test_case("Variant4>>titi,toto,tata", MyEnum::Variant4("titi".into(),"toto,tata".into()))]
#[test_case("Variant5>>", MyEnum::Variant5(Variant5Data{}))]
#[test_case("Variant6>>2,test", MyEnum::Variant6(Variant6Data{id:2,name:"test".into()}))]
#[test_case("Variant7>>", MyEnum::Variant7(Variant7Data))]
#[test_case("Variant8>>45,test2", MyEnum::Variant8{id:45,name:"test2".into()})]
#[test_case("VariantBool>>1", MyEnum::VariantBool(true) ; "True boolean")]
#[test_case("VariantBool>>0", MyEnum::VariantBool(false); "False boolean")]
fn test_successful_deserialization_cases(s: &str, expected: MyEnum) {
    let d: MyEnum = from_str(s).unwrap();
    assert_eq!(expected, d);
}

use struct_field::StructField;
use struct_field_names::StructFieldNames;

#[test]
fn test_field() {
    #[allow(dead_code)]
    #[derive(StructField, StructFieldNames)]
    struct Struct {
        field: String,
    }
    let mut o = Struct { field: String::new() };
    o.update_field(StructField::field(String::from("3")));
    if let Some(StructField::field(field)) = o.fetch_field(Struct::FIELD_NAMES.field) {
        assert_eq!(field, "3");
    } else {
        unreachable!();
    }
    assert_eq!(o.field, "3");
}

#[test]
fn test_with_generic_struct() {
    #[allow(dead_code)]
    #[derive(StructField)]
    struct Struct<'a, T> {
        field: &'a T,
    }
    let i1 = 1u8;
    let i2 = 2u8;
    let mut o: Struct<u8> = Struct { field: &i1 };
    o.update_field(StructField::field(&i2));
    if let Some(StructField::field(field)) = o.fetch_field("field") {
        assert_eq!(field, &i2);
    } else {
        unreachable!();
    }
    assert_eq!(o.field, &i2);
}

#[test]
fn not_a_test_skip_attribute() {
    #[allow(dead_code)]
    #[derive(StructField)]
    struct Struct {
        field_one: i32,
        #[struct_field(skip)]
        field_two: usize,
    }
    let mut o = Struct {
        field_one: 1,
        field_two: 2,
    };
    o.update_field(StructField::field_one(3));
    if let Some(StructField::field_one(field)) = o.fetch_field("field_one") {
        assert_eq!(field, 3);
    } else {
        unreachable!();
    }
    assert_eq!(o.field_one, 3);
    // Uncommenting the line below should produce an error.
    //o.update_field(StructField::field_two(4));
    //assert_eq!(o.field_two, 4);
}

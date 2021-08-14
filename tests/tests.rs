use struct_field::StructField;

#[test]
fn test_field() {
    #[allow(dead_code)]
    #[derive(StructField)]
    struct Struct {
        field: i32,
    }
    let mut o = Struct { field: 0 };
    o.update_field(StructField::field(3));
    assert_eq!(o.field, 3);
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
    assert_eq!(o.field_one, 3);
    // Uncommenting the line below should produce an error.
    //o.update_field(StructField::field_two(4));
    //assert_eq!(o.field_two, 4);
}

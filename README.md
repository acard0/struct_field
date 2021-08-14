# struct_field

Provides `StructField` derive macro.
```rust
#[derive(StructField)]
struct SomeStruct {
    field_one: i32,
    field_two: Vec<bool>,
}
```
generates
```rust
enum SomeStructField {
    field_one(i32),
    field_two(Vec<bool>),
}
impl SomeStruct {
    pub fn update_field(&mut self, field: SomeStructField) {
        match field {
            SomeStructField::field_one(field_one) => self.field_one = field_one,
            SomeStructField::field_two(field_two) => self.field_two = field_two,
        }
    }
}
```

# Usage
Use the `StructField` derive macro.
```rust
#[derive(StructField)]
struct SomeStruct {
    field_one: i32,
    field_two: Vec<bool>,
}
```

Use `#[struct_field(skip)]` to skip fields.
```rust
#[derive(StructField)]
struct SomeStruct {
	field_one: i32,
	#[struct_field(skip)]
	field_two: Vec<bool>,
}
```

use limesherbet::common::value::Value;
#[repr(u8)]
enum Foo {
    Bar(f64),
}
pub fn test() {
    println!("{}", unsafe { limesherbet::rust_bindings::sum(1, 2) });
    let v = Foo::Bar(1.0);
    let enum_bytes = unsafe { ::std::mem::transmute::<Foo, [u8; 16]>(v) };
    let f64_bytes: [u8; 8] = enum_bytes
        .iter()
        .skip(8)
        .copied()
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    let f64_value = f64::from_ne_bytes(f64_bytes);
    dbg!(f64_value);
}

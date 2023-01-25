use limesherbet::common::value::Value;

pub fn test() {
    let v = Value::Number(1.0);
    let bytes = unsafe { ::std::mem::transmute::<Value, [u8; 16]>(v) };
    bytes.iter().enumerate().for_each(|(i, byte)| if i >= 8 {});
    let f64_bytes: [u8; 8] = bytes[7..15].try_into().unwrap();
    let f64_value = f64::from_be_bytes(f64_bytes);
}

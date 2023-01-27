use limesherbet::common::value::Value;

pub fn test() {
    let a = Value::Boolean(true);
    let b = Value::Number(2.0);
    let c = unsafe { limesherbet::sum(a, b) };
    dbg!(&c);
}

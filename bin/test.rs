use limesherbet::rust_bindings::sum;

pub fn test() {
    let sum = unsafe { sum(1, 2) };
    dbg!(sum);
}

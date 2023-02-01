use macros::native_macro;

use crate::common::{
    natives::Native,
    value::{AsValue, Value},
};

use super::VirtualMachine;
native_macro! {
    debug_stack => Native(debug_stack),
    assert_stack => Native(assert_stack),
    to_str => Native(to_str),
}

pub fn to_str(vm: &mut VirtualMachine, mut args: Vec<Value>) {
    let vm = unsafe { &mut *vm };
    let arg = args.pop().unwrap();
    vm.stack.push(format!("{}", arg).to_value());
}
pub fn debug_stack(vm: &mut VirtualMachine, _: Vec<Value>) {
    let vm = unsafe { &mut *vm };
    println!("Stack: {:?}", vm.stack);
}
pub fn assert_stack(vm: &mut VirtualMachine, mut args: Vec<Value>) {
    let vm = unsafe { &mut *vm };
    println!("stack comparison: {:?} == {:?}", args, &vm.stack);
    assert_eq!(args, vm.stack);
}

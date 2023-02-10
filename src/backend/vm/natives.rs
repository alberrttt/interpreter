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
    let arg = args.pop().unwrap();
    vm.stack.push(arg.to_string().to_value());
}
pub fn debug_stack(vm: &mut VirtualMachine, _: Vec<Value>) {
    let callframe = &vm.callframes[vm.frame_count - 1];
    println!("Stack: {:?}", &vm.stack[callframe.slots + 1..]);
}
pub fn assert_stack(vm: &mut VirtualMachine, args: Vec<Value>) {
    println!("stack comparison: {:?} == {:?}", args, &vm.stack);
    assert_eq!(args, vm.stack);
}

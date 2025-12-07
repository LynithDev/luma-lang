use luma_vm::value::StackValue;

use crate::helpers;

#[test]
pub fn test_fib_sequence() {
    let input = r#"
        fn fib(n: u32): u32 {
            if n <= 1 {
                return n;
            } else {
                return fib(n - 1) + fib(n - 2);
            }
        }

        let result = fib(7);
    "#;

    let program = helpers::compile(input);
    let mut vm = helpers::initialize_vm(program);
    vm.run();

    assert_eq!(vm.ctx.get_local(1), Ok(&StackValue::UInt32(13)))
}
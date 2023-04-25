use std::collections::HashMap;

use banana::vm::{Op, Program, Value, Vm};

fn main() {
    let program = Program {
        constants: vec![Value::Number(2.0), Value::Number(4.0)],
        ops: vec![Op::LoadConstant(0), Op::LoadConstant(1), Op::Add, Op::Print],
    };

    let mut vm = Vm {
        stack: vec![],
        program,
        globals: HashMap::new(),
        ip: 0,
    };

    vm.run_to_back();
}

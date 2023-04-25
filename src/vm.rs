use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Nil => "nil".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Value::Nil => "nil",
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
            Value::String(_) => "string",
        }
    }

    pub fn is_truthy(self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => b,
            Value::Number(n) => n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }
}

#[repr(u8)]
pub enum Op {
    LoadConstant(usize),
    TestNot,
    Jump(isize),
    SetGlobal(usize),
    GetGlobal(usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Print,
}

pub struct Program {
    pub constants: Vec<Value>,
    pub ops: Vec<Op>,
}

pub struct Vm {
    pub stack: Vec<Value>,
    pub program: Program,
    pub globals: HashMap<String, Value>,
    pub ip: usize,
}

impl Vm {
    pub fn run_next(&mut self) {
        match self.program.ops[self.ip] {
            Op::LoadConstant(index) => {
                self.stack.push(self.program.constants[index].clone());
            }
            Op::TestNot => {
                let value = self.stack.pop().unwrap();
                if value.is_truthy() {
                    self.ip += 1;
                }
            }
            Op::Jump(offset) => {
                self.ip = self
                    .ip
                    .checked_add_signed(offset)
                    .expect("Jump out of bounds");
            }
            Op::SetGlobal(index) => {
                let value = self.stack.pop().unwrap();
                let name = self.program.constants[index].to_string();
                self.globals.insert(name, value);
            }
            Op::GetGlobal(index) => {
                let name = self.program.constants[index].to_string();
                let value = self.globals.get(&name).unwrap().clone();
                self.stack.push(value);
            }
            Op::Add => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(match (lhs, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),
                    (Value::String(lhs), rhs) => Value::String(lhs + &rhs.to_string()),
                    (lhs, Value::String(rhs)) => Value::String(lhs.to_string() + &rhs),
                    (lhs, rhs) => panic!(
                        "Cannot add '{:?}' with '{:?}'",
                        lhs.get_type(),
                        rhs.get_type()
                    ),
                });
            }
            Op::Sub => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(match (lhs, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
                    (lhs, rhs) => panic!(
                        "Cannot subtract '{:?}' from '{:?}'",
                        rhs.get_type(),
                        lhs.get_type()
                    ),
                });
            }
            Op::Mul => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(match (lhs, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs * rhs),
                    (lhs, rhs) => panic!(
                        "Cannot multiply '{:?}' with '{:?}'",
                        lhs.get_type(),
                        rhs.get_type()
                    ),
                });
            }
            Op::Div => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(match (lhs, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs / rhs),
                    (lhs, rhs) => panic!(
                        "Cannot divide '{:?}' by '{:?}'",
                        lhs.get_type(),
                        rhs.get_type()
                    ),
                });
            }
            Op::Mod => {
                let rhs = self.stack.pop().unwrap();
                let lhs = self.stack.pop().unwrap();
                self.stack.push(match (lhs, rhs) {
                    (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs % rhs),
                    (lhs, rhs) => panic!(
                        "Cannot modulo '{:?}' by '{:?}'",
                        lhs.get_type(),
                        rhs.get_type()
                    ),
                });
            }
            Op::Print => {
                let value = self.stack.pop().unwrap();
                println!("{:?}", value);
            }
        }
        self.ip += 1;
    }

    pub fn run_to_back(&mut self) {
        while self.ip < self.program.ops.len() {
            self.run_next();
        }
    }
}


/*
 * Implement virtual machine to run instructions.
 */

use std::collections::HashMap;
use std::rc::Rc;

use compiler::bytecode;
use compiler::builtins;
use compiler::pyobject::PyObject;

// use objects::objects;

// Container of the virtual machine state:
pub fn evaluate(code: bytecode::CodeObject) {
  let mut vm = VirtualMachine::new();

  // Register built in function:
  vm.scope.insert(String::from("print"), Rc::new(PyObject::RustFunction{ function: builtins::print }));

  // { stack: Vec::new() };
  for i in code.instructions {
    vm.execute(i);
  }
}

// Objects are live when they are on stack, or referenced by a name (for now)

struct VirtualMachine {
  stack: Vec<Rc<PyObject>>,
  scope: HashMap<String, Rc<PyObject>>,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            scope: HashMap::new(),
        }
    }

    // Execute a single instruction:
    fn execute(&mut self, instruction: bytecode::Instruction) {
      println!("Executing instruction: {:?}", instruction);
      match instruction {
        bytecode::Instruction::LoadStringConstant { value } => {
            let obj = Rc::new(PyObject::String { value });
            self.stack.push(obj.clone()); // Put reference on stack
        },
        bytecode::Instruction::LoadConst { value } => {
            let obj = Rc::new(PyObject::Integer { value });
            self.stack.push(obj.clone());
        },
        bytecode::Instruction::LoadName { name } => {
            // Lookup name in scope and put it onto the stack!
            let obj = &self.scope[&name];
            self.stack.push(obj.clone());
        },
        bytecode::Instruction::Pop => {
            // Pop value from stack and ignore.
            self.stack.pop();
        },
        bytecode::Instruction::ReturnValue => {
            self.stack.pop();
        },
        bytecode::Instruction::CallFunction => {
            let mut args: Vec<Rc<PyObject>> = Vec::new();
            // TODO: take the right number of arguments!
            args.push(self.stack.pop().unwrap());
            let f = self.stack.pop().unwrap();
            f.call(args);
            // call_stack.push();
            // If a builtin function, then call directly, otherwise, execute it?
            // execute(function.code);
        },
        bytecode::Instruction::Break => {
        },
        bytecode::Instruction::Pass => {
            // Ah, this is nice, just relax!
        },
        bytecode::Instruction::Continue => {
        },
      }
    }

}
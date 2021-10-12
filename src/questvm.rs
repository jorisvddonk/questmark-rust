use serde_json;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use tzo;

pub struct QuestVM {
    pub vm: tzo::vm::VM,
    pub responses: HashMap<String, i64>,
}

impl QuestVM {
    pub fn new() -> QuestVM {
        let mut vm: tzo::vm::VM = tzo::vm::VM::new();

        return QuestVM {
            vm: vm,
            responses: HashMap::new(),
        };
    }

    pub fn init(&mut self) {
        pub fn emit(vm: &mut tzo::vm::VM) {
            let z = vm.stack.pop().unwrap();
            println!("\n{}\n", z.to_string());
        }

        /*
        fn response(vm: &mut tzo::vm::VM, s: &mut QuestVM) {
            let a = vm.stack.pop().unwrap().as_number();
            let b = vm.stack.pop().unwrap().as_string();
            s.responses.insert(b, a as i64);
            //println!("\n{} {}\n", b, a);
        }
        */

        self.vm.registerForeignFunction("emit".to_string(), emit);
        /*let callback2 = move || response;
        self.vm
            .registerForeignFunction("response".to_string(), callback2);*/
    }

    pub fn load(&mut self, instructions: std::vec::Vec<serde_json::Value>) {
        self.vm.load(instructions);
    }
}

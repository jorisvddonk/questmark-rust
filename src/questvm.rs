use serde_json;
use std::collections::HashMap;
use std::sync::Mutex;
use tzo;
use tzo::vm::ForeignFunc;

pub struct QuestVM {
    pub vm: tzo::vm::VM,
    pub responses: HashMap<String, i64>,
}

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, String>> = Mutex::new(HashMap::new());
}

impl QuestVM {
    pub fn new() -> QuestVM {
        let vm: tzo::vm::VM = tzo::vm::VM::new();

        return QuestVM {
            vm: vm,
            responses: HashMap::new(),
        };
    }

    fn response(vm: &mut tzo::vm::VM) {
        let a = vm.stack.pop().unwrap().as_number();
        let b = vm.stack.pop().unwrap().as_string();
        HASHMAP.lock().unwrap().insert(a as u32, b);
    }

    fn getresponse(vm: &mut tzo::vm::VM) {
        let question = requestty::Question::select("input").message("Select your response:");
        let mut choices_map: HashMap<usize, u32> = HashMap::new();
        let mut choices: Vec<String> = std::vec::Vec::new();

        let mut i = 0;
        for (k, v) in HASHMAP.lock().unwrap().drain() {
            choices_map.insert(i, k);
            i += 1;
            choices.push(v);
        }

        let question = question.choices(choices);
        let question = question.build();
        let x = requestty::prompt_one(question);
        let input = x.unwrap().as_list_item().unwrap().index;
        let input = choices_map.get(&input).unwrap();

        vm.stack.push(tzo::vm::Value::Number(*input as f64));
    }

    pub fn init(&mut self) {
        pub fn emit(vm: &mut tzo::vm::VM) {
            let z = vm.stack.pop().unwrap();
            println!("\n{}\n", z.to_string());
        }
        let emit_ff = ForeignFunc {
            name: String::from("emit"),
            func: emit,
        };
        self.vm.register_foreign_function(emit_ff);

        let response_ff = ForeignFunc {
            name: String::from("response"),
            func: QuestVM::response,
        };
        self.vm.register_foreign_function(response_ff);

        let getresponse_ff = ForeignFunc {
            name: String::from("getResponse"),
            func: QuestVM::getresponse,
        };
        self.vm.register_foreign_function(getresponse_ff);
    }

    pub fn load(&mut self, instructions: std::vec::Vec<serde_json::Value>) {
        self.vm.load(instructions);
    }
}

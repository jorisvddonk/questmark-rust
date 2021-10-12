#[macro_use]
extern crate lazy_static;

use questvm::QuestVM;
use serde_json;
use std::fs;

mod questvm;

fn main() {
    let contents = fs::read_to_string("test.json").expect("Something went wrong reading the file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();
    let instructions = v
        .as_object()
        .unwrap()
        .get("programList")
        .unwrap()
        .as_array()
        .unwrap()
        .to_vec();

    let label_map = v
        .as_object()
        .unwrap()
        .get("labelMap")
        .unwrap()
        .as_object()
        .unwrap();

    let mut quest_vm: QuestVM = QuestVM::new();

    for e in label_map {
        let k = e.0.as_str().to_string();
        let v = e.1.as_i64().unwrap();
        quest_vm.vm.labels.insert(k, v);
    }
    quest_vm.init();
    quest_vm.load(instructions);
    quest_vm.vm.run();
}

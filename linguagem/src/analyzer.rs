use std::{collections::HashMap, sync::Mutex};

static PROGRAM: &str = "
var a
var a
func f() {
    var a
    var b
}
func g() {
    var b
    c = 0
}
";

let mut function_table: HashMap<String, usize> = HashMap::new();
let mut global_symbol_table: HashMap<String, usize> = HashMap::new();
let mut local_symbol_table: Vec<String> = Vec::new();

fn analyzer(p: &str) {
    let mut memory_address = 0;
    let mut in_function = false;

    for (line_number, line) in p.trim().lines().enumerate() {
        match line.lines() {
            ("var", name) => {

            }
        }
    }
}

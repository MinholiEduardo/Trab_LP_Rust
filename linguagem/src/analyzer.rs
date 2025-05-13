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

lazy_static::lazy_static! {
    static ref FUNCTION_TABLE: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
    static ref GLOBAL_SYMBOL_TABLE: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
    static ref LOCAL_SYMBOL_TABLE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

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

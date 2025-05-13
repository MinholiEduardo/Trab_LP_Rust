use std::collections::HashMap;

fn analyzer(p: &str, 
    function_table: &mut HashMap<String, usize>, 
    global_symbol_table: &mut HashMap<String, usize>, 
    local_symbol_table: &mut Vec<String>) {

    let mut memory_address = 0;
    let mut in_function = false;

    for (line_number, line) in p.trim().lines().enumerate() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        match parts[..] {
            ["var", name] => {
                if global_symbol_table.contains_key(name) || local_symbol_table.contains(&name.to_string()) {
                    println!("variable redefined: {}", name)
                }
                else {
                    if in_function {
                        local_symbol_table.push(name.to_string());
                        memory_address += 1;
                    }
                    else {
                        global_symbol_table.insert(name.to_string(), memory_address);
                        memory_address += 1;
                    }
                }
            }
            [name, "=", number] => {
                if !global_symbol_table.contains_key(name) && !local_symbol_table.contains(&name.to_string()) {
                    println!("variable unknown: {}", name);
                }
            }
            ["func", name, "{"] => {
                if function_table.contains_key(name) {
                    println!("function redefined: {}", name);
                }
                else {
                    function_table.insert(name.to_string(), line_number);
                    in_function = true;
                }
            }
            ["}"] => {
                in_function = false
                for var in local_symbol_table
            }
            _ => {
                println!("Unmatched line: {}", line)
            }
        }
    }
}


fn main() {
    let program: &str = "
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

    analyzer(program, &mut function_table, &mut global_symbol_table, &mut local_symbol_table);
}

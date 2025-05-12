static program = "
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

fn analyzer(program) {
    let mut memory_address = 0;
    let mut in_function = false;

    for (line_number, line) in p.trim().lines().enumerate() {
        match line.split() {
            ("var", name) => 
        }
    }
}
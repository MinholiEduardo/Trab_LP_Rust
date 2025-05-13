use std::collections::HashMap;
fn main() {
    // Código-fonte do programa que será analisado e executado
    let program: &str = "
var a
func f() {
    a = 5
    var b
    b = 6
}
func g() {
    var c
    c = 7
    f()
}
g()
";

    // Tabelas para armazenar informações do analisador

    // Mapeia nome de função para número da linha onde está definida
    let mut function_table: HashMap<String, i32> = HashMap::new();
    // Mapeia variáveis globais para endereços de memória
    let mut global_symbol_table: HashMap<String, i32> = HashMap::new();
    // Lista com nomes de variáveis locais
    let mut local_symbol_table: Vec<String> = Vec::new();

    // Executa a análise sintática e semântica

    //  Exibe o conteúdo das tabelas após a análise
    println!("global_symbol_table: {:?}", global_symbol_table);
    println!("local_symbol_table: {:?}", local_symbol_table);
    println!("function_table: {:?}", function_table);

    // Inicializa as estruturas de execução

    // Pilha de frames de ativação (um por chamada de função)
    let mut activation_frames: Vec<HashMap<String, i32>> = Vec::new();
    // Pilha de posições para retornar após chamadas de função
    let mut call_stack: Vec<i32> = Vec::new();
    // Memória alocada para variáveis globais
    let mut memory: Vec<i32> = Vec::new();
    for _i in 0..global_symbol_table.len(){
        memory.push(0);
    }

    // Executa o programa analisado
    // execute(
    //     program,
    //     &mut function_table,
    //     &mut global_symbol_table,
    //     &mut activation_frames,
    //     &mut call_stack,
    //     &mut memory,
    // )

    // Imprime o estado final das estruturas
    println!("memory: {:?}", memory);
    println!("call_stack: {:?}", call_stack);
    println!("activation_frames: {:?}", activation_frames);

}

/// Função que executa o código, após a análise.
/// A pricípio, o Rust não tem variáveis globais mutáveis acessíveis de forma simples como em outras
/// linguagens — pelo menos, não sem usar construções especiais que envolvem static, Mutex, Arc —
/// pois ele exige que você garanta a segurança ao lidar com threads e dados compartilhados. Nesse
/// sentido, o jeito é passar todas as variáveis inicializadas como parâmetros, de forma que elas
/// podem ser mudadas normalmente sem causar problemas envolvendo acesso à threads.
fn execute(
    program: &str,
    function_table: &mut HashMap<String, i32>,
    global_symbol_table: &mut HashMap<String, i32>,
    activation_frames: &mut Vec<HashMap<String, i32>>,
    call_stack: &mut Vec<i32>,
    memory: &mut Vec<i32>,
) {
    // Aqui vai a lógica da função
    let pc: i32 = 0;
    println!("Executando o program...");
}
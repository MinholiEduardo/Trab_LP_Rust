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
    let mut function_table: HashMap<String, usize> = HashMap::new();
    // Mapeia variáveis globais para endereços de memória
    let mut global_symbol_table: HashMap<String, usize> = HashMap::new();
    // Lista com nomes de variáveis locais
    let mut local_symbol_table: Vec<String> = Vec::new();

    // Executa a análise sintática e semântica

    //  Exibe o conteúdo das tabelas após a análise
    println!("global_symbol_table: {:?}", global_symbol_table);
    println!("local_symbol_table: {:?}", local_symbol_table);
    println!("function_table: {:?}", function_table);

    // Inicializa as estruturas de execução

    // Pilha de frames de ativação (um por chamada de função)
    let mut activation_frames: Vec<HashMap<String, usize>> = Vec::new();
    // Pilha de posições para retornar após chamadas de função
    let mut call_stack: Vec<usize> = Vec::new();
    // Memória alocada para variáveis globais
    let mut memory: Vec<usize> = Vec::new();
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
    function_table: &mut HashMap<String, usize>,
    global_symbol_table: &mut HashMap<String, usize>,
    activation_frames: &mut Vec<HashMap<String, usize>>,
    call_stack: &mut Vec<usize>,
    memory: &mut Vec<usize>,
) {
    // Aqui vai a lógica da função
    let mut pc: usize = 0; // Program counter: aponta para a linha atual do programa
    let lines: Vec<&str> = program.trim().split('\n').collect(); // Divide o programa em linhas
    let _len_lines: usize = lines.len();
    
    while pc < _len_lines {
        let parts: Vec<&str> = lines[pc].split_whitespace().collect();
        match parts[..]{
            // Criação de variável local
            ["var", name] => {
                if !global_symbol_table.contains_key(name) { // Não cria variáveis globais aqui
                    memory.push(0); // Aloca espaço na memória
                    let adress: usize = memory.len() -1;  // Endereço da nova variável
                    let last_index = activation_frames.len() - 1; // Índice do frame de ativação atual
                    activation_frames[last_index].insert(name.to_string(), adress); // # Adiciona ao frame
                    println!("created local {} with adress {}", name, adress);
                }
            }
            // Atribuição de valor
            [name, "=", number] => {
                if global_symbol_table.contains_key(name) { // Se for variável global
                    match global_symbol_table.get(name) { // Burocracia funcional para garantir que está no dicinário
                        Some(v) => { 
                            let adress: usize = *v;
                            match number.parse::<usize>() {
                                Ok(number_usize) => { // Burocracia funcial para garantir a conversão é possível
                                    memory[adress] = number_usize; // Escreve na memória
                                    println!("{} at adress {} receives {}", name, adress, number)
                                }
                                Err(e) => println!("Erro ao converter: {} para usize!", e),
                            }
                        }
                        None => println!("Erro de chave não encontrada!"),
                    }
                    
                } else { // Se for Variável local
                    let last_index = activation_frames.len() - 1; // Índice do frame de ativação atual
                    match activation_frames[last_index].get(name) { // Burocracia funcional para garantir que está no dicinário
                        Some(v) => { 
                            let adress: usize = *v; // Busca no frame atual
                            match number.parse::<usize>() {
                                Ok(number_usize) => { // Burocracia funcial para garantir a conversão é possível
                                    memory[adress] = number_usize; // Escreve na memória
                                    println!("{} at adress {} receives {}", name, adress, number)
                                }
                                Err(e) => println!("Erro ao converter: {} para usize!", e),
                            }
                        }
                        None => println!("Erro de chave não encontrada!"),
                    }

                }
            }
            ["func", _name, "{"] => {
                while lines[pc] != "}"{  // pula as linhas da função
                    pc = pc + 1;
                }
            }
            _ => {
            }
        }
        



    }
}
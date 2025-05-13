import rich  # Biblioteca usada para imprimir mensagens com cores no terminal

''' Example program and grammar

    A variable/function must be defined before being used/called
    Global and local variables must have distinct names
    A function may call itself or other functions
    
    program: (statement | func_def)*
    statement: definition | attribution | func_call
    definition: "var" NAME
    attribution: NAME "=" NUMBER
    func_call: NAME()
    func_def: "func" NAME() "{" statement* "}"

    Example:
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
'''

# Código-fonte do programa que será analisado e executado
program = """
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
"""

# Função que analisa o código para verificar declarações e definições
def analyzer(p):
  memory_adress = 0  # contador de endereços na memória (como um ponteiro de alocação)
  in_function = False  # flag que indica se está dentro de uma função
  for line_number, line in enumerate(p.strip().split('\n')):  # percorre o programa linha a linha, com número da linha
    match line.split():  # divide a linha em partes e faz pattern matching
      case ["var", name]:  # declaração de variável
        if name in global_symbol_table or name in local_symbol_table:
          rich.print('[red1]variable redefined:', name)  # erro: variável já declarada
        else:
          if in_function is True:  # variável local
            local_symbol_table.append(name)
            memory_adress += 1
          else:  # variável global
            global_symbol_table[name] = memory_adress  # mapeia o nome para um endereço
            memory_adress += 1
      case [name, "=", number]:  # atribuição de valor
        if name not in global_symbol_table and name not in local_symbol_table:
          rich.print('[red1]variable unknown:', name)  # erro: variável não declarada
      case ['func', name, '{']:  # início de definição de função
        if name in function_table:
          rich.print('[red1]function redefined:', name)  # erro: função já declarada
        else:
          function_table[name] = line_number  # registra a linha onde a função começa
          in_function = True  # entra no escopo da função
      case ['}']:  # fim de função
        in_function = False  # sai do escopo da função
        for var in local_symbol_table:  # limpa variáveis locais
           rich.print('[red1]clearing local_symbol_table:', var)
        local_symbol_table.clear()
      case [name] if name.endswith('()'):  # chamada de função
        if name not in function_table:
          rich.print('[red1]function unknown:', name)  # erro: função não declarada
      case _:  # linha que não se encaixa em nenhuma regra
        rich.print("[red1]Unmatched:", line)
  print('analysis ended\n')


# Tabelas para armazenar informações do analisador
function_table:dict = {}          # mapeia nome de função para número da linha onde está definida
global_symbol_table:dict = {}    # mapeia variáveis globais para endereços de memória
local_symbol_table:list = []     # lista com nomes de variáveis locais
analyzer(program)  # executa a análise sintática e semântica

# Exibe o conteúdo das tabelas após a análise
print('global_symbol_table:', global_symbol_table)
print('local_symbol_table', local_symbol_table)
print('function_table:', function_table)

# Função que executa o código, após a análise
def execute(p):
    pc = 0  # program counter: aponta para a linha atual do programa
    lines = p.strip().split('\n')  # divide o programa em linhas
    while pc < len(lines):  # executa até o fim do programa
        match lines[pc].split():  # pattern matching por linha
            case ["var", name]:  # criação de variável local
              if name not in global_symbol_table:  # não cria variáveis globais aqui
                memory.append(0)  # aloca espaço na memória
                address = len(memory) -1  # endereço da nova variável
                activation_frames[-1][name] = address  # adiciona ao frame de ativação atual
                print('created local', name, 'with address', address)
            case [name, "=", number]:  # atribuição de valor
                if name in global_symbol_table:  # se for variável global
                  address = global_symbol_table[name]
                  memory[address] = int(number)  # escreve na memória
                  print(name, 'at address', address, 'receives', number)
                else:  # variável local
                  address = activation_frames[-1].get(name)  # busca no frame atual
                  memory[address] = int(number)
                  print(name, 'at address', address, 'receives', number)
            case ["func", name, "{"]:  # definição de função — ignora o corpo
                while lines[pc] != '}':  # pula as linhas da função
                    pc += 1
            case ["}"]:  # fim de função (retorno da chamada)
              print('memory before removal of local variables:', memory)
              print('deleting last activation frame:', activation_frames[-1])
              last_frame = activation_frames.pop()  # remove o frame atual (desempilha)
              local_addresses = sorted(last_frame.values(), reverse=True)  # pega os endereços alocados localmente
              for address in local_addresses:
                memory.pop()  # libera a memória usada pelas variáveis locais
              pc = call_stack.pop()  # volta para a linha após a chamada de função
              print('return to line', pc)
            case [name] if name.endswith('()'):  # chamada de função
              call_stack.append(pc)  # empilha a posição de retorno
              print(name, 'called in line', pc)
              activation_frames.append({})  # cria novo frame de ativação
              pc = function_table[name]  # vai para a linha da função chamada
        pc += 1  # avança para a próxima linha
    print('program ended\n')

# Inicializa as estruturas de execução
activation_frames:list = []  # pilha de frames de ativação (um por chamada de função)
call_stack:list = []         # pilha de posições para retornar após chamadas de função
memory = [0] * len(global_symbol_table)  # memória alocada para variáveis globais
execute(program)  # executa o programa analisado

# Imprime o estado final das estruturas
print('memory:', memory)
print('call_stack:', call_stack)
print('activation_frames:', activation_frames)
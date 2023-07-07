//OLD MAIN

//Camada Model
//Atributos da classe:
struct Carro {
    id: i32,
    placa: String,
    horas: i32,
    preco_hora: f32,
}
//Funções da classe:
impl Carro {
    //Construtor da classe
    fn new(id: i32, placa: String, horas: i32, preco_hora: f32) -> Carro {
        Carro {
            id: id,
            placa: placa,
            horas: horas,
            preco_hora: preco_hora,
        }
    }
    //Calcula o preço total de acordo com a quantidade de horas e o preço da hora
    fn calcular_preco(&self) -> f32 {
        return self.horas as f32 * &self.preco_hora;
    }
    //Formata todos os atributos da classe em uma string
    fn to_string(&self) -> String {
        return format!(
            "ID: {}, Placa: {}, Horas: {}, Preço da Hora: {}, Valor Total: {};",
            &self.id,
            &self.placa,
            &self.horas,
            &self.preco_hora,
            &self.calcular_preco()
        );
    }
    //Formata os atributos da classe em uma string para serem salvos em um arquivo
    fn to_file(&self) -> String {
        return format!("{}\n{}\n{}", &self.placa, &self.horas, &self.preco_hora);
    }
}

//Camada Controller
//Atributos da classe:
struct Controller;
//Funções da classe:
impl Controller {
    //Controle de inserção
    fn insert(lista: &mut Vec<Carro>, placa: String, horas: i32, preco_hora: f32) -> bool {
        //Se não foi informado placa, horas e o preço da hora, o objeto não é inserido na lista
        if placa.len() > 0 && horas > 0 && preco_hora > 0.0 {
            let id: i32 = 1 + lista.len() as i32;
            lista.push(Carro::new(id, placa, horas, preco_hora));
            return true;
        } else {
            return false;
        }
    }
    //Controle de listagem
    fn list(lista: &mut Vec<Carro>) -> bool {
        //Se a lista está vazia, ela não é apresentada
        if lista.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    //Controle de remoção
    fn remove(lista: &mut Vec<Carro>, id: i32) -> bool {
        //Se a lista não está vaiza e foi informado um id válido, encontrada-se a posição do objeto na lista e seus dados são removidos
        if id > 0 && lista.len() > 0 {
            let index = lista
                .iter_mut()
                .position(|x| x.id == id)
                .expect("Nenhum objeto foi encontrado com o ID informado");
            lista[index].placa = format!("REMOVIDO");
            lista[index].horas = 0;
            lista[index].preco_hora = 0.0;
            return true;
        } else {
            return false;
        }
    }
    //Controle de gravação
    fn save(lista: &mut Vec<Carro>) -> bool {
        //Se a lista não está vazia, os objetos são gravados no arquivo lista.txt
        if lista.len() > 0 {
            let mut dados = String::new();
            for elem in lista.iter() {
                dados = format!("{}\n{}", dados, elem.to_file());
            }
            dados = dados[1..dados.len()].to_string();

            fs::write("lista.txt", dados).expect("Erro ao criar o arquivo");

            return true;
        } else {
            return false;
        }
    }
    //Controle de leitura
    fn load(lista: &mut Vec<Carro>) -> bool {
        //Se o arqivo lista.txt existe, lê-se o arquivo e as linhas são separadas em um vetor de strings
        //Do vetor de strings, são extraídos os atributos dos objetos, que são inseridos na lista
        if std::path::Path::new("lista.txt").exists() {
            let dados = fs::read_to_string("lista.txt").expect("Erro ao ler o arquivo");
            let dados = dados.split("\n");
            let mut dados = dados.collect::<Vec<&str>>();

            while dados.len() > 3 {
                let placa: String = dados[0].trim().parse().unwrap();
                let horas: i32 = dados[1].trim().parse().expect("Erro: entrada não é i32");
                let preco_hora: f32 = dados[2].trim().parse().expect("Erro: entrada não é f32");
                if Controller::insert(lista, placa, horas, preco_hora) {
                    println!("Objeto inserido com sucesso!");
                } else {
                    println!("Erro ao inserir objeto!");
                }
                dados = dados[3..dados.len()].to_vec();
            }
            let placa: String = dados[0].trim().parse().unwrap();
            let horas: i32 = dados[1].trim().parse().expect("Erro: entrada não é i32");
            let preco_hora: f32 = dados[2].trim().parse().expect("Erro: entrada não é f32");
            if Controller::insert(lista, placa, horas, preco_hora) {
                println!("Objeto inserido com sucesso!");
            } else {
                println!("Erro ao inserir objeto!");
            }

            return true;
        } else {
            return false;
        }
    }
}

//Camada View
//Atributos da classe:
struct View;
//Funções da classe:
impl View {
    //Interface do menu de seleção
    fn menu(lista: &mut Vec<Carro>) {
        loop {
            println!("----------");

            let mut entrada = String::new();

            println!("Menu Estacionamento: (1 = Inserir, 2 = Listar, 3 = Remover, 4 = Salvar, 5 = Carregar, 6 = Sair)");
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");

            let num: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");
            match num {
                1 => View::insert(lista),
                2 => View::list(lista),
                3 => View::remove(lista),
                4 => View::save(lista),
                5 => View::load(lista),
                6 => break,
                i32::MIN..=0_i32 | 7_i32..=i32::MAX => println!("Comando desconhecido."),
            }
        }
    }
    //Interface de inserção
    fn insert(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Inserir carro.");

        let mut entrada = String::new();

        println!("Placa do carro:");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a linha");
        let placa: String = entrada.trim().parse().unwrap();
        entrada = String::new();

        println!("Quantidade de horas:");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a linha");
        let horas: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");
        entrada = String::new();

        println!("Preço da hora:");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a linha");
        let preco_hora: f32 = entrada.trim().parse().expect("Erro: entrada não é f32");

        if Controller::insert(lista, placa, horas, preco_hora) {
            println!("Objeto inserido com sucesso!");
        } else {
            println!("Erro ao inserir objeto!");
        }
    }
    //Interface de listagem
    fn list(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Listar carros.");

        if Controller::list(lista) {
            for elem in lista.iter() {
                println!("{}", elem.to_string());
            }
        } else {
            println!("A lista de objetos está vazia!");
        }
    }
    //Interface de remoção
    fn remove(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Remover carro.");

        let mut entrada = String::new();

        println!("Informe o ID:");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a linha");
        let id: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");

        if Controller::remove(lista, id) {
            println!("Objeto removido com sucesso!");
        } else {
            println!("Erro ao remover Objeto!");
        }
    }
    //Interface de gravação
    fn save(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Salvar lista.");

        if Controller::save(lista) {
            println!("Lista salva com sucesso!");
        } else {
            println!("Erro ao salvar a lista!");
        }
    }
    //Interface de leitura
    fn load(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Carregar lista.");

        if Controller::load(lista) {
            println!("Lista carregada com sucesso!");
        } else {
            println!("Erro ao carregar a lista!");
        }
    }
}

//Main
//Declarações:
use std::fs;
use std::io;
//Função Main:
fn main() {
    println!("START");

    //Criação da lista de carros que será utilizada ao longo do programa:
    let mut lista: Vec<Carro> = Vec::new();

    //Iniciação da View:
    View::menu(&mut lista);

    println!("END");
}

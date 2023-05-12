//Model Layer
struct Carro {
    id: i32,
    placa: String,
    horas: i32,
    preco_hora: f32,
}
impl Carro {
    fn new(id: i32, placa: String, horas: i32, preco_hora: f32) -> Carro {
        Carro {
            id: id,
            placa: placa,
            horas: horas,
            preco_hora: preco_hora,
        }
    }
    fn calcular_preco(&self) -> f32 {
        return self.horas as f32 * &self.preco_hora;
    }
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
    fn to_file(&self) -> String {
        return format!("{}\n{}\n{}", &self.placa, &self.horas, &self.preco_hora);
    }
}

//Controller Layer
struct Controller;
impl Controller {
    fn insert(lista: &mut Vec<Carro>, placa: String, horas: i32, preco_hora: f32) -> bool {
        if placa.len() > 0 && horas > 0 && preco_hora > 0.0 {
            let id: i32 = 1 + lista.len() as i32;
            lista.push(Carro::new(id, placa, horas, preco_hora));
            return true;
        } else {
            return false;
        }
    }
    fn list(lista: &mut Vec<Carro>) -> bool {
        if lista.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    fn remove(lista: &mut Vec<Carro>, id: i32) -> bool {
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
    fn save(lista: &mut Vec<Carro>) -> bool {
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
    fn load(lista: &mut Vec<Carro>) -> bool {
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

//View Layer
struct View;
impl View {
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
    fn save(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Salvar lista.");

        if Controller::save(lista) {
            println!("Lista salva com sucesso!");
        } else {
            println!("Erro ao salvar a lista!");
        }
    }
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
use std::fs;
use std::io;
fn main() {
    println!("START");

    let mut lista: Vec<Carro> = Vec::new();

    View::menu(&mut lista);

    println!("END");
}

//Declarações do RPC:
use tonic::{transport::Server, Request, Response, Status};

use procedures::estacionamento_server::{Estacionamento, EstacionamentoServer};
use procedures::{EstacionamentoResponse, EstacionamentoInsert, EstacionamentoList, EstacionamentoRemove, EstacionamentoSave};

pub mod procedures {
    tonic::include_proto!("procedures");
}

//Criação de um Singleton global mutável para a lista de objetos
use std::sync::{OnceLock, Mutex};
fn lista() -> &'static Mutex<Vec<Carro>> {
    static ARRAY: OnceLock<Mutex<Vec<Carro>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(vec![]))
}

//Classe EstacionamentoService, onde ficam as respostas do servidor e o Controller
//Atributos da classe:
#[derive(Debug, Default)]
pub struct EstacionamentoService {}
//Funções da classe:
#[tonic::async_trait]
impl Estacionamento for EstacionamentoService {
    //Função de resposta para inserção
    async fn send_insert(
        &self,
        request: Request<EstacionamentoInsert>,
    ) -> Result<Response<EstacionamentoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let mut sucesso: bool = false;
        let resultado: String;

        //Se não foi informado placa, horas e o preço da hora, o objeto não é inserido na lista
        if req.placa.len() > 0 && req.horas > 0 && req.preco_hora > 0.0 {
            let id: i32 = 1 + lista().lock().unwrap().len() as i32;
            lista().lock().unwrap().push(Carro::new(id, req.placa, req.horas, req.preco_hora));
            sucesso = true;
            resultado = format!("Objeto inserido no sistema!");
        } else {
            resultado = format!("Os dados informados são inválidos!");
        }

        let reply = EstacionamentoResponse {
            successful: sucesso,
            message: format!("{}", resultado),
        };

        Ok(Response::new(reply))
    }
    //Função de resposta para listagem
    async fn send_list(
        &self,
        request: Request<EstacionamentoList>,
    ) -> Result<Response<EstacionamentoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut sucesso: bool = false;
        let mut resultado: String = format!("");

        //Se a lista não estiver vazia, são retornados os objetos dela
        if lista().lock().unwrap().len() > 0 {
            for elem in lista().lock().unwrap().iter() {
                resultado = format!("{}{}", resultado, elem.to_string());
            }
            resultado = format!("Lista: {}", resultado);
            sucesso = true;
        } else {
            resultado = format!("Não há objetos registrados no sistema!");
        }

        let reply = EstacionamentoResponse {
            successful: sucesso,
            message: format!("{}", resultado),
        };

        Ok(Response::new(reply))
    }
    //Função de resposta para remoção
    async fn send_remove(
        &self,
        request: Request<EstacionamentoRemove>,
    ) -> Result<Response<EstacionamentoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();
        let mut sucesso: bool = false;
        let resultado: String;

        //Se a lista não está vazia e foi informado um id válido, encontrada-se a posição do objeto na lista e seus dados são removidos
        if req.id > 0 && lista().lock().unwrap().len() > 0 {
            let mut found: bool = false;
            for elem in lista().lock().unwrap().iter() {
                if elem.id == req.id {
                    found = true;
                    break;
                }
            }
            if found {
                let index = lista().lock().unwrap()
                    .iter_mut()
                    .position(|x| x.id == req.id)
                    .expect("Nenhum objeto foi encontrado com o ID informado");
                lista().lock().unwrap().remove(index);
                resultado = format!("Objeto com o ID informado foi removido do sistema!");
                sucesso = true;
            } else {
                resultado = format!("Não há objetos registrados no sistema com o ID informado!");
            }
        } else {
            resultado = format!("Não há objetos registrados no sistema com o ID informado!");
        }

        let reply = EstacionamentoResponse {
            successful: sucesso,
            message: format!("{}", resultado),
        };

        Ok(Response::new(reply))
    }
    //Função de resposta para gravação
    async fn send_save(
        &self,
        request: Request<EstacionamentoSave>,
    ) -> Result<Response<EstacionamentoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut sucesso: bool = false;
        let resultado: String;

        //Se a lista não está vazia, os objetos são gravados no arquivo lista.txt
        if lista().lock().unwrap().len() > 0 {
            let mut dados = String::new();
            for elem in lista().lock().unwrap().iter() {
                dados = format!("{}\n{}", dados, elem.to_file());
            }
            dados = dados[1..dados.len()].to_string();

            fs::write("lista.txt", dados).expect("Erro ao criar o arquivo");

            sucesso = true;
            resultado = format!("Objetos gravados no sistema!");
        } else {
            resultado = format!("Não há objetos registrados no sistema!");
        }

        let reply = EstacionamentoResponse {
            successful: sucesso,
            message: format!("{}", resultado),
        };

        Ok(Response::new(reply))
    }
}

//Main
use std::fs;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Definição do endereço:
    let addr = "0.0.0.0:50051".parse()?;
    //Instanciação da classe EstacionamentoService:
    let estacionamento_service = EstacionamentoService::default();
    //Carregamento do arquivo lista.txt:
    //Se o arqivo lista.txt existe, lê-se o arquivo e as linhas são separadas em um vetor de strings
    //Do vetor de strings, são extraídos os atributos dos objetos, que são inseridos na lista
    if std::path::Path::new("lista.txt").exists() {
        let dados = fs::read_to_string("lista.txt").expect("Erro ao ler o arquivo");
        let dados = dados.split("\n");
        let mut dados = dados.collect::<Vec<&str>>();

        while dados.len() > 0 {
            let placa: String = dados[0].trim().parse().unwrap();
            let horas: i32 = dados[1].trim().parse().expect("Erro: entrada não é i32");
            let preco_hora: f32 = dados[2].trim().parse().expect("Erro: entrada não é f32");
            let id: i32 = 1 + lista().lock().unwrap().len() as i32;

            lista().lock().unwrap().push(Carro::new(id, placa, horas, preco_hora));

            if dados.len() > 3 {
                dados = dados[3..dados.len()].to_vec();
            } else {
                break;
            }
        }
    }

    //Inicialização do servidor
    Server::builder()
        .add_service(EstacionamentoServer::new(estacionamento_service))
        .serve(addr)
        .await?;

    Ok(())
}

//Model
//Atributos da classe:
#[derive(Debug)]
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

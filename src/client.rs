//Declarações do RPC:
use procedures::estacionamento_client::EstacionamentoClient;
use procedures::{EstacionamentoInsert, EstacionamentoList, EstacionamentoRemove, EstacionamentoSave};

pub mod procedures {
    tonic::include_proto!("procedures");
}

//Main
use std::io;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Realiza a conexão com o servidor:
    let mut client = EstacionamentoClient::connect(
        "https://0.0.0.0:50051"
    ).await?;

    //View
    loop {
        println!("----------");

        let mut entrada = String::new();

        println!("Menu Estacionamento: (1 = Inserir, 2 = Listar, 3 = Remover, 4 = Salvar e Sair)");
        io::stdin()
            .read_line(&mut entrada)
            .expect("Erro ao ler a linha");

        let num: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");
        if num == 1 {
            println!("----------");
            println!("Inserir carro.");
    
            let mut entrada = String::new();
    
            println!("Placa do carro:");
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");
            let obj_placa: String = entrada.trim().parse().unwrap();
            entrada = String::new();
    
            println!("Quantidade de horas:");
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");
            let obj_horas: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");
            entrada = String::new();
    
            println!("Preço da hora:");
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");
            let obj_preco_hora: f32 = entrada.trim().parse().expect("Erro: entrada não é f32");
    
            let request = tonic::Request::new(
                EstacionamentoInsert {
                    placa: obj_placa,
                    horas: obj_horas,
                    preco_hora: obj_preco_hora,
                }
            );
            let response = client.send_insert(request).await?;
            println!("RESPOSTA={:?}", response);
        } else if num == 2 {
            println!("----------");
            println!("Listar carros.");
    
            let request = tonic::Request::new(
                EstacionamentoList {}
            );
            let response = client.send_list(request).await?;
            println!("RESPOSTA={:?}", response);
        } else if num == 3 {
            println!("----------");
            println!("Remover carro.");
    
            let mut entrada = String::new();
    
            println!("Informe o ID:");
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");
            let obj_id: i32 = entrada.trim().parse().expect("Erro: entrada não é i32");
    
            let request = tonic::Request::new(
                EstacionamentoRemove {
                    id: obj_id,
                }
            );
            let response = client.send_remove(request).await?;
            println!("RESPOSTA={:?}", response);
        } else if num == 4 {
            println!("----------");
            println!("Salvar lista.");

            let request = tonic::Request::new(
                EstacionamentoSave {}
            );
            let response = client.send_save(request).await?;
            println!("RESPOSTA={:?}", response);
            break;
        } else {
            println!("Comando desconhecido.");
        }
    }

    Ok(())
}

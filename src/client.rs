use procedures::estacionamento_client::EstacionamentoClient;
use procedures::EstacionamentoRequest;

pub mod procedures {
    tonic::include_proto!("procedures");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = EstacionamentoClient::connect(
        "https://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        EstacionamentoRequest {
            from_addr: "123456".to_owned(),
            to_addr: "654321".to_owned(),
            code: 404,
        }
    );

    let response = client.send_request(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
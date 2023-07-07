//Função para compilar o arquivo procedures.proto
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/procedures.proto")?;
    Ok(())
}
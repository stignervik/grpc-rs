use unit::unit_client::UnitClient;
use unit::CreateUnitRequest;

pub mod unit {
    tonic::include_proto!("unit");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    let mut client = UnitClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(CreateUnitRequest {
        name: "Unit 1".into(),
        unit_class: "Unit class".into(),
        unit_function: "Unit function".into(),
        function_block: "Function block".into(),
    });

    let response = client.create_unit(request).await?;

    println!("RESPONSE Unit Create={:?}", response);

    Ok(())
}

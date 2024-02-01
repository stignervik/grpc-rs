use tonic::{transport::Server, Request, Response, Status};

use unit::unit_server::{Unit, UnitServer};
use unit::{CreateUnitReply, CreateUnitRequest};

pub mod unit {
    tonic::include_proto!("unit");
}

#[derive(Debug, Default)]
pub struct MyUnit {}

#[tonic::async_trait]
impl Unit for MyUnit {
    async fn create_unit(
        &self,
        request: Request<CreateUnitRequest>,
    ) -> Result<Response<CreateUnitReply>, Status> {
        println!("Got a unit request: {:?}", request);
        let request = request.into_inner();

        let reply = unit::CreateUnitReply {
            name: format!("Hello {}!", request.name).into(),
            unit_class: format!("Hello {}!", request.unit_class).into(),
            unit_function: format!("Hello {}!", request.unit_function).into(),
            function_block: format!("Hello {}!", request.function_block).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let unit = MyUnit::default();

    Server::builder()
        .add_service(UnitServer::new(unit))
        .serve(addr)
        .await?;

    Ok(())
}

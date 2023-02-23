use crate::processors::grpc_expression_processor_client::GrpcExpressionProcessorClient;
use crate::processors::{GrpcExpressionProcessingRequest, GrpcExpressionProcessingResponse};

pub mod processors {
    tonic::include_proto!("processors");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GrpcExpressionProcessorClient::connect(
      "http://[::1]:50051"
    ).await?;

    let request = tonic::Request::new(
        GrpcExpressionProcessingRequest {
            expression: "11+22*33+(44-55)/66".to_owned()
        }
    );

    println!("Send {request:?}");
    let response = client.process(request).await?;
    println!("Received {response:?}");

    Ok(())
    //let mut args: Args = args();

    // let last_arg = args.nth(1).unwrap();
    // let op_arg = args.nth(0).unwrap();
    // let second_arg = args.nth(0).unwrap();
    //
    // let first = first_arg.parse::<i16>().unwrap();
    // let second = second_arg.parse::<i16>().unwrap();
    // let op = op_arg.chars().next().unwrap();

    //println!("{first_arg} {op_arg} {second_arg} = {}", calculate(first, second, op));

    // 11+22*33+(44-55)/66

}
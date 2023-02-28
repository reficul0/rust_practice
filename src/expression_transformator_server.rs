mod expression_processors;
use expression_processors::{ExpressionProcessor, PostfixReversePolishNotation};

use tonic::{transport::Server, Request, Response, Status, IntoRequest};
use crate::processors::grpc_expression_processor_server::{GrpcExpressionProcessor, GrpcExpressionProcessorServer};
use crate::processors::{GrpcExpressionProcessingRequest, GrpcExpressionProcessingResponse};

pub mod processors {
    tonic::include_proto!("processors");
}

#[derive(Debug, Default)]
pub struct GrpcExpressionProcessorService {
}

#[tonic::async_trait]
impl GrpcExpressionProcessor for GrpcExpressionProcessorService {
    async fn process(
        &self,
        request: Request<GrpcExpressionProcessingRequest>
    ) -> Result<Response<GrpcExpressionProcessingResponse>, Status> {
        println!("Received {request:?}");
        let processor = PostfixReversePolishNotation::new();

        let processed_expr = processor.process(&request.into_inner().expression);
        let response = GrpcExpressionProcessingResponse {
            expression: processed_expr
        };
        println!("Send {response:?}");

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let processor_service = GrpcExpressionProcessorService::default();
    Server::builder()
        .add_service(GrpcExpressionProcessorServer::new(processor_service))
        .serve(addr)
        .await?;

    Ok(())
}

fn calculate(first_operand: i16, second_operand: i16, operator: char) -> i16 {
    match operator {
        '+' => first_operand + second_operand,
        '-' => first_operand - second_operand,
        '*' | 'x' => first_operand * second_operand,
        '/' | ':' => first_operand / second_operand,
        _ => panic!("{operator} is unsupported")
    }
}
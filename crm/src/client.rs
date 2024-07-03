use anyhow::Result;
use tonic::Request;

use crm::pb::{user_service_client::UserServiceClient, CreateUserRequest};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;

    let request = Request::new(CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@acme.org".to_string(),
    });

    let response = client.create_user(request).await?.into_inner();

    println!("Response: {:?}", response);

    Ok(())
}
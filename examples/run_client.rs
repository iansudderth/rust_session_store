use mini_redis::{client, Result};


// TODO: Learn Tokio

#[tokio::main]
pub async fn main() -> Result<()>{
   let mut client = client::connect("127.0.0.1:30006").await?;

    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;

    println!("got value :: {:?}", result);

    Ok(())

}
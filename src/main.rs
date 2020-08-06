use tokio_postgres::{NoTls, Error};
use tokio::prelude::*;
use postgres_types::FromSql;


#[tokio::main]
async fn main() -> Result<(), Error> {

    println!("connecting");

    let (client, connection) =
        tokio_postgres::connect(
            "host=192.168.99.104 port=30004 user=postgres password=postgres",
            NoTls)
            .await?;

    println!("connected");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });


    println!("querrying");

    let rows = client.query("SELECT * FROM public.stuff", &[]).await?;

    println!("querried");

    let key: i32 = rows[0].get(0);
    let value: &str = rows[0].get(1);
    println!("{} : {}", key, value);

    Ok(())
}

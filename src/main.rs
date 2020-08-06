use tokio_postgres::{NoTls, Error, AsyncMessage, Connection};
use tokio::prelude::*;
use postgres_types::FromSql;
use tokio::{spawn};
use futures::{future, stream, TryStreamExt, StreamExt, FutureExt};
use futures::channel::mpsc;


// TODO : Figure out what the fuck is going on with all of this
#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("connecting");

    let (client, mut connection) =
        tokio_postgres::connect(
            "host=192.168.99.104 port=30004 user=postgres password=postgres",
            NoTls)
            .await?;

    println!("connected");

    let (tx, rx) = mpsc::unbounded();
    let stream = stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| panic!(e));

    let connection = stream.forward(tx).map(|r| r.unwrap());
    spawn(connection);

    println!("query");
    client.batch_execute(
        "LISTEN db_notifications;").await.unwrap();

    println!("querried");

    let notifications = rx.filter_map(|m| match m {
        AsyncMessage::Notification(n) => {
            println!("{:?}", n);
            future::ready(Some(n))
        }
        _ => future::ready(None)
    })
        .collect::<Vec<_>>()
        .await;

    for n in notifications {
        println!("{:?}", n)
    }

    Ok(())
}

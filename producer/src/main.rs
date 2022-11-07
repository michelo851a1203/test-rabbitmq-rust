use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://user:secret@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);
    exchange.publish(Publish::new( "hello world".as_bytes(), "main_queue"))?;
    connection.close()
}

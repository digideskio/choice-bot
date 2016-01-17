extern crate amqp;

use amqp::protocol::basic::{BasicProperties, Deliver};
use amqp::{Basic, Channel, ConsumerCallBackFn, Session, Table};
use std::thread;

fn main() {
    let url = "amqp://localhost//";
    let mut session = match Session::open_url(url) {
        Ok(session) => session,
        Err(e) => panic!("Session creation failed: {:?}", e)
    };
    let mut channel = session.open_channel(1).ok().expect("channel creation failed");
    // exchange: S, _type: S, passive: bool,
    // durable: bool, auto_delete: bool, internal: bool, nowait: bool,
    // arguments: Table
    channel.exchange_declare("inbound_hooks", "fanout", false,
                             false, true, false, false,
                             Table::new()).ok().expect("exchange declairation failed");
    let queue = channel.queue_declare("", false, false, true, false, false,
                                      Table::new()).ok().expect("queue declairation failed");
    channel.queue_bind(&queue.queue as &str, "inbound_hooks", "", false,
                       Table::new()).ok().expect("queue bind failed");
    let consumer_name = channel.basic_consume(consumer as ConsumerCallBackFn,
                                              &queue.queue as &str, "", false,
                                              false, false, false,
                                              Table::new());

    let consumers_thread = thread::spawn(move || {
        channel.start_consuming();
        channel
    });
    consumers_thread.join().unwrap();
}

//type ConsumerCallBackFn = fn(channel: &mut Channel, method: Deliver, headers: BasicProperties, body: Vec<u8>);

fn consumer(channel: &mut Channel, deliver: Deliver, headers: BasicProperties, body: Vec<u8>) {
    println!("Got message");
    channel.basic_ack(deliver.delivery_tag, false).unwrap();
}
// token=XXXXXXXXXXXXXXXXXX
// team_id=T0001
// team_domain=example
// channel_id=C2147483705
// channel_name=test
// timestamp=1355517523.000005
// user_id=U2147483697
// user_name=Steve
// text=googlebot: What is the air-speed velocity of an unladen swallow?
// trigger_word=googlebot:

// struct OutboundMessage {
//     username: String,
//     icon_emoji: String,
//     text: String,
//     channel: String
// }

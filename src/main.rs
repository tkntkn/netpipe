mod receiver;
use broker::{Broker, StdoutBroker, UdpBroker, WebSocketBroker};
use receiver::{
    ReceiverCreator, StdinReceiverCreator, UdpReceiverCreator, WebSocketReceiverCreator,
};
mod broker;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_option = &args[1];
    let out_options = &args[2..];

    let receiver_creators: Vec<Box<dyn ReceiverCreator>> = vec![
        Box::new(StdinReceiverCreator),
        Box::new(WebSocketReceiverCreator),
        Box::new(UdpReceiverCreator),
    ];

    let mut receiver_creators = receiver_creators.iter();
    let creator = receiver_creators.find(|c| c.matches(in_option)).unwrap();
    let receiver = creator.create_receiver(in_option);

    let brokers: Vec<Box<dyn Broker>> = vec![
        Box::new(StdoutBroker::new()),
        Box::new(WebSocketBroker::new()),
        Box::new(UdpBroker::new()),
    ];

    for option in out_options {
        let broker = brokers.iter().find(|c| c.matches(option)).unwrap();
        broker.add_destination(option);
    }

    for message in receiver {
        for broker in brokers.iter() {
            broker.send(&message);
        }
    }
}

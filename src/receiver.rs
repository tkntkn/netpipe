use std::{io::stdin, net::UdpSocket, sync::mpsc, thread};
use tungstenite::connect;
use url::Url;

pub trait ReceiverCreator {
    fn matches(&self, option: &String) -> bool;
    fn create_receiver(&self, option: &String) -> Box<dyn Iterator<Item = String>>;
}

pub struct StdinReceiverCreator;
impl ReceiverCreator for StdinReceiverCreator {
    fn matches(&self, option: &String) -> bool {
        return option.eq("stdin");
    }

    fn create_receiver(&self, _option: &String) -> Box<dyn Iterator<Item = String>> {
        return Box::new(stdin().lines().into_iter().map(|l| l.unwrap()));
    }
}

pub struct WebSocketReceiverCreator;
impl ReceiverCreator for WebSocketReceiverCreator {
    fn matches(&self, option: &String) -> bool {
        return option.starts_with("ws://");
    }

    fn create_receiver(&self, option: &String) -> Box<dyn Iterator<Item = String>> {
        let (mut socket, _) = connect(Url::parse(&option).unwrap()).unwrap();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            let message = socket.read_message().unwrap();
            tx.send(message.into_text().unwrap()).unwrap();
        });
        return Box::new(rx.into_iter());
    }
}

pub struct UdpReceiverCreator;
impl ReceiverCreator for UdpReceiverCreator {
    fn matches(&self, _option: &String) -> bool {
        return true;
    }

    fn create_receiver(&self, option: &String) -> Box<dyn Iterator<Item = String>> {
        let socket = UdpSocket::bind(option).unwrap();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || loop {
            let mut buf = [0; 8192];
            let buf_size = socket.recv(&mut buf).unwrap();
            let buf = &buf[..buf_size];
            tx.send(String::from_utf8(buf.to_vec()).unwrap()).unwrap();
        });
        return Box::new(rx.into_iter());
    }
}

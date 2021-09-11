use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use bytes::Bytes;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;

#[derive(Default)]
pub struct Broadcaster {
    clients: Mutex<Vec<Sender<Bytes>>>,
}

enum Message {
    Connected,
    Ping,
}

impl From<Message> for Bytes {
    fn from(msg: Message) -> Self {
        match msg {
            Message::Connected => Bytes::from_static(b"data: connected\n\n"),
            Message::Ping => Bytes::from_static(b":\n\n"),
        }
    }
}

impl Broadcaster {
    pub fn new() -> Arc<Broadcaster> {
        let broadcaster = Arc::new(Broadcaster {
            clients: Mutex::new(Vec::new()),
        });

        // Ping clients every 15 seconds to check if they're still connected
        tokio::spawn({
            let broadcaster = broadcaster.clone();
            async move {
                let mut interval = tokio::time::interval(Duration::from_secs(15));
                loop {
                    interval.tick().await;
                    broadcaster.remove_disconnected_clients();
                }
            }
        });

        broadcaster
    }

    fn remove_disconnected_clients(&self) {
        let mut clients = self.clients.lock().unwrap();
        let connected_clients = clients.iter().filter_map(|client| {
            client
                .try_send(Message::Ping.into())
                .ok()
                .map(|_| client.clone())
        });

        *clients = connected_clients.collect();
    }

    pub async fn new_client(&self) -> impl Stream<Item = Result<Bytes, Infallible>> {
        let (tx, rx) = mpsc::channel(100);
        tx.try_send(Message::Connected.into()).unwrap();
        self.clients.lock().unwrap().push(tx);
        ReceiverStream::new(rx).map(Ok)
    }

    pub fn send(&self, message: impl Into<Bytes>) {
        let bytes = message.into();
        self.clients.lock().unwrap().iter().for_each(|client| {
            client.try_send(bytes.clone()).unwrap_or(());
        });
    }
}

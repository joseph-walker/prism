use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    response::Html,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::{
    net::TcpListener,
    sync::broadcast::{channel, Receiver},
};

struct AppState {
    buffer: Arc<Mutex<Vec<Box<str>>>>,
    rx_line: Receiver<Box<str>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            rx_line: self.rx_line.resubscribe(),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel::<Box<str>>(256);

    let app_state = AppState {
        buffer: Arc::new(Mutex::new(vec![])),
        rx_line: rx,
    };

    let buffer = app_state.buffer.clone();

    thread::spawn(move || {
        let stdin = io::stdin();

        let buffer = buffer.clone();

        for line in stdin.lock().lines() {
            let line = line.expect("Error: Could not read line from stdin");

            let entry: Box<str> = line.into();

            tx.send(entry.clone()).unwrap();

            // TODO: Add max buffer capacity so it doesn't eat memory
            // until the end of time
            buffer.lock().unwrap().push(entry);
        }
    });

    let app = Router::new()
        .route("/", get(app_handler))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn app_handler() -> Html<&'static str> {
    let app_content = include_str!("../client/dist/index.html");
    Html(app_content)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, mut state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Dump the existing buffer to the client when they connect
    {
        let buffer_cat = state.buffer.lock().unwrap().join("\n");
        let _ = sender.send(Message::Text(buffer_cat)).await;
    }

    'listen: loop {
        tokio::select! {
            client_msg = receiver.next() => {
                if let Some(client_msg) = client_msg {
                    let _msg = if let Ok(client_msg) = client_msg {
                        client_msg
                    } else {
                        // Client errored
                        break 'listen;
                    };
                } else {
                    // Client disconnected
                    break 'listen;
                }
            },
            line = state.rx_line.recv() => {
                let _ = sender.send(Message::Text(line.unwrap().to_string())).await;
            }
        };
    }
}

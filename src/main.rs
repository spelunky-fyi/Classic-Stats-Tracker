use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures::{sink::SinkExt, stream::StreamExt};
use hyper::header::CONTENT_TYPE;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper_tungstenite::{tungstenite, HyperWebsocket};
use static_files::Resource;
use tokio::select;
use tokio::sync::broadcast::{channel, Receiver, Sender};

use classic_trackers::mem_reader::{run_forever, TrackerMessage};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

struct Tracker {
    static_assets: Arc<HashMap<&'static str, Resource>>,
    stats_tx: Sender<TrackerMessage>,
}

impl Service<Request<Body>> for Tracker {
    type Response = Response<Body>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let path = req.uri().path();
        let mut static_key = &path[1..path.len()];
        if static_key.is_empty() {
            static_key = "index.html";
        }

        // Are you a static request?
        if let Some(resource) = self.static_assets.get(static_key) {
            let response = Response::builder()
                .header(CONTENT_TYPE, resource.mime_type)
                .status(StatusCode::OK)
                .body(Body::from(resource.data));
            return Box::pin(async { response });
        }

        // Handle Websockets
        if path == "/ws/" && hyper_tungstenite::is_upgrade_request(&req) {
            let (response, websocket) = match hyper_tungstenite::upgrade(&mut req, None) {
                Ok((response, websocket)) => (response, websocket),
                Err(_) => {
                    return Box::pin(async {
                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(Body::from("Failed to upgrade request"))
                    });
                }
            };

            let stats_rx = self.stats_tx.subscribe();
            // Spawn a task to handle the websocket connection.
            tokio::spawn(async move {
                if let Err(e) = serve_websocket(websocket, stats_rx).await {
                    eprintln!("Error in websocket connection: {}", e);
                }
            });

            // Return the response so the spawned future can continue.
            return Box::pin(async { Ok(response) });
        }

        let res = Ok(Response::new(Body::from("There's nothing here!")));
        Box::pin(async { res })
    }
}

/// Handle a websocket connection.
async fn serve_websocket(
    websocket: HyperWebsocket,
    mut stats_rx: Receiver<TrackerMessage>,
) -> Result<(), anyhow::Error> {
    let mut websocket = websocket.await?;

    loop {
        select! {
            val = websocket.next() => {
                match val {
                    Some(msg) => {
                        match msg? {
                            // Not currently doing anything with
                            tungstenite::Message::Text(msg) => {
                                websocket.send(tungstenite::Message::text(msg)).await?;
                            }
                            tungstenite::Message::Binary(_msg) => {}
                            tungstenite::Message::Ping(_msg) => {}
                            tungstenite::Message::Pong(_msg) => {}
                            tungstenite::Message::Close(_msg) => {}
                            tungstenite::Message::Frame(_msg) => {
                                unreachable!();
                            }
                        }
                    },
                    None => {
                        break
                    }
                }
            }
            val = stats_rx.recv() => {
                match &val {
                    Ok(msg) => {
                        match serde_json::to_string(msg) {
                            Err(err) => {
                                println!("Failed to understand message: {}", err);
                            },
                            Ok(ser_msg) => websocket.send(tungstenite::Message::text(ser_msg)).await?,
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to receive message: {}", err);
                    }
                }
            }
        }
    }

    Ok(())
}

struct MakeSvc {
    static_assets: Arc<HashMap<&'static str, Resource>>,
    stats_tx: Sender<TrackerMessage>,
}

impl<T> Service<T> for MakeSvc {
    type Response = Tracker;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let assets = self.static_assets.clone();

        let stats_tx = self.stats_tx.clone();
        let fut = async move {
            Ok(Tracker {
                static_assets: assets,
                stats_tx,
            })
        };
        Box::pin(fut)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (tx, _) = channel::<TrackerMessage>(1);
    let stats_tx = tx.clone();
    std::thread::spawn(|| run_forever(stats_tx));
    let addr = ([127, 0, 0, 1], 4224).into();
    let service = MakeSvc {
        static_assets: Arc::new(generate()),
        stats_tx: tx,
    };

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

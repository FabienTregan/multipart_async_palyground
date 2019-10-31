use httparse::Error::Status;
use hyper::rt::{self, Future};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode, Method, Chunk};
use std::net::TcpStream;
use std::convert::Infallible;
use futures::{FutureExt};

use futures::Stream;
use futures_util::stream::StreamExt;

use multipart_async::server::Multipart;
use multipart_async::BodyChunk;
use futures_util::try_stream::TryStreamExt;

use bytes::Bytes;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        println!("\n\nrequest from: {}", socket.remote_addr());

//        async move { Ok::<_, Error>(service_fn(handle_request)) }
        async {
            Ok::<_, Infallible>(service_fn(echo))
        }
    });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_svc);

    println!("server running on {}", server.local_addr());

    if let Err(e) = server.await {
        eprintln!("an error occurred: {}", e);
    }
}

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/image") => {

                let toto = req.into_body().try_concat().await;

                let bytes_request = req
                    .map(|b| {
                        b.map(|cr| {
                            match cr {
                                Ok(c) => Ok(c.into_bytes()),
                                Err(e) => Err(e),
                            }
                        })
                    });

                match Multipart::try_from_request(bytes_request) {
                Ok(multipart) => {
                    *response.body_mut() = Body::from("Got something multipart");
                }
                _ => *response.body_mut() = Body::from("Try POSTing multipart to /image")
            };

        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

/*
async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    Ok(match Multipart::try_from_request(req) {
        Ok(multipart) => match handle_multipart(multipart).await {
            Ok(()) => Response::new(Body::from("successful request!")),
            Err(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))?,
        },
        Err(req) => Response::new(Body::from("expecting multipart/form-data")),
    })
}

async fn handle_multipart(mut multipart: Multipart<Body>) -> Result<(), Error> {
    while let Some(mut field) = multipart.next_field().await? {
        println!("got field: {:?}", field.headers);

        if field.headers.is_text() {
            println!("field text: {:?}", field.data.read_to_string().await?);
        } else {
            while let Some(chunk) = field.data.try_next().await? {
                println!("got field chunk, len: {:?}", chunk.len());
            }
        }
    }

    Ok(())
}*/

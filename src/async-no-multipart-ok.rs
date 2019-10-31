use {
    futures::executor::block_on,
    hyper::{
        service::{make_service_fn, service_fn},
        Body, Method, Request, Response, Server, StatusCode,
    },
    std::convert::Infallible,
};


async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
}

async fn run_server() {
    let addr = ([127, 0, 0, 1], 3000).into();

    let make_svc = make_service_fn(|_| {
        async {
            Ok::<_, Infallible>(service_fn(echo))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(err) = server.await {
        eprintln!("server error: {}", err);
    }
}

#[tokio::main]
async fn main() {
    block_on(run_server());
}

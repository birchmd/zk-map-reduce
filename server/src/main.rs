use {
    client_route::ClientFiles,
    std::net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    warp::{http::Response, reply::Reply, Filter},
    zkmr_types::Submission,
};

mod client_route;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080));

    // Serve files for client
    let ClientFiles {
        wasm,
        js,
        html,
        snippet,
        wasm_name,
        js_name,
        snippet_name,
    } = client_route::files().await?;
    let wasm_route = warp::path(wasm_name).map(move || {
        Response::builder()
            .header("Content-Type", "application/wasm")
            .header("Content-Length", wasm.len())
            .body(wasm.clone())
    });
    let js_route = warp::path(js_name).map(move || {
        Response::builder()
            .header("Content-Type", "text/javascript")
            .header("Content-Length", js.len())
            .body(js.clone())
    });
    let [snippet_0, snippet_1, snippet_2]: [String; 3] = snippet_name
        .split('/')
        .map(Into::into)
        .collect::<Vec<String>>()
        .try_into()
        .expect("snippet path has 3 parts");
    let snippet_route = warp::path(snippet_0)
        .and(warp::path(snippet_1))
        .and(warp::path(snippet_2))
        .map(move || {
            Response::builder()
                .header("Content-Type", "text/javascript")
                .header("Content-Length", snippet.len())
                .body(snippet.clone())
        });
    let html_route = warp::path("client").map(move || {
        Response::builder()
            .header("Content-Type", "text/html")
            .header("Content-Length", html.len())
            .body(html.clone())
    });

    // Endpoint for clients to submit work
    let submit_route = warp::path("submit").and(warp::body::json().map(handle_submit));

    let route = wasm_route
        .or(js_route)
        .or(snippet_route)
        .or(html_route)
        .or(submit_route);

    warp::serve(route).run(server_addr).await;
    Ok(())
}

fn handle_submit(submission: Submission) -> impl Reply {
    // TODO: real handling
    println!("{submission:?}");
    warp::reply()
}

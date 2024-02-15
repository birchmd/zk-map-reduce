use {
    client_route::ClientFiles,
    miden::Program,
    state::Msg,
    std::{
        net::{Ipv4Addr, SocketAddr, SocketAddrV4},
        sync::Arc,
    },
    tokio::sync::mpsc::UnboundedSender,
    warp::{http::Response, reply::Reply, Filter},
    zkmr_types::Submission,
};

mod client_route;
mod state;
mod verifier;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080));
    let (state, sender) = state::State::create();
    let program = state.program();
    let state_task = state.spawn_actor();

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
    let submit_route = warp::path("submit").and(
        warp::body::json()
            .and(warp::any().map(move || (program.clone(), sender.clone())))
            .map(handle_submit),
    );

    let route = wasm_route
        .or(js_route)
        .or(snippet_route)
        .or(html_route)
        .or(submit_route);

    warp::serve(route).run(server_addr).await;
    state_task.await?;
    Ok(())
}

fn handle_submit(submission: Submission, env: (Arc<Program>, UnboundedSender<Msg>)) -> impl Reply {
    let (program, sender) = env;
    let verify_result = verifier::validate_submission(&program, &submission);
    let msg = match verify_result {
        Ok(_) => Msg::CompletedJob {
            worker_id: submission.worker_id,
            job_id: submission.job_id,
            result: submission.result,
        },
        Err(e) => Msg::Log {
            message: format!(
                "Incorrect submission from {}: {:?}",
                submission.worker_id, e
            ),
        },
    };
    sender.send(msg).ok();
    warp::reply()
}

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
        wasm_name,
        js_name,
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

    let route = wasm_route.or(js_route).or(html_route).or(submit_route);

    warp::serve(route).run(server_addr).await;
    state_task.await?;
    Ok(())
}

fn handle_submit(submission: Submission, env: (Arc<Program>, UnboundedSender<Msg>)) -> impl Reply {
    let (program, sender) = env;
    let verify_result = verifier::validate_submission(&program, &submission);
    let result = submission.output_stack.first().copied().unwrap();
    let msg = match verify_result.as_ref() {
        Ok(_) => Msg::CompletedJob {
            worker_id: submission.worker_id,
            job_id: submission.job_id,
            result,
        },
        Err(e) => Msg::Log {
            message: format!(
                "Incorrect submission for job ID {} from {}: {:?}",
                submission.job_id, submission.worker_id, e
            ),
        },
    };
    sender.send(msg).ok();
    let response: String = match verify_result {
        Ok(_) => "Thank you for your honest work.".into(),
        Err(e) => format!("Your submission was incorrect. Error = {e:?}"),
    };
    let response = zkmr_types::Response::new(response);
    warp::reply::json(&response)
}

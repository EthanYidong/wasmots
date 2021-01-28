use warp::Filter;
use crossbeam_queue::ArrayQueue;
use tokio::sync::mpsc::Sender;

use std::sync::Arc;

async fn handle_get_wasm(wasm: Vec<u8>) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::http::Response::new(wasm))
}

async fn handle_get_task(task_queue: Arc<ArrayQueue<Vec<u8>>>) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(task) = task_queue.pop() {
        Ok(warp::http::Response::new(task))
    } else {
        Err(warp::reject())
    }
}

async fn handle_post_result(result: warp::hyper::body::Bytes, channel: Sender<Vec<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    channel.send(result.to_vec()).await.map_err(|_| warp::reject())?;
    Ok(warp::reply())
}

pub async fn run_server(wasm: Vec<u8>, tasks: Vec<Vec<u8>>, channel: Sender<Vec<u8>>) {
    let queue = ArrayQueue::new(tasks.len());
    for task in tasks {
        queue.push(task).unwrap();
    }
    let queue = Arc::new(queue);

    let get_wasm = warp::path!("wasmots" / "getwasm")
        .and(warp::get())
        .and(warp::any().map(move || wasm.clone()))
        .and_then(handle_get_wasm);
    
    let get_task = warp::path!("wasmots" / "gettask")
        .and(warp::get())
        .and(warp::any().map(move || queue.clone()))
        .and_then(handle_get_task);
    
    let post_result = warp::path!("wasmots" / "postresult")
        .and(warp::post())
        .and(warp::body::bytes())
        .and(warp::any().map(move || channel.clone()))
        .and_then(handle_post_result);

    warp::serve(get_wasm.or(get_task).or(post_result))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

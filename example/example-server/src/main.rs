use serde::{Serialize, Deserialize};

// Example of a very taxing computation you want to offload to other people's computers
#[derive(Serialize)]
struct AdditionRequest {
    lhs: i32,
    rhs: i32,
}

#[derive(Deserialize)]
struct AdditionResponse {
    result: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = Vec::new();
    // Let's generate a few requests
    for i in 0..100 {
        tasks.push(serde_cbor::to_vec(&AdditionRequest {
            lhs: 1,
            rhs: i,
        })?);
    }

    let wasm_bytes = std::fs::read("./target/wasm32-wasi/release/example_wasm.wasm")?;

    let (tx, mut rx) = tokio::sync::mpsc::channel::<Vec<u8>>(32);

    let receiver = tokio::spawn(async move {
        while let Some(response) = rx.recv().await {
            println!("Got result: {}", serde_cbor::from_slice::<AdditionResponse>(&response).unwrap().result);
        }
    });

    let _ = tokio::join!(wasmots::run_server(wasm_bytes, tasks, tx), receiver);

    Ok(())
}

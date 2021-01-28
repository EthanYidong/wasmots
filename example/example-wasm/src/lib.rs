use serde::{Serialize, Deserialize};

use std::io::prelude::*;

#[derive(Deserialize)]
struct AdditionRequest {
    lhs: i32,
    rhs: i32,
}

#[derive(Serialize)]
struct AdditionResponse {
    result: i32,
}

#[no_mangle]
pub fn process() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let request = serde_cbor::from_slice::<AdditionRequest>(&input).unwrap();

    let mut stdout = std::io::stdout();
    stdout.write_all(&serde_cbor::to_vec(&AdditionResponse {
        result: request.lhs + request.rhs,
    }).unwrap()).unwrap();

    // MAKE SURE YOU FLUSH!
    // Will not work otherwise.
    stdout.flush().unwrap();
}

use wasmer::*;
use wasmer_wasi as wasi;

use structopt::StructOpt;

use std::io::prelude::*;

#[derive(StructOpt)]
struct Opts {
    host: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();

    let mut wasm_bytes = Vec::new();
    ureq::get(&format!("{}/wasmots/getwasm", opts.host))
        .call()?
        .into_reader()
        .read_to_end(&mut wasm_bytes)?;

    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;

    let state = wasi::WasiState::new("hello_world")
        .stdin(Box::new(wasi::Pipe::new()))
        .stdout(Box::new(wasi::Pipe::new()))
        .build()?;
    let wasi_env = wasi::WasiEnv::new(state);
    let imports = wasi::generate_import_object_from_env(&store, wasi_env.clone(), wasi::WasiVersion::Latest);

    let instance = Instance::new(&module, &imports)?;
    let run_func: NativeFunc<(), ()> = instance.exports.get_native_function("process")?;

    loop {
        let mut task_bytes = Vec::new();
        let mut response_bytes = Vec::new();

        let task_response = ureq::get(&format!("{}/wasmots/gettask", opts.host))
            .call()?;
        
        if task_response.status() != 200 {
            break;
        }

        println!("Got a task");
        
        task_response
            .into_reader()
            .read_to_end(&mut task_bytes)?;
        
        wasi_env.state().fs.stdin_mut()?.as_mut().unwrap().downcast_mut::<wasi::Pipe>().unwrap().write_all(&task_bytes)?;
        run_func.call()?;
        wasi_env.state().fs.stdout_mut()?.as_mut().unwrap().downcast_mut::<wasi::Pipe>().unwrap().read_to_end(&mut response_bytes)?;

        ureq::post(&format!("{}/wasmots/postresult", opts.host))
            .send_bytes(&response_bytes)?;
    }

    Ok(())
}

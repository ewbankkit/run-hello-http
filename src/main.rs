use std::collections::hash_map::HashMap;
use wascc_host::{host, Actor, NativeCapability};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let actor = Actor::from_file("/home/kit/wrk/src/github.com/ewbankkit/wascc-actor-hello-http/target/wasm32-unknown-unknown/debug/wascc_actor_hello_http_signed.wasm")?;
    host::add_actor(actor)?;
    let capability = NativeCapability::from_file("/home/kit/wrk/src/github.com/wascc/wascc-host/examples/.assets/libwascc_httpsrv.so")?;
    host::add_native_capability(capability)?;

    host::configure(
        "MCE2V3C7MOUIA5I2W5TJ2QFZMUP7C4NYZ5LL5OZ6XYGA4WKUX5DNMD57",
        "wascc:http_server",
        generate_port_config(8081),
    )?;

    std::thread::park();

    Ok(())
}

fn generate_port_config(port: u16) -> HashMap<String, String> {
    let mut hm = HashMap::new();
    hm.insert("PORT".to_string(), port.to_string());

    hm
}

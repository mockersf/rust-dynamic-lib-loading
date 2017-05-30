#[no_mangle]
pub extern "Rust" fn compute(req: &str) -> String {
    println!("incoming data: {:?}", req);
    "done".to_string()
}

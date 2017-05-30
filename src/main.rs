extern crate libloading as lib;

fn main() {
    let path = "module1/target/debug/libmodule1.dylib";
    
    let lib = lib::Library::new(path).unwrap();
    let compute = unsafe {
        let func: lib::Symbol<extern "Rust" fn(&str) -> String> =
            lib.get(b"compute\0").unwrap();
        func.into_raw()
    };
    for i in 0..10000 {
        println!("{}", compute(&format!("{}", i)));
    }

}

extern crate libloading as lib;

pub struct LoadedModule {
    lib: lib::Library,
    compute: lib::os::unix::Symbol<extern "Rust" fn(&str) -> String>,
}

impl LoadedModule {
    pub fn load(path: &str) -> LoadedModule {
        let lib = lib::Library::new(path).unwrap();
        let compute = unsafe {
            let func: lib::Symbol<extern "Rust" fn(&str) -> String> =
                lib.get(b"compute\0").unwrap();
            func.into_raw()
        };
        println!("Loaded {:?}", path);
        LoadedModule {
            lib: lib,
            compute,
        }
    }

    fn compute(&self, req: &str) -> String {
        let compute = &self.compute;
        compute(req)
    }
}

fn main() {
    let module_path = "module1/target/debug/libmodule1.dylib";
    let module1 = LoadedModule::load(module_path);
    
    for i in 0..10000 {
        println!("{}", module1.compute(&format!("{}", i)));
    }

}

/* -- Lots of reuse from: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs */

extern crate bindgen;

use std::env;
use std::fs;
// use std::fs::{self, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
// use bindgen::*; //dirty

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(n) => n,
        Err(e) => fail(&format!("\n{} failed with {}\n", stringify!($e), e)),
    })
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

//TODO: Auto binding generation
// bindgen -l lib/libafcuda.dylib -I . -builtins -o arrayfire.rs arrayfire.h
// fn build_bindings(package_name: &str
//   , out_dir: &std::path::PathBuf
//   , arrayfire_dir: &std::path::PathBuf) 
// {
//   let rust_header = package_name.to_string() + ".rs";
//   let c_header = package_name.to_string() + ".h";

//   let include_path = arrayfire_dir.join("include");
//   // let include_dir = include_path.to_str().unwrap();
//   let af_dir = include_path.join("af");
  
//   // let clang_args = include_files;//"-I . ";//-I ".to_string() + include_path.to_str().unwrap();
//   // println!("clang args --> {:?}", clang_args);

//   let rs_dir = std::path::Path::new(&out_dir).join(rust_header);
//   let rs_path = rs_dir.to_str().unwrap();

//   let mut bindings = bindgen::builder();
//   // bindings.forbid_unknown_types();
//   bindings.emit_builtins();
  
//   // let include_files = fs::read_dir(&Path::new(af_dir.to_str().unwrap())).unwrap();
//   // for p in include_files{
//   //   // println!("include_files {:?}", p.unwrap().path().display());
//   //   let filewrap = p.unwrap();
//   //   let filepath = filewrap.path();
//   //   bindings.header(filepath.display().to_string());
//   // }

//   let h_path = std::path::Path::new(&include_path).join(c_header);
//   let h_path = String::from(h_path.to_str().unwrap());
//   bindings.header(h_path);

//   let bindings = bindings.generate();
//   let bindings = bindings.unwrap();
//   bindings.write_to_file(rs_path).unwrap();
// }

fn main() {
    // Setup pathing
    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let arrayfire_dir = src.join("arrayfire");
    let build_dir = arrayfire_dir.join("build");
    let src_dir = src.join("src");
    let _ = fs::create_dir(&build_dir);
    
    
    println!("CARGO_MANIFEST_DIR : {:?}", src);

    // Run our cmake operations
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);
    run(cmake_cmd.arg("..")
      .arg("-DCMAKE_BUILD_TYPE=Release")
      .arg("-DBUILD_CUDA=ON")
      .arg("-DBUILD_OPENCL=OFF")
      .arg("-DBUILD_CPU=OFF"), "cmake");

    // run make
    let mut make_cmd = Command::new("make");
    make_cmd.current_dir(&build_dir);
    run(make_cmd.arg("-j8"), "make");

    println!("cargo:rustc-link-search=native={}", build_dir.join("src/backend/cuda").display());
    println!("cargo:rustc-link-lib=dylib=afcuda");
    println!("cargo:include={:?}", "/Users/jramapuram/projects/rust_arrayfire/arrayfire/include");

    // build_bindings("arrayfire", &src_dir, &arrayfire_dir);//, "afcuda")
}
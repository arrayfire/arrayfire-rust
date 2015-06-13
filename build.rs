/* -- Lots of reuse from: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs */

extern crate bindgen;
extern crate rustc_serialize;

use std::env;
use std::fs;
use rustc_serialize::json;
use std::fs::{OpenOptions, File};
use std::io::{ErrorKind, Seek, SeekFrom, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use bindgen::*; //dirty

#[derive(RustcDecodable)]
struct Config {
  // TODO: Use these variables to pull in from path  
  use_lib: bool,
  lib_dir: String,
  inc_dir: String,
  
  // Build related
  release_type: String,
  make_flags: String,
  build_cuda: String,
  build_opencl: String,
  build_cpu: String,
  build_examples: String,
  build_test: String,
}

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

// Original CLI command: bindgen -l lib/libafcuda.dylib -I . -builtins -o arrayfire.rs arrayfire.h
fn build_bindings(package_name: &str
  , out_dir: &std::path::PathBuf
  , arrayfire_dir: &std::path::PathBuf) 
{
  let rust_header = package_name.to_string() + ".rs";
  let c_header = package_name.to_string() + ".h";

  let include_path = arrayfire_dir.join("include");
  let af_dir = include_path.join("af");
  
  let rs_dir = std::path::Path::new(&out_dir).join(rust_header);
  let rs_path = rs_dir.to_str().unwrap();

  let mut bindings = bindgen::builder();
  bindings.emit_builtins();
  
  // Blob in '-I arrayfire/include' to the "VPATH" 
  bindings.header("-I");
  bindings.header(include_path.to_str().unwrap());

  let h_path = std::path::Path::new(&include_path).join(c_header);
  let h_path = String::from(h_path.to_str().unwrap());
  bindings.header(h_path);

  let bindings = bindings.generate();
  let bindings = bindings.unwrap();
  bindings.write_to_file(rs_path).unwrap();
}

fn read_file(file_name: &std::path::PathBuf) -> String {
  let file_path = file_name.to_str().unwrap();
  let options = OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&file_path);
  let mut file = match options {
    Ok(file) => file,
    Err(..) => panic!("error reading file"),
  };

  let mut s = String::new();
  file.read_to_string(&mut s);
  return s.to_string()
}

fn read_conf(conf_file: &std::path::PathBuf) -> Config {
  let raw_conf = read_file(conf_file);
  let decoded: Config = json::decode(&raw_conf).unwrap();
  decoded
}

fn main() {
    // Setup pathing
    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let conf_file = src.join("build.conf");
    let conf = read_conf(&conf_file);

    // create build directories
    let arrayfire_dir = src.join("arrayfire");
    let build_dir = arrayfire_dir.join("build");
    let src_dir = src.join("src");
    let _ = fs::create_dir(&build_dir);

    // Run our cmake operations
    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);
    run(cmake_cmd.arg("..")
      .arg(format!("-DCMAKE_BUILD_TYPE={}", conf.release_type))
      .arg(format!("-DBUILD_CUDA={}", conf.build_cuda))
      .arg(format!("-DBUILD_OPENCL={}", conf.build_opencl))
      .arg(format!("-DBUILD_EXAMPLES={}", conf.build_examples))
      .arg(format!("-DBUILD_TEST={}", conf.build_test))
      .arg(format!("-DBUILD_CPU={}", conf.build_cpu)), "cmake");

    // run make
    let mut make_cmd = Command::new("make");
    make_cmd.current_dir(&build_dir);
    run(make_cmd.arg(conf.make_flags), "make");

    // build correct backend
    let mut backend_dir = String::new();
    let mut backend = String::new();

    if conf.build_cuda == "ON"{
      backend = backend + "afcuda";
      backend_dir = backend_dir + build_dir.join("src/backend/cuda").to_str().unwrap();
    }else if conf.build_opencl == "ON"{
      backend = backend + "afcl";
      let cl_dir = format!("{}", build_dir.join("src/backend/opencl").to_str().unwrap());
      backend_dir = backend_dir + &cl_dir;
    }else if conf.build_cpu == "ON"{
      backend = backend + "afcpu";
      let cpu_dir = format!("{}", build_dir.join("src/backend/cpu").to_str().unwrap());
      backend_dir = backend_dir + &cpu_dir;  
    }

    println!("cargo:rustc-link-search=native={}", backend_dir);
    println!("cargo:rustc-link-lib=dylib={}", backend);

    build_bindings("arrayfire", &src_dir, &arrayfire_dir);
}

/* -- Lots of reuse from: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs */

extern crate bindgen;

use std::env;
use std::fs;
use std::fs::{OpenOptions, File};
use std::io::{ErrorKind, Seek, SeekFrom, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use bindgen::*; //dirty

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

// https://stackoverflow.com/questions/30412521/how-to-read-specific-number-of-bytes-from-a-stream/30413877#30413877
fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
    where R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    let status = chunk.read_to_end(&mut buf);
    // Do appropriate error handling
    match status {
        Ok(n) => assert_eq!(bytes_to_read as usize, n),
        _ => panic!("Didn't read enough"),
    }
    buf
}

// Dirty, prepends a header dependency on libc which is not auto-added
fn prepend_deps(header: &str, deps: &str){
  let options = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(false)
                    .open(header);
  let mut file = match options {
    Ok(file) => file,
    Err(..) => panic!("error reading file"),
  };

  // read N bytes first
  let num_bytes = deps.len();
  let buf = read_n(&mut file, num_bytes as u64);

  file.seek(SeekFrom::Start(0));
  file.write(deps.as_bytes());
  file.write(&buf);
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

  prepend_deps(rs_path, "extern crate libc;\n");
}

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

    build_bindings("arrayfire", &src_dir, &arrayfire_dir);
}
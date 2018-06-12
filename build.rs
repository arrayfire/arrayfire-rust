/* -- Lots of reuse from: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs */
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rustc_version;

use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read};
use std::path::PathBuf;
use std::process::Command;
use std::convert::AsRef;
use rustc_version::{version, Version};

// Windows specific library file names
static WIN_CUDA_LIB: &'static str = "afcuda";
static WIN_OCL_LIB: &'static str = "afopencl";
static WIN_UNI_LIB: &'static str = "af";
// Linux & OSX specific library file names
static UNIX_CUDA_LIB: &'static str = "libafcuda";
static UNIX_OCL_LIB: &'static str = "libafopencl";
static UNIX_UNI_LIB: &'static str = "libaf";

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Config {
    // Use existing arrayfire installation
    use_lib: bool,

    // Variables used for building arrayfire submodule
    build_type: String,
    build_threads: u8,
    build_cpu: String,
    build_cuda: String,
    build_opencl: String,
    build_nonfree: String,
    build_examples: String,
    build_test: String,

    with_intelmkl: String,
    with_imageio: String,
    with_graphics: String,
    with_opencl_blas_lib: String,

    vcpkg_toolchain_file: String,
    cuda_sdk: String,
    opencl_sdk: String,
    win_cmake_generator: String,
    win_vs_toolset: String,
    cuda_host_compiler: String,
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn dir_exists(location: &str) -> bool {
    match fs::metadata(location) {
        Ok(f)  => f.is_dir(),
        Err(_) => false,
    }
}

fn file_exists(location: &str) -> bool {
    match fs::metadata(location) {
        Ok(f)  => f.is_file(),
        Err(_) => false,
    }
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
            Ok(status) => status,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                    fail(&format!("failed to run cmd: {}\nis `{}` not installed?", e, program));
                }
                Err(e) => fail(&format!("failed to execute command: {}", e)),
        };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn read_file(file_name: &std::path::PathBuf) -> String {
    let file_path = file_name.to_str().unwrap();
    let options = OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_path);

    let mut file = match options {
            Ok(file)=> file,
            Err(..) => panic!("error reading file"),
        };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_)   => s,
        Err(..) => panic!("Error reading file to a string"),
    }
}

fn read_conf(conf_file: &std::path::PathBuf) -> Config {
    let raw_conf = read_file(conf_file);
    let decoded: Config = serde_json::from_str(&raw_conf).unwrap();
    decoded
}

fn prep_cmake_options(conf: &Config) -> Vec<String> {
    let mut options: Vec<String> = vec![];

    match conf.build_type.as_ref() {
        "Release" | "RelWithDebInfo" | "Debug" => {
            options.push(format!("-DCMAKE_BUILD_TYPE:STRING={}", conf.build_type));
        },
        _ => fail("Invalid value for build_type option"),
    };
    match conf.build_cpu.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_BUILD_CPU:BOOL={}",
                                 conf.build_cpu));
        },
        _ => fail("Invalid value for build_cpu option"),
    };
    match conf.build_cuda.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_BUILD_CUDA:BOOL={}",
                                 conf.build_cuda));
        },
        _ => fail("Invalid value for build_cuda option"),
    };
    match conf.build_opencl.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_BUILD_OPENCL:BOOL={}",
                                 conf.build_opencl));
        },
        _ => fail("Invalid value for build_opencl option"),
    };
    match conf.build_nonfree.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_BUILD_NONFREE:BOOL={}",
                                 conf.build_nonfree));
        },
        _ => fail("Invalid value for build_nonfree option"),
    };
    match conf.build_examples.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_BUILD_EXAMPLES:BOOL={}",
                                 conf.build_examples));
        },
        _ => fail("Invalid value for build_examples option"),
    };
    match conf.build_test.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DBUILD_TESTING:BOOL={}",
                                 conf.build_test));
        },
        _ => fail("Invalid value for build_test option"),
    };
    match conf.with_intelmkl.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DUSE_CPU_MKL:BOOL={0}",
                                 conf.with_intelmkl));
            options.push(format!("-DUSE_OPENCL_MKL:BOOL={0}",
                                 conf.with_intelmkl));
        },
        _ => fail("Invalid value for with_intelmkl option"),
    };
    match conf.with_imageio.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_WITH_IMAGEIO:BOOL={0}",
                                 conf.with_imageio));
        },
        _ => fail("Invalid value for with_imageio option"),
    };
    match conf.with_graphics.as_ref() {
        "ON" | "OFF" => {
            options.push(format!("-DAF_WITH_GRAPHICS:BOOL={0}",
                                 conf.with_graphics));
        },
        _ => fail("Invalid value for with_graphics option"),
    };
    match conf.with_opencl_blas_lib.as_ref() {
        "clblast" => {
            options.push(format!("-DAF_OPENCL_BLAS_LIBRARY:STRING=CLBlast"));
        },
        "clblas" => {
            options.push(format!("-DAF_OPENCL_BLAS_LIBRARY:STRING=clBLAS"));
        },
        _ => fail("Invalid value for with_opencl_blas_lib option"),
    };
    return options;
}

#[cfg(windows)]
fn run_cmake_command(conf: &Config, build_dir: &std::path::PathBuf) {
    let _ = fs::create_dir(&build_dir);

    let options = prep_cmake_options(conf);

    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);

    run(cmake_cmd.arg("..")
        .arg("-T").arg(format!("{}", conf.win_vs_toolset))
        .arg("-G").arg(format!("{}", conf.win_cmake_generator))
        .args(&options)
        .arg(format!("-DCMAKE_TOOLCHAIN_FILE:FILEPATH={}",
                        conf.vcpkg_toolchain_file))
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", "package"))
        , "cmake");

    let mut make_cmd= Command::new("MSBuild.exe");
    make_cmd.current_dir(&build_dir);
    run(make_cmd
        .arg(format!("/m:{}", conf.build_threads))
        .arg(format!("/p:Configuration={}", conf.build_type))
        .arg(format!("ArrayFire.sln")),
        "MSBuild");

    let mut install_cmd= Command::new("MSBuild.exe");
    install_cmd.current_dir(&build_dir);
    run(install_cmd
        .arg(format!("/p:Configuration={}", conf.build_type))
        .arg(format!("INSTALL.vcxproj")),
        "Install");
}

#[cfg(not(windows))]
fn run_cmake_command(conf: &Config, build_dir: &std::path::PathBuf) {
    let _ = fs::create_dir(&build_dir);

    let options = prep_cmake_options(conf);
    println!("options are {:?}", options);

    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);

    run(cmake_cmd.arg("..")
        .args(&options)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", "package"))
        .arg(format!("-DCUDA_HOST_COMPILER={}", conf.cuda_host_compiler))
        , "cmake");

    let mut make_cmd= Command::new("make");
    make_cmd.current_dir(&build_dir);
    run(make_cmd
        .arg(format!("-j{}", conf.build_threads))
        .arg(format!("install")), "make");
}

fn backend_exists(name: &str) -> bool{
    let win_backend   = name.to_string() + ".dll";
    let osx_backend   = name.to_string() + ".dylib";
    let linux_backend = name.to_string() + ".so";

    return file_exists(&win_backend)
        || file_exists(&osx_backend)
        || file_exists(&linux_backend)
}

fn blob_backends(conf: &Config,
                 build_dir: &std::path::PathBuf) -> (Vec<String>, Vec<String>) {
    let mut backend_dirs :Vec<String>= Vec::new();
    let mut backends :Vec<String> = Vec::new();

    if conf.use_lib {
        let afpath  = match env::var("AF_PATH") {
            Ok(af_path) => PathBuf::from(&af_path),
            Err(_)      => {
                println!("WARNING! USE_LIB is defined,
                          but AF_PATH is not found,");
                println!("Trying to find libraries from
                          known default locations");
                if cfg!(target_os = "windows") {
                    PathBuf::from("C:/Program Files/ArrayFire/v3/")
                } else {
                    PathBuf::from("/opt/arrayfire/")
                }
            },
        };

        let libpath = afpath.join("lib");
        backend_dirs.push(libpath.to_str().to_owned().unwrap().to_string());

        if !cfg!(target_os = "windows") {
            backend_dirs.push(String::from("/usr/local/lib"));
            backend_dirs.push(String::from("/usr/lib"));
        }
    } else {
        backend_dirs.push(build_dir.join("package/lib")
                          .to_str().to_owned().unwrap().to_string());
    }

    let mut uni_lib_exists = false;
    let mut cud_lib_exists = false;
    let mut ocl_lib_exists = false;

    for backend_dir in backend_dirs.iter() {
        let lib_dir = PathBuf::from(backend_dir);

        let culib_name = if cfg!(windows) {WIN_CUDA_LIB} else {UNIX_CUDA_LIB};
        cud_lib_exists = cud_lib_exists || backend_exists(&lib_dir
                                                          .join(culib_name)
                                                          .to_string_lossy());
        let ocllib_name = if cfg!(windows) {WIN_OCL_LIB} else {UNIX_OCL_LIB};
        ocl_lib_exists = ocl_lib_exists || backend_exists(&lib_dir
                                                          .join(ocllib_name)
                                                          .to_string_lossy());
        let unilib_name = if cfg!(windows) {WIN_UNI_LIB} else {UNIX_UNI_LIB};
        uni_lib_exists = uni_lib_exists || backend_exists(&lib_dir
                                                          .join(unilib_name)
                                                          .to_string_lossy());
    }

    if ! conf.use_lib {
        // blob in cuda deps
        if cud_lib_exists {
            if cfg!(windows) {
                backend_dirs.push(format!("{}\\lib\\x64", conf.cuda_sdk));
            } else {
                let sdk_dir = format!("{}/{}", conf.cuda_sdk, "lib64");
                match dir_exists(&sdk_dir){
                    true  => {
                        backend_dirs.push(sdk_dir);
                    },
                    false => {
                        backend_dirs.push(format!("{}/{}",
                                                  conf.cuda_sdk, "lib"));
                    },
                };
            }
        }

        //blob in opencl deps
        if ocl_lib_exists {
            if ! cfg!(target_os = "macos") {
                backends.push("OpenCL".to_string());
            }
            if cfg!(windows) {
                let sdk_dir = format!("{}\\lib\\x64", conf.opencl_sdk);
                if dir_exists(&sdk_dir) {
                    backend_dirs.push(sdk_dir);
                } else {
                    backend_dirs.push(format!("{}\\lib\\x86_64",
                                              conf.opencl_sdk));
                }
            } else {
                let sdk_dir = format!("{}/{}", conf.opencl_sdk, "lib64");
                if dir_exists(&sdk_dir) {
                    backend_dirs.push(sdk_dir);
                } else {
                    backend_dirs.push(format!("{}/{}", conf.opencl_sdk, "lib"));
                }
            }
        }

        if conf.with_graphics=="ON" {
            if !conf.use_lib {
                backend_dirs.push(build_dir
                                  .join(format!("third_party/forge/lib"))
                                  .to_str().to_owned().unwrap().to_string());
            }
        }
    }

    if uni_lib_exists {
        backends.push("af".to_string());
        if !conf.use_lib && conf.with_graphics=="ON" {
            backends.push("forge".to_string());
        }
    }

    return (backends, backend_dirs);
}

fn main() {
    // Setup pathing
    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let conf_file = src.join("build.conf");
    let conf = read_conf(&conf_file);

    let arrayfire_dir = src.join("arrayfire");
    let build_dir = arrayfire_dir.join("build");

    if !conf.use_lib {
        run_cmake_command(&conf, &build_dir);
    }

    let (backends, backend_dirs) = blob_backends(&conf, &build_dir);
    for backend in backends.iter() {
        println!("cargo:rustc-link-lib=dylib={}", backend);
    }
    for backend_dir in backend_dirs.iter() {
        println!("cargo:rustc-link-search=native={}", backend_dir);
    }
    if version().unwrap() >= Version::parse("1.8.0").unwrap() {
        println!("cargo:rustc-cfg=op_assign");
    }
}

/* -- Lots of reuse from: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs */
extern crate rustc_serialize;

use std::env;
use std::fs;
use rustc_serialize::json;
use std::fs::OpenOptions;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::convert::AsRef;

#[allow(dead_code)]
#[derive(RustcDecodable)]
struct Config {
    // Use the existing lib if it exists
    use_lib: bool,
    lib_dir: String,
    inc_dir: String,

    // Build related
    build_type: String,
    build_threads: String,
    build_examples: String,
    build_test: String,
    build_graphics: String,

    // backend upstream library options
    glew_static: String,
    freeimage_type: String,
    cpu_fft_type: String,
    cpu_blas_type: String,
    cpu_lapack_type: String,

    // backend upstream library install paths
    freeimage_dir: String,
    fftw_dir: String,
    acml_dir: String,
    mkl_dir: String,
    lapacke_dir: String,
    glew_dir: String,
    glfw_dir: String,
    boost_dir: String,

    // GPU backends
    cuda_sdk: String,
    opencl_sdk: String,
    sdk_lib_dir: String,
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
    let decoded: Config = json::decode(&raw_conf).unwrap();
    decoded
}

#[cfg(windows)]
fn run_cmake_command(conf: &Config, build_dir: &std::path::PathBuf) {
    // create build directories
    let _ = fs::create_dir(&build_dir);
    // Run our cmake operations
    let mut fft_options     = vec![];
    let mut blas_options    = vec![];
    let mut lapack_options  = vec![];
    let mut glew_lib        = vec![];
    let mut graphics_options= vec![];
    let mut freeimage_options=vec![];
    match conf.cpu_fft_type.as_ref() {
        "FFTW" => {
            fft_options.push(format!("-DFFTW_ROOT:STRING={0}", conf.fftw_dir));
            fft_options.push(format!("-DFFTW_LIB:STRING={0}\\libfftw3-3.lib", conf.fftw_dir));
            fft_options.push(format!("-DFFTWF_LIB:STRING={0}\\libfftw3f-3.lib", conf.fftw_dir));
            fft_options.push(format!("-DFFTWL_LIB:STRING={0}\\libfftw3l-3.lib", conf.fftw_dir));
        },
        "ACML" => {
            fft_options.push(format!("-DFFTW_ROOT:STRING={0}", conf.acml_dir));
            fft_options.push(format!("-DFFTW_LIBRARIES:STRING={0}\\lib\\acml_fftw.lib",
                                    conf.acml_dir));
        },
        "MKL" => {
            fft_options.push(format!("-DFFTW_ROOT:STRING={0}", conf.mkl_dir));
            fft_options.push(format!("-DFFTW_LIBRARIES:STRING={0}\\lib\\mkl_rt.lib",
                                    conf.mkl_dir));
        },
        _ => fail("Invalid FFT upstream option set"),
    };
    match conf.cpu_blas_type.as_ref() {
        "LAPACKE" => {
            blas_options.push(format!("-DUSE_CPU_F77_BLAS:BOOL={}", "ON"));
            blas_options.push(format!("-DCBLAS_INCLUDE_DIR:STRING={0}\\include",
                                    conf.lapacke_dir));
            blas_options.push(format!("-DCBLAS_cblas_LIBRARY:STRING={0}\\lib\\libblas.lib",
                                    conf.lapacke_dir));
        },
        "MKL" => {
            blas_options.push(format!("-DUSE_CPU_MKL:BOOL={}", "ON"));
            blas_options.push(format!("-DCBLAS_INCLUDE_DIR:STRING={0}\\include", conf.mkl_dir));
            blas_options.push(format!("-DCBLAS_cblas_LIBRARY:STRING={0}\\lib\\mkl_rt.lib",
                                    conf.mkl_dir));
        },
        _ => fail("Invalid BLAS upstream option set"),
    };
    match conf.cpu_lapack_type.as_ref() {
        "LAPACKE" => {
            lapack_options.push(format!("-DLAPACKE_ROOT:STRING={0}",
                                        conf.lapacke_dir));
            lapack_options.push(format!("-DLAPACK_INCLUDE_DIR:STRING={0}\\include",
                                        conf.lapacke_dir));
            lapack_options.push(format!("-DLAPACKE_LIB:STRING={0}\\lib\\liblapacke.lib",
                                        conf.lapacke_dir));
            lapack_options.push(format!("-DLAPACK_LIB:STRING={0}\\lib\\liblapack.lib",
                                        conf.lapacke_dir));
        },
        "MKL" => {
            lapack_options.push(format!("-DUSE_CPU_MKL:BOOL={0}", "ON"));
            lapack_options.push(format!("-DUSE_OPENCL_MKL:BOOL={0}", "ON"));
            lapack_options.push(format!("-DLAPACKE_INCLUDES:STRING={0}\\include", conf.mkl_dir));
            lapack_options.push(format!("-DLAPACKE_LIB:STRING={0}\\lib\\mkl_rt.lib", conf.mkl_dir));
            lapack_options.push(format!("-DLAPACK_LIB:STRING={0}\\lib\\mkl_rt.lib", conf.mkl_dir));
        },
        _ => fail("Invalid LAPACK upstream option set"),
    };
    match conf.glew_static.as_ref() {
        "OFF" => {
            glew_lib.push(format!("-DGLEW_LIBRARY:STRING={0}\\lib\\Release\\x64\\glew32.lib",
                                    conf.glew_dir));
            glew_lib.push(format!("-DGLEWmxd_LIBRARY:STRING={0}\\lib\\Release MX\\x64\\glew32mx.lib",
                                    conf.glew_dir));
        },
        "ON" => {
            glew_lib.push(format!("-DGLEW_LIBRARY:STRING={0}\\lib\\Release\\x64\\glew32s.lib",
                                conf.glew_dir));
            glew_lib.push(format!("-DGLEWmxs_LIBRARY:STRING={0}\\lib\\Release MX\\x64\\glew32mxs.lib",
                                conf.glew_dir));
        },
        _ => fail("Invalid GLEW STATIC library option option set"),
    };
    match conf.build_graphics.as_ref() {
        "OFF" => {
            graphics_options.push(format!("-DBUILD_GRAPHICS:BOOL={0}", "OFF"));
        },
        "ON" => {
            graphics_options.push(format!("-DBUILD_GRAPHICS:BOOL={0}", "ON"));
            graphics_options.push(format!("-DGLEW_ROOT_DIR={0}", conf.glew_dir));
            graphics_options.push(format!("-DUSE_GLEWmx_STATIC:BOOL={0}", conf.glew_static));
            graphics_options.push(format!("-DGLEW_INCLUDE_DIR:STRING={0}\\include", conf.glew_dir));
            graphics_options.push(format!("-DGLFW_INCLUDE_DIR:STRING={0}\\include", conf.glfw_dir));
            graphics_options.push(format!("-DGLFW_LIBRARY:STRING={0}\\lib-msvc120\\glfw3.dll",
                                        conf.glfw_dir));
            for glew_curr_lib in glew_lib {
                graphics_options.push(glew_curr_lib);
            }
        },
        _ => fail("Invalid graphics build option set"),
    };
    match conf.freeimage_type.as_ref() {
        "OFF"    => {
            freeimage_options.push(format!("-DFREEIMAGE_FOUND:STRING={}", conf.freeimage_type));
        },
        "STATIC" => {
            freeimage_options.push(format!("-DFREEIMAGE_FOUND:STRING={}", "ON"));
            freeimage_options.push(format!("-DUSE_FREEIMAGE_STATIC:BOOL={}", "ON"));
            freeimage_options.push(format!("-DFREEIMAGE_INCLUDE_PATH:STRING={0}",
                                        conf.freeimage_dir));
            freeimage_options.push(format!("-DFREEIMAGE_STATIC_LIBRARY:STRING={0}\\FreeImageLib.lib",
                                        conf.freeimage_dir));
        },
        "DYNAMIC" => {
            freeimage_options.push(format!("-DFREEIMAGE_FOUND:STRING={}", "ON"));
            freeimage_options.push(format!("-DUSE_FREEIMAGE_STATIC:BOOL={}", "OFF"));
            freeimage_options.push(format!("-DFREEIMAGE_INCLUDE_PATH:STRING={0}",
                                        conf.freeimage_dir));
            freeimage_options.push(format!("-DFREEIMAGE_DYNAMIC_LIBRARY:STRING={0}\\FreeImage.lib",
                                        conf.freeimage_dir));
        },
        _ => fail("Invalid freeimage build option set"),
    };

    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);

    run(cmake_cmd.arg("..").arg("-G").arg("Visual Studio 12 2013 Win64")
        .args(&[format!("-DCMAKE_BUILD_TYPE:STRING={}", conf.build_type),
                format!("-DBUILD_EXAMPLES:BOOL={}", conf.build_examples),
                format!("-DBUILD_TEST:BOOL={}", conf.build_test),
                format!("-DBOOST_ROOT={}", conf.boost_dir),
                format!("-DCMAKE_INSTALL_PREFIX={}", "package")])
        .args(&freeimage_options)
        .args(&fft_options)
        .args(&blas_options)
        .args(&lapack_options)
        .args(&graphics_options)
        , "cmake");

    let mut make_cmd= Command::new("C:\\Program Files (x86)\\MSBuild\\12.0\\Bin\\MSBuild.exe");
    make_cmd.current_dir(&build_dir);
    run(make_cmd
        .arg(format!("/m:{}", conf.build_threads))
        .arg(format!("/p:Configuration={}", conf.build_type))
        .arg(format!("ArrayFire.sln")),
        "MSBuild");
    let mut install_cmd= Command::new("C:\\Program Files (x86)\\MSBuild\\12.0\\Bin\\MSBuild.exe");
    install_cmd.current_dir(&build_dir);
    run(install_cmd
        .arg(format!("/p:Configuration={}", conf.build_type))
        .arg(format!("INSTALL.vcxproj")),
        "Install");
}

#[cfg(not(windows))]
fn run_cmake_command(conf: &Config, build_dir: &std::path::PathBuf) {
    // create build directories
    let _ = fs::create_dir(&build_dir);
    // Run our cmake operations
    let mut blas_options    = vec![];
    let mut lapack_options  = vec![];
    let mut graphics_options= vec![];
    let mut freeimage_options=vec![];
    match conf.cpu_fft_type.as_ref() {
        "FFTW" => println!("Using FFTW upstream for fft functions on cpu backend"),
        "ACML" => println!("Using ACML upstream for fft functions on cpu backend"),
        "MKL" => println!("Using MKL upstream for fft functions on cpu backend"),
        _ => fail("Invalid FFT upstream option set"),
    };
    match conf.cpu_blas_type.as_ref() {
        "LAPACKE" => { blas_options.push(format!("-DUSE_CPU_F77_BLAS:BOOL={}", "OFF")); },
        "MKL" => { blas_options.push(format!("-DUSE_CPU_MKL:BOOL={}", "ON")); },
        _ => fail("Invalid BLAS upstream option set"),
    };
    match conf.cpu_lapack_type.as_ref() {
        "LAPACKE" => {},
        "MKL" => {
            lapack_options.push(format!("-DUSE_CPU_MKL:BOOL={0}", "ON"));
            lapack_options.push(format!("-DUSE_OPENCL_MKL:BOOL={0}", "ON"));
        },
        _ => fail("Invalid LAPACK upstream option set"),
    };
    match conf.build_graphics.as_ref() {
        "OFF" => {
            graphics_options.push(format!("-DBUILD_GRAPHICS:BOOL={0}", "OFF"));
        },
        "ON" => {
            graphics_options.push(format!("-DBUILD_GRAPHICS:BOOL={0}", "ON"));
            graphics_options.push(format!("-DUSE_GLEWmx_STATIC:BOOL={0}", conf.glew_static));
        },
        _ => fail("Invalid graphics build option set"),
    };
    match conf.freeimage_type.as_ref() {
        "OFF"    => { println!("Using Freeimage upstream for image io functions"); },
        "STATIC" => { freeimage_options.push(format!("-DUSE_FREEIMAGE_STATIC:BOOL={}", "ON")); },
        "DYNAMIC" => { freeimage_options.push(format!("-DUSE_FREEIMAGE_STATIC:BOOL={}", "OFF")); },
        _ => fail("Invalid freeimage build option set"),
    };

    let mut cmake_cmd = Command::new("cmake");
    cmake_cmd.current_dir(&build_dir);

    run(cmake_cmd.arg("..")
        .args(&[format!("-DCMAKE_BUILD_TYPE:STRING={}", conf.build_type),
                format!("-DBUILD_EXAMPLES:BOOL={}", conf.build_examples),
                format!("-DBUILD_TEST:BOOL={}", conf.build_test),
                format!("-DCMAKE_INSTALL_PREFIX:STRING={}", "package")])
        .args(&freeimage_options)
        .args(&blas_options)
        .args(&lapack_options)
        .args(&graphics_options)
        , "cmake");

    // run make
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

fn blob_backends(conf: &Config, build_dir: &std::path::PathBuf) -> (Vec<String>, Vec<String>) {
    let mut backend_dirs :Vec<String>= Vec::new();
    let mut backends :Vec<String> = Vec::new();

    if conf.use_lib {
        backend_dirs.push(conf.lib_dir.to_owned());
    } else {
        backend_dirs.push(build_dir.join("package/lib").to_str().to_owned().unwrap().to_string());
    }

    let lib_dir = PathBuf::from(backend_dirs.last().unwrap());

    // blob in cuda deps
    if backend_exists(&lib_dir.join("libafcuda").to_string_lossy()) {
        if cfg!(windows) {
            backend_dirs.push(format!("{}\\lib\\x64", conf.cuda_sdk));
            backend_dirs.push(format!("{}\\nvvm\\lib\\x64", conf.cuda_sdk));
        } else {
            backend_dirs.push(format!("{}/{}", conf.cuda_sdk, conf.sdk_lib_dir));
            backend_dirs.push(format!("{}/nvvm/{}", conf.cuda_sdk, conf.sdk_lib_dir));
        }
    }

    //blob in opencl deps
    if backend_exists(&lib_dir.join("libafopencl").to_string_lossy()) {
        if ! cfg!(target_os = "macos"){
          backends.push("OpenCL".to_string());
        }
        if cfg!(windows) {
            backend_dirs.push(format!("{}\\lib\\x64", conf.opencl_sdk));
        } else {
            backend_dirs.push(format!("{}/{}", conf.opencl_sdk, conf.sdk_lib_dir));
        }
    }

    if backend_exists(&lib_dir.join("libaf").to_string_lossy()) {
        backends.push("af".to_string());
    }

    if conf.build_graphics=="ON" {
        backends.push("forge".to_string());
        if !conf.use_lib {
            backend_dirs.push(build_dir.join("third_party/forge/lib")
                            .to_str().to_owned().unwrap().to_string());
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

    // build correct backend
    let (backends, backend_dirs) = blob_backends(&conf, &build_dir);
    for backend in backends.iter() {
        println!("cargo:rustc-link-lib=dylib={}", backend);
    }
    for backend_dir in backend_dirs.iter() {
        println!("cargo:rustc-link-search=native={}", backend_dir);
    }
}

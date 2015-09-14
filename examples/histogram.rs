extern crate arrayfire as af;

use af::*;
use std::env;
use std::path::PathBuf;

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    set_device(0);
    info();

    let assets_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("arrayfire").join("assets").join("examples").join("images");

    let img_wnd = match Window::new(480, 640, String::from("Input Image")) {
        Ok(v) => { v.set_position(100, 100).unwrap(); v },
        Err(e)=> panic!("Window creation failed, exiting: {}", e),
    };

    let hst_wnd = match Window::new(512, 512, String::from("Input Image Histogram")) {
        Ok(v) => { v.set_position(600, 100).unwrap(); v },
        Err(e)=> panic!("Window creation failed, exiting: {}", e),
    };

    let (man, hst) = match load_image(format!("{}/man.jpg", assets_dir.display()), false) {
        Ok(v) => match histogram(&v, 256, 0.0, 255.0) {
            Ok(h) => (v, h),
            Err(e)=> panic!("Histogram computation failed, exiting: {}", e),
        },
        Err(e)=> panic!("Image loading failed, exiting: {}", e),
    };

    let disp_img = man.dims()
        .and_then(|x| constant(255 as f32, x))
        .and_then(|x| div(&man, &x, false))
        .unwrap();

    loop {
        img_wnd.draw_image(&disp_img, None);
        hst_wnd.draw_hist(&hst, 0.0, 255.0, None);

        if img_wnd.is_closed().unwrap() == true { break; }
        if hst_wnd.is_closed().unwrap() == true { break; }
    }
}

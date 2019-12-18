use arrayfire::*;
use std::env;
use std::path::PathBuf;

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    set_device(0);
    info();

    let assets_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("arrayfire")
        .join("assets")
        .join("examples")
        .join("images");

    let img_wnd = Window::new(480, 640, String::from("Input Image"));
    img_wnd.set_position(100, 100);

    let hst_wnd = Window::new(512, 512, String::from("Input Image Histogram"));
    hst_wnd.set_position(600, 100);

    let man = load_image::<f32>(format!("{}/man.jpg", assets_dir.display()), false);
    let hst = histogram(&man, 256, 0.0, 255.0);

    let disp_img = div(&man, &constant(255_f32, man.dims()), false);

    loop {
        img_wnd.draw_image(&disp_img, None);
        hst_wnd.draw_hist(&hst, 0.0, 255.0, None);

        if img_wnd.is_closed() {
            break;
        }
        if hst_wnd.is_closed() {
            break;
        }
    }
}

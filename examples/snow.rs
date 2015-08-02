extern crate arrayfire as af;

use af::*;

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    set_device(0);
    info();

    let wnd = match Window::new(1280, 720, String::from("Snow")) {
        Ok(v) => v,
        Err(e)=> panic!("Window creation failed, exiting"),
    };

    let dims = Dim4::new(&[1280, 720, 3, 1]);

    loop {
        randu(dims, Aftype::F32).as_ref()
            .map(|arr| wnd.draw_image(arr, None));

        if wnd.is_closed().unwrap() == true { break; }
    }
}

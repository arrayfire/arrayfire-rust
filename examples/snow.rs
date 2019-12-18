use arrayfire::*;

#[allow(unused_variables)]
#[allow(unused_must_use)]
fn main() {
    set_device(0);
    info();

    let wnd = Window::new(1280, 720, String::from("Snow"));

    let dims = Dim4::new(&[1280, 720, 3, 1]);

    loop {
        wnd.draw_image(&randu::<f32>(dims), None);

        if wnd.is_closed() {
            break;
        }
    }
}

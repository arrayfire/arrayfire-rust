extern crate libc;

use defines::AfError;
use defines::Aftype;
use self::libc::{uint8_t, c_int, c_uint, c_double};

type MutWindow = *mut self::libc::c_ulonglong;
type Window    = self::libc::c_ulonglong;
type AfArray   = self::libc::c_longlong;
type Cell      = *const self::libc::c_void;

#[allow(dead_code)]
extern {
    fn af_create_window(out: MutWindow, w: c_int, h: c_int, title: *const u8) -> c_int;
    fn af_set_position(wnd: Window, x: c_uint, y: c_uint) -> c_int;
    fn af_set_title(wnd: Window, title: *const u8) -> c_int;
    fn af_draw_image(wnd: Window, arr: AfArray, props: Cell) -> c_int;
    fn af_draw_plot(wnd: Window, x: AfArray,  y: AfArray, props: Cell) -> c_int;
    fn af_grid(wnd: Window, rows: c_int, cols: c_int) -> c_int;
    fn af_show(wnd: Window) -> c_int;
    fn af_is_window_closed(out: *mut c_int, wnd: Window) -> c_int;
    fn af_destroy_window(wnd: Window) -> c_int;

    fn af_draw_hist(wnd: Window, x: AfArray,
                    minval: c_double, maxval: c_double, props: Cell) -> c_int;
}

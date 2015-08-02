extern crate libc;

use array::Array;
use defines::AfError;
use defines::ColorMap;
use self::libc::{c_int, c_uint, c_double};

type MutWndHandle = *mut self::libc::c_ulonglong;
type WndHandle    = self::libc::c_ulonglong;
type AfArray   = self::libc::c_longlong;
type CellPtr      = *const self::libc::c_void;

#[allow(dead_code)]
extern {
    fn af_create_window(out: MutWndHandle, w: c_int, h: c_int, title: *const u8) -> c_int;
    fn af_set_position(wnd: WndHandle, x: c_uint, y: c_uint) -> c_int;
    fn af_set_title(wnd: WndHandle, title: *const u8) -> c_int;
    fn af_draw_image(wnd: WndHandle, arr: AfArray, props: CellPtr) -> c_int;
    fn af_draw_plot(wnd: WndHandle, x: AfArray, y: AfArray, props: CellPtr) -> c_int;
    fn af_grid(wnd: WndHandle, rows: c_int, cols: c_int) -> c_int;
    fn af_show(wnd: WndHandle) -> c_int;
    fn af_is_window_closed(out: *mut c_int, wnd: WndHandle) -> c_int;
    fn af_destroy_window(wnd: WndHandle) -> c_int;

    fn af_draw_hist(wnd: WndHandle, x: AfArray,
                    minval: c_double, maxval: c_double, props: CellPtr) -> c_int;
}

#[repr(C)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub title: String,
    pub cmap: ColorMap,
}

#[derive(Clone)]
pub struct Window {
    handle: u64,
    row: i32,
    col: i32,
    cmap: ColorMap,
}

impl From<u64> for Window {
    fn from(t: u64) -> Window {
        Window {handle: t, row: -1, col: -1, cmap: ColorMap::DEFAULT}
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let err_val = af_destroy_window(self.handle);
            match err_val {
                0 => (),
                _ => panic!("Window object destruction failed with error code: {}", err_val),
            }
        }
    }
}

impl Window {
    #[allow(unused_mut)]
    pub fn new(width: i32, height: i32, title: String) -> Result<Window, AfError> {
        unsafe {
            let mut temp: u64 = 0;
            let err_val = af_create_window(&mut temp as MutWndHandle,
                                           width as c_int, height as c_int,
                                           title.as_bytes().as_ptr() as *const u8);
            match err_val {
                0 => Ok(Window::from(temp)),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn set_position(&self, x: u32, y: u32) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_position(self.handle as WndHandle, x as c_uint, y as c_uint);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn set_title(&self, title: String) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_title(self.handle as WndHandle,
                                       title.as_bytes().as_ptr() as *const u8);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn set_colormap(&mut self, cmap: ColorMap) {
        self.cmap = cmap;
    }

    pub fn is_closed(&self) -> Result<bool, AfError> {
        unsafe {
            let mut temp: i32 = 1;
            let err_val = af_is_window_closed(&mut temp as *mut c_int, self.handle as WndHandle);
            match err_val {
                0 => Ok(temp > 0),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn grid(&self, rows: i32, cols: i32) -> Result<(), AfError> {
        unsafe {
            let err_val = af_grid(self.handle as WndHandle, rows as c_int, cols as c_int);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    pub fn show(&mut self) -> Result<(), AfError> {
        unsafe {
            let err_val = af_show(self.handle as WndHandle);
            if err_val != 0 {
                return Err(AfError::from(err_val));
            }
            self.row = -1;
            self.col = -1;
            Ok(())
        }
    }

    pub fn set_view(&mut self, r: i32, c: i32) {
        self.row = r;
        self.col = c;
    }

    pub fn draw_image(&self, input: &Array, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_image(self.handle as WndHandle, input.get() as AfArray,
                                        cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering the image failed: {}", err_val),
            }
        }
    }

    pub fn draw_plot(&self, x: &Array, y: &Array, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_plot(self.handle as WndHandle,
                                       x.get() as AfArray,
                                       y.get() as AfArray,
                                       cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering the image failed: {}", err_val),
            }
        }
    }

    pub fn draw_hist(&self, hst: &Array, minval: f64, maxval: f64, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_hist(self.handle as WndHandle, hst.get() as AfArray,
                                       minval as c_double, maxval as c_double,
                                       cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering the image failed: {}", err_val),
            }
        }
    }
}

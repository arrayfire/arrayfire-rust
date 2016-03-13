extern crate libc;

use array::Array;
use defines::AfError;
use defines::{ColorMap, MarkerType};
use self::libc::{c_int, c_uint, c_double, c_char};
use std::ffi::CString;

type MutWndHandle = *mut self::libc::c_ulonglong;
type WndHandle    = self::libc::c_ulonglong;
type AfArray      = self::libc::c_longlong;
type CellPtr      = *const self::libc::c_void;

#[allow(dead_code)]
extern {
    fn af_create_window(out: MutWndHandle, w: c_int, h: c_int, title: *const c_char) -> c_int;
    fn af_set_position(wnd: WndHandle, x: c_uint, y: c_uint) -> c_int;
    fn af_set_title(wnd: WndHandle, title: *const c_char) -> c_int;
    fn af_set_size(wnd: WndHandle, w: c_uint, h: c_uint) -> c_int;
    fn af_set_visibility(wnd: WndHandle, is_visible: c_int) -> c_int;
    fn af_draw_image(wnd: WndHandle, arr: AfArray, props: CellPtr) -> c_int;
    fn af_draw_plot(wnd: WndHandle, x: AfArray, y: AfArray, props: CellPtr) -> c_int;
    fn af_draw_plot3(wnd: WndHandle, P: AfArray, props: CellPtr) -> c_int;
    fn af_draw_hist(wnd: WndHandle, x: AfArray,
                    minval: c_double, maxval: c_double, props: CellPtr) -> c_int;
    fn af_draw_surface(wnd: WndHandle, xvals: AfArray, yvals: AfArray, S: AfArray,
                       props: CellPtr) -> c_int;
    fn af_draw_scatter(wnd: WndHandle, x: AfArray, y: AfArray, marker: c_int, props: CellPtr) -> c_int;
    fn af_draw_scatter3(wnd: WndHandle, P: AfArray, marker: c_int, props: CellPtr) -> c_int;
    fn af_grid(wnd: WndHandle, rows: c_int, cols: c_int) -> c_int;
    fn af_show(wnd: WndHandle) -> c_int;
    fn af_is_window_closed(out: *mut c_int, wnd: WndHandle) -> c_int;
    fn af_destroy_window(wnd: WndHandle) -> c_int;
}

/// Represents a sub-view of Window
///
/// This struct is used in conjunction with [Window](./struct.Window.html) in multiview
/// mode to render multiple targets to sub-regions of a given window.
///
#[repr(C)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
    pub title: String,
    pub cmap: ColorMap,
}

/// Used to render [Array](./struct.Array.html) objects
///
/// The renderings can be either plots, histograms or simply just image displays.
/// A single window can also display multiple of the above renderings at the same time, which
/// is known as multiview mode. An example of that is given below.
///
/// # Examples
///
/// ```no_run
/// use arrayfire::{histogram, load_image, Window};
/// let mut wnd = Window::new(1280, 720, String::from("Image Histogram")).unwrap();
/// let img = match load_image("Path to image".to_string(), true/*If color image, 'false' otherwise*/) {
///     Ok(img) => img,
///     Err(err) => panic!("Image loading failed with error code {}", err),
/// };
/// let hst = histogram(&img, 256, 0 as f64, 255 as f64).unwrap();
///
/// loop {
///     wnd.grid(2, 1);
///
///     wnd.set_view(0, 0);
///     wnd.draw_image(&img, Some("Input Image".to_string()));
///
///     wnd.set_view(1, 0);
///     wnd.draw_hist(&hst, 0.0, 255.0, Some("Input Image Histogram".to_string()));
///
///     wnd.show();
///
///     if wnd.is_closed().unwrap() == true { break; }
/// }
/// ```
#[derive(Clone)]
pub struct Window {
    handle: u64,
    row: i32,
    col: i32,
    cmap: ColorMap,
}

/// Used to create Window object from native(ArrayFire) resource handle
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
    /// Creates new Window object
    #[allow(unused_mut)]
    pub fn new(width: i32, height: i32, title: String) -> Result<Window, AfError> {
        unsafe {
            let mut temp: u64 = 0;
            let cstr_ret = CString::new(title.as_bytes());
            match cstr_ret {
                Ok(cstr) => {
                    let err_val = af_create_window(&mut temp as MutWndHandle
                                                   , width as c_int, height as c_int
                                                   , cstr.to_bytes_with_nul().as_ptr() as *const c_char);
                    match err_val {
                        0 => Ok(Window::from(temp)),
                        _ => Err(AfError::from(err_val)),
                    }
                },
                Err(_)   => Err(AfError::ERR_INTERNAL),
            }
        }
    }

    /// Set window starting position on the screen
    pub fn set_position(&self, x: u32, y: u32) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_position(self.handle as WndHandle, x as c_uint, y as c_uint);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Set window title
    pub fn set_title(&self, title: String) -> Result<(), AfError> {
        unsafe {
            let cstr_ret = CString::new(title.as_bytes());
            match cstr_ret {
                Ok(cstr) => {
                    let err_val = af_set_title(self.handle as WndHandle
                                               , cstr.to_bytes_with_nul().as_ptr() as *const c_char);
                    match err_val {
                        0 => Ok(()),
                        _ => Err(AfError::from(err_val)),
                    }
                },
                Err(_)   => Err(AfError::ERR_INTERNAL),
            }
        }
    }

    /// Set window visibility
    ///
    /// # Parameters
    ///
    /// - `is_visible` is a boolean indicating whether window is to be hidden or brought into focus
    ///
    /// # Return Values
    ///
    /// None
    pub fn set_visibility(&self, is_visible: bool) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_visibility(self.handle as WndHandle, is_visible as c_int);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Set window size
    ///
    /// # Parameters
    ///
    /// - `w` is the target width of window
    /// - `h` is the target height of window
    pub fn set_size(&self, w: u32, h: u32) -> Result<(), AfError> {
        unsafe {
            let err_val = af_set_size(self.handle as WndHandle, w as c_uint, h as c_uint);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Set color map to be used for rendering image, it can take one of the values of enum
    /// [ColorMap](./enum.ColorMap.html)
    pub fn set_colormap(&mut self, cmap: ColorMap) {
        self.cmap = cmap;
    }

    /// Returns true if the window close is triggered by the user
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

    /// Used to setup display layout in multiview mode
    pub fn grid(&self, rows: i32, cols: i32) -> Result<(), AfError> {
        unsafe {
            let err_val = af_grid(self.handle as WndHandle, rows as c_int, cols as c_int);
            match err_val {
                0 => Ok(()),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Used in multiview mode to swap back buffer with front buffer to show the recently rendered
    /// frame
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

    /// Used in multiview mode to set the current sub-region to which the subsequence draw call
    /// renders to
    pub fn set_view(&mut self, r: i32, c: i32) {
        self.row = r;
        self.col = c;
    }

    /// Render given Array as an image
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

    /// Render given two Array's `x` and `y` as a 2d line plot
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
                _ => panic!("Rendering 2d line plot failed: {}", err_val),
            }
        }
    }

    /// Render give Arrays of points as a 3d line plot
    pub fn draw_plot3(&self, points: &Array, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_plot3(self.handle as WndHandle, points.get() as AfArray,
                                        cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering 3d line plot failed: {}", err_val),
            }
        }
    }

    /// Render given Array as a histogram
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
                _ => panic!("Rendering histogram failed: {}", err_val),
            }
        }
    }

    /// Render give Arrays as 3d surface
    pub fn draw_surface(&self, xvals: &Array, yvals: &Array, zvals: &Array, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_surface(self.handle as WndHandle,
                                          xvals.get() as AfArray,
                                          yvals.get() as AfArray,
                                          zvals.get() as AfArray,
                                          cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering surface failed: {}", err_val),
            }
        }
    }

    /// Render give Arrays as 2d scatter plot
    pub fn draw_scatter(&self, xvals: &Array, yvals: &Array, marker: MarkerType, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_scatter(self.handle as WndHandle,
                                          xvals.get() as AfArray,
                                          yvals.get() as AfArray,
                                          marker as c_int,
                                          cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering scatter failed: {}", err_val),
            }
        }
    }

    /// Render give Array as 3d scatter plot
    pub fn draw_scatter3(&self, vals: &Array, marker: MarkerType, title: Option<String>) {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row)
        };
        let cprops = &Cell {row: self.row, col: self.col, title: tstr.clone(), cmap: self.cmap};
        unsafe {
            let err_val = af_draw_scatter3(self.handle as WndHandle,
                                           vals.get() as AfArray,
                                           marker as c_int,
                                           cprops as *const Cell as CellPtr);
            match err_val {
                0 => (),
                _ => panic!("Rendering scatter3 failed: {}", err_val),
            }
        }
    }
}

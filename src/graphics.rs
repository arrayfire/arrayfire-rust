extern crate libc;

use self::libc::{c_char, c_double, c_float, c_int, c_uint};
use crate::array::Array;
use crate::defines::AfError;
use crate::defines::{ColorMap, MarkerType};
use crate::error::HANDLE_ERROR;
use crate::util::{AfArray, CellPtr, HasAfEnum, MutWndHandle, WndHandle};
use std::ffi::CString;
use std::ptr;

#[allow(dead_code)]
extern "C" {
    fn af_create_window(out: MutWndHandle, w: c_int, h: c_int, title: *const c_char) -> c_int;

    fn af_set_position(wnd: WndHandle, x: c_uint, y: c_uint) -> c_int;
    fn af_set_title(wnd: WndHandle, title: *const c_char) -> c_int;
    fn af_set_size(wnd: WndHandle, w: c_uint, h: c_uint) -> c_int;
    fn af_set_visibility(wnd: WndHandle, is_visible: c_int) -> c_int;

    fn af_set_axes_titles(
        wnd: WndHandle,
        xtitle: *const c_char,
        ytitle: *const c_char,
        ztitle: *const c_char,
        props: CellPtr,
    ) -> c_int;
    fn af_set_axes_limits_compute(
        wnd: WndHandle,
        x: AfArray,
        y: AfArray,
        z: AfArray,
        exact: c_int,
        props: CellPtr,
    ) -> c_int;
    fn af_set_axes_limits_2d(
        wnd: WndHandle,
        xmin: c_float,
        xmax: c_float,
        ymin: c_float,
        ymax: c_float,
        exact: c_int,
        props: CellPtr,
    ) -> c_int;
    fn af_set_axes_limits_3d(
        wnd: WndHandle,
        xmin: c_float,
        xmax: c_float,
        ymin: c_float,
        ymax: c_float,
        zmin: c_float,
        zmax: c_float,
        exact: c_int,
        props: CellPtr,
    ) -> c_int;

    fn af_draw_image(wnd: WndHandle, arr: AfArray, props: CellPtr) -> c_int;
    fn af_draw_hist(
        wnd: WndHandle,
        x: AfArray,
        minval: c_double,
        maxval: c_double,
        props: CellPtr,
    ) -> c_int;
    fn af_draw_surface(
        wnd: WndHandle,
        xvals: AfArray,
        yvals: AfArray,
        S: AfArray,
        props: CellPtr,
    ) -> c_int;

    fn af_draw_plot_2d(wnd: WndHandle, x: AfArray, y: AfArray, props: CellPtr) -> c_int;
    fn af_draw_plot_3d(wnd: WndHandle, x: AfArray, y: AfArray, z: AfArray, props: CellPtr)
        -> c_int;
    fn af_draw_plot_nd(wnd: WndHandle, P: AfArray, props: CellPtr) -> c_int;

    fn af_draw_scatter_2d(
        wnd: WndHandle,
        x: AfArray,
        y: AfArray,
        marker: c_int,
        props: CellPtr,
    ) -> c_int;
    fn af_draw_scatter_3d(
        wnd: WndHandle,
        x: AfArray,
        y: AfArray,
        z: AfArray,
        marker: c_int,
        props: CellPtr,
    ) -> c_int;
    fn af_draw_scatter_nd(wnd: WndHandle, P: AfArray, marker: c_int, props: CellPtr) -> c_int;

    fn af_draw_vector_field_2d(
        wnd: WndHandle,
        xpnts: AfArray,
        ypnts: AfArray,
        xdirs: AfArray,
        ydirs: AfArray,
        props: CellPtr,
    ) -> c_int;
    fn af_draw_vector_field_3d(
        wnd: WndHandle,
        xpnts: AfArray,
        ypnts: AfArray,
        xdirs: AfArray,
        ydirs: AfArray,
        zdirs: AfArray,
        zdirs: AfArray,
        props: CellPtr,
    ) -> c_int;
    fn af_draw_vector_field_nd(
        wnd: WndHandle,
        pnts: AfArray,
        dirs: AfArray,
        props: CellPtr,
    ) -> c_int;

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
    pub title: *const c_char,
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
/// ```rust,no_run
/// use arrayfire::{histogram, load_image, Window};
/// let mut wnd = Window::new(1280, 720, String::from("Image Histogram"));
/// let img = load_image::<f32>("Path to image".to_string(), true/*If color image, 'false' otherwise*/);
/// let hst = histogram(&img, 256, 0 as f64, 255 as f64);
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
///     if wnd.is_closed() == true { break; }
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
    fn from(t: u64) -> Self {
        Self {
            handle: t,
            row: -1,
            col: -1,
            cmap: ColorMap::DEFAULT,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            let err_val = af_destroy_window(self.handle);
            match err_val {
                0 => (),
                _ => panic!(
                    "Window object destruction failed with error code: {}",
                    err_val
                ),
            }
        }
    }
}

impl Window {
    /// Creates new Window object
    ///
    /// # Parameters
    ///
    /// - `width` is width of the window
    /// - `height` is the height of window
    /// - `title` is the string displayed on window title bar
    ///
    /// # Return Values
    ///
    /// Window Object
    #[allow(unused_mut)]
    pub fn new(width: i32, height: i32, title: String) -> Self {
        unsafe {
            let mut temp: u64 = 0;
            let cstr_ret = CString::new(title);
            match cstr_ret {
                Ok(cstr) => {
                    let err_val = af_create_window(
                        &mut temp as MutWndHandle,
                        width as c_int,
                        height as c_int,
                        cstr.as_ptr(),
                    );
                    HANDLE_ERROR(AfError::from(err_val));
                    Self::from(temp)
                }
                Err(_) => {
                    panic!("String creation failed while prepping params for window creation.")
                }
            }
        }
    }

    /// Set window starting position on the screen
    ///
    /// # Parameters
    ///
    /// - `x` is the horiontal coordinate where window is to be placed
    /// - `y` is the vertical coordinate where window is to be placed
    pub fn set_position(&self, x: u32, y: u32) {
        unsafe {
            let err_val = af_set_position(self.handle as WndHandle, x as c_uint, y as c_uint);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set window title
    ///
    /// # Parameters
    ///
    /// - `title` is the string to be displayed on window title bar
    pub fn set_title(&self, title: String) {
        unsafe {
            let cstr_ret = CString::new(title);
            match cstr_ret {
                Ok(cstr) => {
                    let err_val = af_set_title(self.handle as WndHandle, cstr.as_ptr());
                    HANDLE_ERROR(AfError::from(err_val));
                }
                Err(_) => HANDLE_ERROR(AfError::ERR_INTERNAL),
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
    pub fn set_visibility(&self, is_visible: bool) {
        unsafe {
            let err_val = af_set_visibility(self.handle as WndHandle, is_visible as c_int);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set window size
    ///
    /// # Parameters
    ///
    /// - `w` is the target width of window
    /// - `h` is the target height of window
    pub fn set_size(&self, w: u32, h: u32) {
        unsafe {
            let err_val = af_set_size(self.handle as WndHandle, w as c_uint, h as c_uint);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set color map to be used for rendering image, it can take one of the values of enum
    /// [ColorMap](./enum.ColorMap.html)
    pub fn set_colormap(&mut self, cmap: ColorMap) {
        self.cmap = cmap;
    }

    /// Returns true if the window close is triggered by the user
    pub fn is_closed(&self) -> bool {
        unsafe {
            let mut temp: i32 = 1;
            let err_val = af_is_window_closed(&mut temp as *mut c_int, self.handle as WndHandle);
            HANDLE_ERROR(AfError::from(err_val));
            temp > 0
        }
    }

    /// Setup display layout in multiview mode
    ///
    /// # Parameters
    ///
    /// - `rows` is the number of rows into which whole window is split into in multiple view mode
    /// - `cols` is the number of cols into which whole window is split into in multiple view mode
    pub fn grid(&self, rows: i32, cols: i32) {
        unsafe {
            let err_val = af_grid(self.handle as WndHandle, rows as c_int, cols as c_int);
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Used in multiview mode to swap back buffer with front buffer to show the recently rendered
    /// frame
    pub fn show(&mut self) {
        unsafe {
            let err_val = af_show(self.handle as WndHandle);
            HANDLE_ERROR(AfError::from(err_val));
            self.row = -1;
            self.col = -1;
        }
    }

    /// Set the current sub-region to render
    ///
    /// This function is only to be used into multiview mode
    ///
    /// # Parameters
    ///
    /// - `r` is the target row id
    /// - `c` is the target row id
    pub fn set_view(&mut self, r: i32, c: i32) {
        self.row = r;
        self.col = c;
    }

    /// Set chart axes titles
    ///
    /// # Parameters
    ///
    /// - `xlabel` is x axis title
    /// - `ylabel` is y axis title
    /// - `zlabel` is z axis title
    pub fn set_axes_titles(&mut self, xlabel: String, ylabel: String, zlabel: String) {
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: ptr::null(),
            cmap: self.cmap,
        };
        let xstr = CString::new(xlabel).unwrap();
        let ystr = CString::new(ylabel).unwrap();
        let zstr = CString::new(zlabel).unwrap();
        unsafe {
            let err_val = af_set_axes_titles(
                self.handle as WndHandle,
                xstr.as_ptr(),
                ystr.as_ptr(),
                zstr.as_ptr(),
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set chart axes limits by computing limits from data
    ///
    /// In multiple view (grid) mode, setting limits will effect the chart that is currently
    /// active via set_view call
    ///
    /// # Parameters
    ///
    /// - `xrange` is set of all x values to compute min/max for x axis
    /// - `yrange` is set of all y values to compute min/max for y axis
    /// - `zrange` is set of all z values to compute min/max for z axis. If None is passed to
    ///    this paramter, 2d chart limits are set.
    /// - `exact` indicates if the exact min/max values from `xrange`, `yrange` and `zrange`
    ///    are to extracted. If exact is false then the most significant digit is rounded up
    ///    to next power of 2 and the magnitude remains the same.
    pub fn set_axes_limits_compute<T>(
        &mut self,
        xrange: &Array<T>,
        yrange: &Array<T>,
        zrange: Option<&Array<T>>,
        exact: bool,
    ) where
        T: HasAfEnum,
    {
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: ptr::null(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_set_axes_limits_compute(
                self.handle as WndHandle,
                xrange.get() as AfArray,
                yrange.get() as AfArray,
                match zrange {
                    Some(z) => z.get() as AfArray,
                    None => 0,
                },
                exact as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set 2d chart axes limits
    ///
    /// In multiple view (grid) mode, setting limits will effect the chart that is currently
    /// active via set_view call
    ///
    /// # Parameters
    ///
    /// - `xmin` is minimum value on x axis
    /// - `xmax` is maximum value on x axis
    /// - `ymin` is minimum value on y axis
    /// - `ymax` is maximum value on y axis
    /// - `exact` indicates if the exact min/max values from `xrange`, `yrange` and `zrange`
    ///    are to extracted. If exact is false then the most significant digit is rounded up
    ///    to next power of 2 and the magnitude remains the same.
    pub fn set_axes_limits_2d(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32, exact: bool) {
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: ptr::null(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_set_axes_limits_2d(
                self.handle as WndHandle,
                xmin as c_float,
                xmax as c_float,
                ymin as c_float,
                ymax as c_float,
                exact as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Set 3d chart axes limits
    ///
    /// In multiple view (grid) mode, setting limits will effect the chart that is currently
    /// active via set_view call
    ///
    /// # Parameters
    ///
    /// - `xmin` is minimum value on x axis
    /// - `xmax` is maximum value on x axis
    /// - `ymin` is minimum value on y axis
    /// - `ymax` is maximum value on y axis
    /// - `zmin` is minimum value on z axis
    /// - `zmax` is maximum value on z axis
    /// - `exact` indicates if the exact min/max values from `xrange`, `yrange` and `zrange`
    ///    are to extracted. If exact is false then the most significant digit is rounded up
    ///    to next power of 2 and the magnitude remains the same.
    pub fn set_axes_limits_3d(
        &mut self,
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
        exact: bool,
    ) {
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: ptr::null(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_set_axes_limits_3d(
                self.handle as WndHandle,
                xmin as c_float,
                xmax as c_float,
                ymin as c_float,
                ymax as c_float,
                zmin as c_float,
                zmax as c_float,
                exact as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Array as an image
    ///
    /// # Parameters
    ///
    /// - `input` image
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_image<T>(&self, input: &Array<T>, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_image(
                self.handle as WndHandle,
                input.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given two Array's `x` and `y` as a 2d line plot
    ///
    /// # Parameters
    ///
    /// - `x` is the x coordinates of the plot
    /// - `y` is the y coordinates of the plot
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_plot2<T>(&self, x: &Array<T>, y: &Array<T>, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_plot_2d(
                self.handle as WndHandle,
                x.get() as AfArray,
                y.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Array's `x`, `y` and `z` as a 3d line plot
    ///
    /// # Parameters
    ///
    /// - `x` is the x coordinates of the plot
    /// - `y` is the y coordinates of the plot
    /// - `z` is the z coordinates of the plot
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_plot3<T>(&self, x: &Array<T>, y: &Array<T>, z: &Array<T>, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_plot_3d(
                self.handle as WndHandle,
                x.get() as AfArray,
                y.get() as AfArray,
                z.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render give Arrays of points as a 3d line plot
    ///
    /// # Parameters
    ///
    /// - `points` is an Array containing list of points of plot
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_plot<T>(&self, points: &Array<T>, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_plot_nd(
                self.handle as WndHandle,
                points.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Array as a histogram
    ///
    /// # Parameters
    ///
    /// - `hst` is an Array containing histogram data
    /// - `minval` is the minimum bin value of histogram
    /// - `maxval` is the maximum bin value of histogram
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_hist<T>(&self, hst: &Array<T>, minval: f64, maxval: f64, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_hist(
                self.handle as WndHandle,
                hst.get() as AfArray,
                minval as c_double,
                maxval as c_double,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render give Arrays as 3d surface
    ///
    /// # Parameters
    ///
    /// - `x` is the x coordinates of the surface plot
    /// - `y` is the y coordinates of the surface plot
    /// - `z` is the z coordinates of the surface plot
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_surface<T>(
        &self,
        xvals: &Array<T>,
        yvals: &Array<T>,
        zvals: &Array<T>,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_surface(
                self.handle as WndHandle,
                xvals.get() as AfArray,
                yvals.get() as AfArray,
                zvals.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Arrays as 2d scatter plot
    ///
    /// # Parameters
    ///
    /// - `xvals` is the x coordinates of the scatter plot
    /// - `yvals` is the y coordinates of the scatter plot
    /// - `marker` is of enum type [MarkerType](./enum.MarkerType.html)
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_scatter2<T>(
        &self,
        xvals: &Array<T>,
        yvals: &Array<T>,
        marker: MarkerType,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_scatter_2d(
                self.handle as WndHandle,
                xvals.get() as AfArray,
                yvals.get() as AfArray,
                marker as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Arrays as 3d scatter plot
    ///
    /// # Parameters
    ///
    /// - `xvals` is the x coordinates of the scatter plot
    /// - `yvals` is the y coordinates of the scatter plot
    /// - `zvals` is the z coordinates of the scatter plot
    /// - `marker` is of enum type [MarkerType](./enum.MarkerType.html)
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_scatter3<T>(
        &self,
        xvals: &Array<T>,
        yvals: &Array<T>,
        zvals: &Array<T>,
        marker: MarkerType,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_scatter_3d(
                self.handle as WndHandle,
                xvals.get() as AfArray,
                yvals.get() as AfArray,
                zvals.get() as AfArray,
                marker as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render give Array as 3d scatter plot
    ///
    /// # Parameters
    ///
    /// - `points` is an Array containing list of points of plot
    /// - `marker` is of enum type [MarkerType](./enum.MarkerType.html)
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_scatter<T>(&self, vals: &Array<T>, marker: MarkerType, title: Option<String>)
    where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_scatter_nd(
                self.handle as WndHandle,
                vals.get() as AfArray,
                marker as c_int,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Arrays as 2d vector field
    ///
    /// # Parameters
    ///
    /// - `xpnts` is an Array containing list of x coordinates
    /// - `xdirs` is an Array containing direction component of x coord
    /// - `ypnts` is an Array containing list of y coordinates
    /// - `ydirs` is an Array containing direction component of y coord
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_vector_field2<T>(
        &self,
        xpnts: &Array<T>,
        ypnts: &Array<T>,
        xdirs: &Array<T>,
        ydirs: &Array<T>,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_vector_field_2d(
                self.handle as WndHandle,
                xpnts.get() as AfArray,
                ypnts.get() as AfArray,
                xdirs.get() as AfArray,
                ydirs.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Arrays as 3d vector field
    ///
    /// # Parameters
    ///
    /// - `xpnts` is an Array containing list of x coordinates
    /// - `xdirs` is an Array containing direction component of x coord
    /// - `ypnts` is an Array containing list of y coordinates
    /// - `ydirs` is an Array containing direction component of y coord
    /// - `zpnts` is an Array containing list of z coordinates
    /// - `zdirs` is an Array containing direction component of z coord
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_vector_field3<T>(
        &self,
        xpnts: &Array<T>,
        ypnts: &Array<T>,
        zpnts: &Array<T>,
        xdirs: &Array<T>,
        ydirs: &Array<T>,
        zdirs: &Array<T>,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_vector_field_3d(
                self.handle as WndHandle,
                xpnts.get() as AfArray,
                ypnts.get() as AfArray,
                zpnts.get() as AfArray,
                xdirs.get() as AfArray,
                ydirs.get() as AfArray,
                zdirs.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }

    /// Render given Array as vector field
    ///
    /// # Parameters
    ///
    /// - `points` is an Array containing list of coordinates of vector field
    /// - `directions` is an Array containing directions at the coordinates specified in `points`
    /// Array.
    /// - `title` parameter has effect only in multiview mode, where this string
    ///    is displayed as the respective cell/view title.
    pub fn draw_vector_field<T>(
        &self,
        points: &Array<T>,
        directions: &Array<T>,
        title: Option<String>,
    ) where
        T: HasAfEnum,
    {
        let tstr = match title {
            Some(s) => s,
            None => format!("Cell({},{}))", self.col, self.row),
        };
        let tstr = CString::new(tstr).unwrap();
        let cprops = &Cell {
            row: self.row,
            col: self.col,
            title: tstr.as_ptr(),
            cmap: self.cmap,
        };
        unsafe {
            let err_val = af_draw_vector_field_nd(
                self.handle as WndHandle,
                points.get() as AfArray,
                directions.get() as AfArray,
                cprops as *const Cell as CellPtr,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
    }
}

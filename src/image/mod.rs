extern crate libc;

use array::Array;
use defines::AfError;
use defines::Aftype;
use defines::BorderType;
use defines::ColorSpace;
use defines::Connectivity;
use defines::InterpType;
use self::libc::{uint8_t, c_uint, c_int, c_float, c_double};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;

#[allow(dead_code)]
extern {
    fn af_gradient(dx: MutAfArray, dy: MutAfArray, arr: AfArray) -> c_int;
    fn af_load_image(out: MutAfArray, filename: *const u8, iscolor: c_int) -> c_int;
    fn af_save_image(filename: *const u8, input: AfArray) -> c_int;

    fn af_resize(out: MutAfArray, input: AfArray,
                 odim0: DimT, odim1: DimT, method: uint8_t) -> c_int;

    fn af_transform(out: MutAfArray, input: AfArray, trans: AfArray,
                    odim0: DimT, odim1: DimT, method: uint8_t, is_inverse: c_int) -> c_int;

    fn af_rotate(out: MutAfArray, input: AfArray, theta: c_float, crop: c_int,
                 method: uint8_t) -> c_int;

    fn af_translate(out: MutAfArray, input: AfArray, trans0: c_float, trans1: c_float,
                    odim0: DimT, odim1: DimT, method: uint8_t) -> c_int;

    fn af_scale(out: MutAfArray, input: AfArray, scale0: c_float, scale1: c_float,
                odim0: DimT, odim1: DimT, method: uint8_t) -> c_int;

    fn af_skew(out: MutAfArray, input: AfArray, skew0: c_float, skew1: c_float,
               odim0: DimT, odim1: DimT, method: uint8_t, is_inverse: c_int) -> c_int;

    fn af_histogram(out: MutAfArray, input: AfArray, nbins: c_uint,
                    minval: c_double, maxval: c_double) -> c_int;

    fn af_dilate(out: MutAfArray, input: AfArray, mask: AfArray) -> c_int;
    fn af_dilate3(out: MutAfArray, input: AfArray, mask: AfArray) -> c_int;
    fn af_erode(out: MutAfArray, input: AfArray, mask: AfArray) -> c_int;
    fn af_erode3(out: MutAfArray, input: AfArray, mask: AfArray) -> c_int;
    fn af_regions(out: MutAfArray, input: AfArray, conn: uint8_t, aftype: uint8_t) -> c_int;
    fn af_sobel_operator(dx: MutAfArray, dy: MutAfArray, i: AfArray, ksize: c_uint) -> c_int;
    fn af_rgb2gray(out: MutAfArray, input: AfArray, r: c_float, g: c_float, b: c_float) -> c_int;
    fn af_gray2rgb(out: MutAfArray, input: AfArray, r: c_float, g: c_float, b: c_float) -> c_int;
    fn af_hist_equal(out: MutAfArray, input: AfArray, hist: AfArray) -> c_int;
    fn af_hsv2rgb(out: MutAfArray, input: AfArray) -> c_int;
    fn af_rgb2hsv(out: MutAfArray, input: AfArray) -> c_int;

    fn af_bilateral(out: MutAfArray, input: AfArray,
                    sp_sig: c_float, ch_sig: c_float, iscolor: c_int) -> c_int;

    fn af_mean_shift(out: MutAfArray, input: AfArray, sp_sig: c_float,
                     ch_sig: c_float, iter: c_uint, iscolor: c_int) -> c_int;

    fn af_medfilt(out: MutAfArray, input: AfArray,
                  wlen: DimT, wwid: DimT, etype: uint8_t) -> c_int;

    fn af_minfilt(out: MutAfArray, input: AfArray,
                  wlen: DimT, wwid: DimT, etype: uint8_t) -> c_int;

    fn af_maxfilt(out: MutAfArray, input: AfArray,
                  wlen: DimT, wwid: DimT, etype: uint8_t) -> c_int;

    fn af_gaussian_kernel(out: MutAfArray, rows: c_int, cols: c_int,
                          sigma_r: c_double, sigma_c: c_double) -> c_int;

    fn af_color_space(out: MutAfArray, input: AfArray,
                      tospace: uint8_t, fromspace: uint8_t) -> c_int;
}

#[allow(unused_mut)]
pub fn gradient(input: &Array) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut dx: i64 = 0;
        let mut dy: i64 = 0;
        let err_val = af_gradient(&mut dx as MutAfArray, &mut dy as MutAfArray,
                                  input.get() as AfArray);
        match err_val {
            0 => Ok((Array::from(dx), Array::from(dy))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn load_image(filename: &[u8], is_color: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_load_image(&mut temp as MutAfArray,
                                    filename.as_ptr() as *const u8, is_color as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn save_image(filename: &[u8], input: &Array) -> Result<(), AfError> {
    unsafe {
        let err_val = af_save_image(filename.as_ptr() as *const u8, input.get() as AfArray);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn resize(input: &Array, odim0: i64, odim1: i64,
              method: InterpType) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_resize(&mut temp as MutAfArray, input.get() as AfArray,
                                odim0 as DimT, odim1 as DimT, method as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn transform(input: &Array, trans: &Array, odim0: i64, odim1: i64,
                 method: InterpType, is_inverse: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_transform(&mut temp as MutAfArray,
                                   input.get() as AfArray, trans.get() as AfArray,
                                   odim0 as DimT, odim1 as DimT,
                                   method as uint8_t, is_inverse as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn rotate(input: &Array, theta: f64, crop: bool,
              method: InterpType) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_rotate(&mut temp as MutAfArray, input.get() as AfArray,
                                theta as c_float, crop as c_int, method as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! trans_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, p0: f32, p1: f32,
                        odim0: i64, odim1: i64,
                        method: InterpType) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        input.get() as AfArray,
                                        p0 as c_float, p1 as c_float,
                                        odim0 as DimT, odim1 as DimT,
                                        method as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

trans_func_def!(translate, af_translate);
trans_func_def!(scale, af_scale);

#[allow(unused_mut)]
pub fn skew(input: &Array, skew0: f32, skew1: f32, odim0: i64, odim1: i64,
            method: InterpType, is_inverse: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_skew(&mut temp as MutAfArray, input.get() as AfArray,
                              skew0 as c_float, skew1 as c_float,
                              odim0 as DimT, odim1 as DimT,
                              method as uint8_t, is_inverse as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn histogram(input: &Array, nbins: u32,
                 minval: f64, maxval: f64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_histogram(&mut temp as MutAfArray, input.get() as AfArray,
                                   nbins as c_uint, minval as c_double, maxval as c_double);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! morph_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, mask: &Array) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray,
                                        input.get() as AfArray, mask.get() as AfArray);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

morph_func_def!(dilate, af_dilate);
morph_func_def!(erode, af_erode);
morph_func_def!(dilate3, af_dilate3);
morph_func_def!(erode3, af_erode3);

#[allow(unused_mut)]
pub fn bilateral(input: &Array, spatial_sigma: f32, chromatic_sigma: f32,
                 iscolor: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_bilateral(&mut temp as MutAfArray, input.get() as AfArray,
                                   spatial_sigma as c_float, chromatic_sigma as c_float,
                                   iscolor as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn mean_shift(input: &Array, spatial_sigma: f32, chromatic_sigma: f32,
                 iter: u32, iscolor: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_mean_shift(&mut temp as MutAfArray, input.get() as AfArray,
                                    spatial_sigma as c_float, chromatic_sigma as c_float,
                                    iter as c_uint, iscolor as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! filt_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, wlen: i64, wwid: i64,
                        etype: BorderType) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, input.get() as AfArray,
                                        wlen as DimT, wwid as DimT, etype as uint8_t);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

filt_func_def!(medfilt, af_medfilt);
filt_func_def!(minfilt, af_minfilt);
filt_func_def!(maxfilt, af_maxfilt);

#[allow(unused_mut)]
pub fn gaussian_kernel(rows: i32, cols: i32,
                       sigma_r: f64, sigma_c: f64) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_gaussian_kernel(&mut temp as MutAfArray,
                                         rows as c_int, cols as c_int,
                                         sigma_r as c_double, sigma_c as c_double);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn color_space(input: &Array,
                   tospace: ColorSpace, fromspace: ColorSpace) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_color_space(&mut temp as MutAfArray, input.get() as AfArray,
                                     tospace as uint8_t, fromspace as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn regions(input: &Array, conn: Connectivity, aftype: Aftype) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_regions(&mut temp as MutAfArray, input.get() as AfArray,
                                 conn as uint8_t, aftype as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn sobel(input: &Array, ker_size: u32) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut dx: i64 = 0;
        let mut dy: i64 = 0;
        let err_val = af_sobel_operator(&mut dx as MutAfArray, &mut dy as MutAfArray,
                                        input.get() as AfArray, ker_size as c_uint);
        match err_val {
            0 => Ok((Array::from(dx), Array::from(dy))),
            _ => Err(AfError::from(err_val)),
        }
    }
}

#[allow(unused_mut)]
pub fn hist_equal(input: &Array, hist: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_hist_equal(&mut temp as MutAfArray,
                                    input.get() as AfArray, hist.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

macro_rules! grayrgb_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array, r: f32, g: f32, b: f32) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, input.get() as AfArray,
                                        r as c_float, g as c_float, b as c_float);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

grayrgb_func_def!(rgb2gray, af_rgb2gray);
grayrgb_func_def!(gray2rgb, af_gray2rgb);

macro_rules! hsvrgb_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        #[allow(unused_mut)]
        pub fn $fn_name(input: &Array) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, input.get() as AfArray);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

hsvrgb_func_def!(hsv2rgb, af_hsv2rgb);
hsvrgb_func_def!(rgb2hsv, af_rgb2hsv);

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

/// Calculate the gradients
///
/// The gradients along the first and second dimensions are calculated simultaneously.
///
/// # Parameters
///
/// - `input` is the input Array
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array is `dx` which is the gradient along the 1st dimension.
///
/// The second Array is `dy` which is the gradient along the 2nd dimension.
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

/// Load Image into Array
///
/// # Parameters
///
/// - `filename` is aboslute path of the image to be loaded.
/// - `is_color` indicates if the image file at given path is color or gray scale.
///
/// # Return Arrays
///
/// An Array with pixel values loaded from the image
#[allow(unused_mut)]
pub fn load_image(filename: String, is_color: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_load_image(&mut temp as MutAfArray,
                                    filename.clone().as_bytes().as_ptr() as *const u8,
                                    is_color as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Save an Array to an image file
///
/// # Parameters
///
/// - `filename` is the abolute path(includes filename) at which input Array is going to be saved
/// - `input` is the Array to be stored into the image file
#[allow(unused_mut)]
pub fn save_image(filename: String, input: &Array) -> Result<(), AfError> {
    unsafe {
        let err_val = af_save_image(filename.clone().as_bytes().as_ptr() as *const u8,
                                    input.get() as AfArray);
        match err_val {
            0 => Ok(()),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Resize an Image
///
/// Resizing an input image can be done using either NEAREST or BILINEAR interpolations.
/// Nearest interpolation will pick the nearest value to the location, whereas bilinear
/// interpolation will do a weighted interpolation for calculate the new size.
///
/// This function does not differentiate between images and data. As long as the array is defined
/// and the output dimensions are not 0, it will resize any type or size of array.
///
/// # Parameters
///
/// - `input` is the image to be resized
/// - `odim0` is the output height
/// - `odim1` is the output width
/// - `method` indicates which interpolation method to use for resizing. It uses enum
/// [InterpType](./enum.InterpType.html) to identify the interpolation method.
///
/// # Return Values
///
/// Resized Array
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

/// Transform(Affine) an Image
///
/// The transform function uses an affine transform matrix to tranform an input image into a new
/// one. The transform matrix tf is a 3x2 matrix of type float. The matrix operation is applied to each
/// location (x, y) that is then transformed to (x', y') of the new array. Hence the transformation
/// is an element-wise operation.
///
/// The operation is as below: tf = [r00 r10 r01 r11 t0 t1]
///
/// x' = x * r00 + y * r01 + t0; y' = x * r10 + y * r11 + t1;
///
/// Interpolation types of NEAREST, LINEAR, BILINEAR and CUBIC are allowed. Affine transforms can be used for various purposes. [translate](./fn.translate.html), [scale](./fn.scale.html) and [skew](./fn.skew.html) are
/// specializations of the transform function.
///
/// This function can also handle batch operations.
///
/// # Parameters
///
/// - `input` is the image to be resized
/// - `trans` is the transformation matrix to be used for image transformation
/// - `odim0` is the output height
/// - `odim1` is the output width
/// - `method` indicates which interpolation method to use for resizing. It uses enum
/// [InterpType](./enum.InterpType.html) to identify the interpolation method.
/// - `is_inverse` indicates if to apply inverse/forward transform
///
/// # Return Values
///
/// Resized Array
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

/// Rotate an Image
///
/// Rotating an input image can be done using either NEAREST or BILINEAR interpolations.
/// Nearest interpolation will pick the nearest value to the location, whereas bilinear
/// interpolation will do a weighted interpolation for calculate the new size.
///
/// This function does not differentiate between images and data. As long as the array is defined,
/// it will rotate any type or size of array.
///
/// The crop option allows you to choose whether to resize the image. If crop is set to false, ie.
/// the entire rotated image will be a part of the array and the new array size will be greater
/// than or equal to the input array size. If crop is set to true, then the new array size is same
/// as the input array size and the data that falls outside the boundaries of the array is
/// discarded.
///
/// Any location of the rotated array that does not map to a location of the input array is set to
/// 0.
///
/// # Parameters
///
/// - `input` is the input image
/// - `theta` is the amount of angle (in radians) image should be rotated
/// - `crop` indicates if the rotated image has to be cropped to original size
/// - `method` indicates which interpolation method to use for rotating the image. It uses enum
/// [InterpType](./enum.InterpType.html) to identify the interpolation method.
///
/// # Return Values
///
/// Rotated Array
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

/// Skew an image
///
/// Skew function skews the input array along dim0 by skew0 and along dim1 by skew1. The skew
/// areguments are in radians. Skewing the data means the data remains parallel along 1 dimensions
/// but the other dimensions gets moved along based on the angle. If both skew0 and skew1 are
/// specified, then the data will be skewed along both directions. Explicit output dimensions
/// can be specified using odim0 and odim1. All new values that do not map to a location of the input array are set to 0.
///
/// Skew is a special case of the [transform](./fn.transform.html) function.
///
/// # Parameters
///
/// - `input` is the image to be skewed
/// - `skew0` is the factor by which data is skewed along first dimension
/// - `skew1` is the factor by which data is skewed along second dimension
/// - `odim0` is the output length along first dimension
/// - `odim1` is the output length along second dimension
/// - `method` indicates which interpolation method to use for rotating the image. It uses enum
/// [InterpType](./enum.InterpType.html) to identify the interpolation method.
/// - `is_inverse` indicates if to apply inverse/forward transform
///
/// # Return Values
///
/// Skewed Image
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

/// Compute Histogram of an Array
///
/// A histogram is a representation of the distribution of given data. This representation is
/// essentially a graph consisting of the data range or domain on one axis and frequency of
/// occurence on the other axis. All the data in the domain is counted in the appropriate bin. The
/// total number of elements belonging to each bin is known as the bin's frequency.
///
/// The regular histogram function creates bins of equal size between the minimum and maximum of
/// the input data (min and max are calculated internally). The histogram min-max function takes
/// input parameters minimum and maximum, and divides the bins into equal sizes within the range
/// specified by min and max parameters. All values less than min in the data range are placed in
/// the first (min) bin and all values greater than max will be placed in the last (max) bin.
///
/// # Parameters
///
/// - `input` is the Array whose histogram has to be computed
/// - `nbins` is the number bins the input data has to be categorized into.
/// - `minval` is the minimum value of bin ordering
/// - `maxval` is the maximum value of bin ordering
///
/// # Return Values
///
/// Histogram of input Array
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

/// Bilateral Filter.
///
/// A bilateral filter is a edge-preserving filter that reduces noise in an image. The intensity of
/// each pixel is replaced by a weighted average of the intensities of nearby pixels. The weights
/// follow a Gaussian distribution and depend on the distance as well as the color distance.
///
/// The bilateral filter requires the size of the filter (in pixels) and the upper bound on color
/// values, N, where pixel values range from 0–N inclusively.
///
/// # Parameters
///
/// - `input` array is the input image
/// - `spatial_sigma` is the spatial variance parameter that decides the filter window
/// - `chromatic_sigma` is the chromatic variance parameter
/// - `iscolor` indicates if the input is color image or grayscale
///
/// # Return Values
///
/// Filtered Image - Array
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

/// Meanshift Filter.
///
/// A meanshift filter is an edge-preserving smoothing filter commonly used in object tracking and
/// image segmentation.
///
/// This filter replaces each pixel in the image with the mean of the values within a given given
/// color and spatial radius. The meanshift filter is an iterative algorithm that continues until a
/// maxium number of iterations is met or until the value of the means no longer changes.
///
/// # Parameters
///
/// - `input` array is the input image
/// - `spatial_sigma` is the spatial variance parameter that decides the filter window
/// - `chromatic_sigma` is the chromatic variance parameter
/// - `iter` is the number of iterations filter operation is performed
/// - `iscolor` indicates if the input is color image or grayscale
///
/// # Return Values
///
/// Filtered Image - Array
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
        pub fn $fn_name(input: &Array, wlen: u64, wwid: u64,
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

/// Creates a Gaussian Kernel.
///
/// This function creates a kernel of a specified size that contains a Gaussian distribution. This
/// distribution is normalized to one. This is most commonly used when performing a Gaussian blur
/// on an image. The function takes two sets of arguments, the size of the kernel (width and height
/// in pixels) and the sigma parameters (for row and column) which effect the distribution of the
/// weights in the y and x directions, respectively.
///
/// Changing sigma causes the weights in each direction to vary. Sigma is calculated internally as
/// (0.25 * rows + 0.75) for rows and similarly for columns.
///
/// # Parameters
///
/// - `rows` is number of rows of kernel
/// - `cols` is number of cols of kernel
/// - `sigma_r` is standard deviation of rows
/// - `sigma_c` is standard deviation of cols
///
/// # Return Values
///
/// An Array with gaussian kernel values
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

/// Color space conversion
///
/// Following are the supported conversions
///
/// - RGB => GRAY
/// - GRAY => RGB
/// - RGB => HSV
/// - HSV => RGB
///
/// RGB (Red, Green, Blue) is the most common format used in computer imaging. RGB stores
/// individual values for red, green and blue, and hence the 3 values per pixel. A combination of
/// these three values produces the gamut of unique colors.
///
/// HSV (Hue, Saturation, Value), also known as HSB (hue, saturation, brightness), is often used by
/// artists because it is more natural to think about a color in terms of hue and saturation than
/// in terms of additive or subtractive color components (as in RGB). HSV is a transformation of
/// RGB colorspace; its components and colorimetry are relative to the RGB colorspace from which it
/// was derived. Like RGB, HSV also uses 3 values per pixel.
///
/// GRAY is a single channel color space where pixel value ranges from 0 to 1. Zero represents
/// black, one represent white and any value between zero & one is a gray value
///
/// # Parameters
///
/// - `input` is the input image
/// - `tospace` is the target color space. Takes values of [ColorSpace](./enum.ColorSpace.html)
/// - `fromspace` is the source image color space. Takes values of
/// [ColorSpace](./enum.ColorSpace.html)
///
/// # Return Values
///
/// An Array with input image values in target color space
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

/// Find blobs in given image.
///
/// Given a binary image (with zero representing background pixels), regions computes a floating
/// point image where each connected component is labeled from 1 to N, the total number of
/// components in the image.
///
/// A component is defined as one or more nonzero pixels that are connected by the specified
/// connectivity (either [`Connectivity::FOUR`](./enum.Connectivity.html) or [`Connectivity::EIGHT`](./enum.Connectivity.html)) in two dimensions.
///
/// # Parameters
///
/// - `input` is the input image
/// - `conn` can take one of the values of [Connectivity](./enum.Connectivity.html)
/// - `aftype` can take one of the values of [Aftype](./enum.Aftype.html)
///
/// # Return Values
///
/// Array with labels indicating different regions
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

/// Sobel Operator
///
/// Sobel operators perform a 2-D spatial gradient measurement on an image to emphasize the regions
/// of high spatial frequency, namely edges. A more in depth discussion on it can be found [here](https://en.wikipedia.org/wiki/Sobel_operator).
///
/// # Parameters
///
/// - `input` is the input image
/// - `ker_size` is the kernel size of sobel operator
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array has derivatives along horizontal direction
///
/// The second Array has derivatives along vertical direction
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

/// Histogram Equalization
///
/// # Parameters
///
/// - `input` is the input Array to be equalized
/// - `hist` is the Array to be used for equalizing input
///
/// # Return Values
/// Equalized Array
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
        /// Color space conversion functions
        ///
        /// # Parameters
        ///
        /// - `r` is fraction of red channel to appear in output
        /// - `g` is fraction of green channel to appear in output
        /// - `b` is fraction of blue channel to appear in output
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
        /// Color space conversion functions
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

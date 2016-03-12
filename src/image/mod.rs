extern crate libc;

use array::Array;
use defines::AfError;
use defines::BorderType;
use defines::ColorSpace;
use defines::Connectivity;
use defines::InterpType;
use defines::YCCStd;
use util::HasAfEnum;
use self::libc::{uint8_t, c_uint, c_int, c_float, c_double};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;

// unused functions from image.h header
// af_load_image_memory
// af_save_image_memory
// af_delete_image_memory

#[allow(dead_code)]
extern {
    fn af_gradient(dx: MutAfArray, dy: MutAfArray, arr: AfArray) -> c_int;
    fn af_load_image(out: MutAfArray, filename: *const u8, iscolor: c_int) -> c_int;
    fn af_save_image(filename: *const u8, input: AfArray) -> c_int;
    fn af_load_image_native(out: MutAfArray, filename: *const u8) -> c_int;
    fn af_save_image_native(filename: *const u8, input: AfArray) -> c_int;

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

    fn af_unwrap(out: MutAfArray, input: AfArray, wx: DimT, wy: DimT, sx: DimT, sy: DimT,
                 px: DimT, py: DimT, is_column: c_int) -> c_int;

    fn af_wrap(out: MutAfArray, input: AfArray,
               ox: DimT, oy: DimT,
               wx: DimT, wy: DimT,
               sx: DimT, sy: DimT,
               px: DimT, py: DimT,
               is_column: c_int) -> c_int;

    fn af_sat(out: MutAfArray, input: AfArray) -> c_int;

    fn af_ycbcr2rgb(out: MutAfArray, input: AfArray, stnd: c_int) -> c_int;
    fn af_rgb2ycbcr(out: MutAfArray, input: AfArray, stnd: c_int) -> c_int;
    fn af_is_image_io_available(out: *mut c_int) -> c_int;
    fn af_transform_coordinates(out: MutAfArray, tf: AfArray, d0: c_float, d1: c_float) -> c_int;
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

/// Load Image into Array in it's native type
///
/// This load image function allows you to load images as U8, U16 or F32
/// depending on the type of input image as shown by the table below.
///
///  Bits per Color (Gray/RGB/RGBA Bits Per Pixel) | Array Type  | Range
/// -----------------------------------------------|-------------|---------------
///   8 ( 8/24/32  BPP)                            | u8          | 0 - 255
///  16 (16/48/64  BPP)                            | u16         | 0 - 65535
///  32 (32/96/128 BPP)                            | f32         | 0 - 1
///
/// # Parameters
///
/// - `filename` is name of file to be loaded
///
/// # Return Arrays
///
/// An Array with pixel values loaded from the image
#[allow(unused_mut)]
pub fn load_image_native(filename: String) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_load_image_native(&mut temp as MutAfArray,
                                    filename.clone().as_bytes().as_ptr() as *const u8);
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

/// Save an Array without modifications to an image file
///
/// This function only accepts U8, U16, F32 arrays. These arrays are saved to images without any modifications. You must also note that note all image type support 16 or 32 bit images. The best options for 16 bit images are PNG, PPM and TIFF. The best option for 32 bit images is TIFF. These allow lossless storage.
///
/// The images stored have the following properties:
///
///  Array Type  | Bits per Color (Gray/RGB/RGBA Bits Per Pixel) | Range
/// -------------|-----------------------------------------------|---------------
///  U8          |  8 ( 8/24/32  BPP)                            | 0 - 255
///  U16         | 16 (16/48/64  BPP)                            | 0 - 65535
///  F32         | 32 (32/96/128 BPP)                            | 0 - 1
///
/// # Parameters
///
/// - `filename` is name of file to be saved
/// - `input` is the Array to be saved. Should be U8 for saving 8-bit image, U16 for 16-bit image, and F32 for 32-bit image.
#[allow(unused_mut)]
pub fn save_image_native(filename: String, input: &Array) -> Result<(), AfError> {
    unsafe {
        let err_val = af_save_image_native(filename.clone().as_bytes().as_ptr() as *const u8,
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

/// Translate an Image
///
/// Translating an image is moving it along 1st and 2nd dimensions by trans0 and trans1. Positive
/// values of these will move the data towards negative x and negative y whereas negative values of
/// these will move the positive right and positive down. See the example below for more.
///
/// To specify an output dimension, use the odim0 and odim1 for dim0 and dim1 respectively. The
/// size of 2rd and 3rd dimension is same as input. If odim0 and odim1 and not defined, then the
/// output dimensions are same as the input dimensions and the data out of bounds will be
/// discarded.
///
/// All new values that do not map to a location of the input array are set to 0.
///
/// Translate is a special case of the [transform](./fn.transform.html) function.
///
/// # Parameters
///
/// - `input` is input image
/// - `trans0` is amount by which the first dimension is translated
/// - `trans1` is amount by which the second dimension is translated
/// - `odim0` is the first output dimension
/// - `odim1` is the second output dimension
/// - `method` is the interpolation type (Nearest by default)
///
/// # Return Values
///
/// Translated Image(Array).
#[allow(unused_mut)]
pub fn translate(input: &Array, trans0: f32, trans1: f32,
                 odim0: i64, odim1: i64, method: InterpType) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_translate(&mut temp as MutAfArray,
                                   input.get() as AfArray,
                                   trans0 as c_float, trans1 as c_float,
                                   odim0 as DimT, odim1 as DimT,
                                   method as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Scale an Image
///
/// Scale is the same functionality as [resize](./fn.resize.html) except that the scale function uses the transform kernels. The other difference is that scale does not set boundary values to be the boundary of the input array. Instead these are set to 0.
///
/// Scale is a special case of the [transform](./fn.transform.html) function.
///
/// # Parameters
///
/// - `input` is input image
/// - `trans0` is amount by which the first dimension is translated
/// - `trans1` is amount by which the second dimension is translated
/// - `odim0` is the first output dimension
/// - `odim1` is the second output dimension
/// - `method` is the interpolation type (Nearest by default)
///
/// # Return Values
///
/// Translated Image(Array).
#[allow(unused_mut)]
pub fn scale(input: &Array, scale0: f32, scale1: f32,
             odim0: i64, odim1: i64, method: InterpType) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_scale(&mut temp as MutAfArray,
                               input.get() as AfArray,
                               scale0 as c_float, scale1 as c_float,
                               odim0 as DimT, odim1 as DimT,
                               method as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

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

/// Dilate an Image
///
/// The dilation function takes two pieces of data as inputs. The first is the input image to be
/// morphed, and the second is the mask indicating the neighborhood around each pixel to match.
///
/// In dilation, for each pixel, the mask is centered at the pixel. If the center pixel of the mask
/// matches the corresponding pixel on the image, then the mask is accepted. If the center pixels
/// do not matches, then the mask is ignored and no changes are made.
///
/// For further reference, see [here](https://en.wikipedia.org/wiki/Dilation_(morphology)).
///
/// # Parameters
///
/// - `input` is the input image
/// - `mask` is the morphological operation mask
///
/// # Return Values
///
/// Dilated Image(Array)
#[allow(unused_mut)]
pub fn dilate(input: &Array, mask: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_dilate(&mut temp as MutAfArray,
                                input.get() as AfArray, mask.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Erode an Image
///
/// The erosion function is a morphological transformation on an image that requires two inputs.
/// The first is the image to be morphed, and the second is the mask indicating neighborhood that
/// must be white in order to preserve each pixel.
///
/// In erode, for each pixel, the mask is centered at the pixel. If each pixel of the mask matches
/// the corresponding pixel on the image, then no change is made. If there is at least one
/// mismatch, then pixels are changed to the background color (black).
///
/// For further reference, see [here](https://en.wikipedia.org/wiki/Erosion_(morphology)).
///
/// # Parameters
///
/// - `input` is the input image
/// - `mask` is the morphological operation mask
///
/// # Return Values
///
/// Eroded Image(Array)
#[allow(unused_mut)]
pub fn erode(input: &Array, mask: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_erode(&mut temp as MutAfArray,
                               input.get() as AfArray, mask.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Dilate a Volume
///
/// Dilation for a volume is similar to the way dilation works on an image. Only difference is that
/// the masking operation is performed on a volume instead of a rectangular region.
///
/// # Parameters
///
/// - `input` is the input volume
/// - `mask` is the morphological operation mask
///
/// # Return Values
///
/// Dilated Volume(Array)
#[allow(unused_mut)]
pub fn dilate3(input: &Array, mask: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_dilate3(&mut temp as MutAfArray,
                                input.get() as AfArray, mask.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Erode a Volume
///
/// Erosion for a volume is similar to the way erosion works on an image. Only difference is that
/// the masking operation is performed on a volume instead of a rectangular region.
///
/// # Parameters
///
/// - `input` is the input volume
/// - `mask` is the morphological operation mask
///
/// # Return Values
///
/// Eroded Volume(Array)
#[allow(unused_mut)]
pub fn erode3(input: &Array, mask: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_erode3(&mut temp as MutAfArray,
                               input.get() as AfArray, mask.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

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
/// - YCbCr => RGB
/// - RGB => YCbCr
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
///
/// # Return Values
///
/// Array with labels indicating different regions
#[allow(unused_mut)]
pub fn regions<OutType: HasAfEnum>(input: &Array, conn: Connectivity) -> Result<Array, AfError> {
    unsafe {
        let otype = OutType::get_af_dtype();
        let mut temp: i64 = 0;
        let err_val = af_regions(&mut temp as MutAfArray, input.get() as AfArray,
                                 conn as uint8_t, otype as uint8_t);
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

/// Generate an array with image windows as columns
///
/// unwrap takes in an input image along with the window sizes wx and wy, strides sx and sy, and
/// padding px and py. This function then generates a matrix where each windows is an independent
/// column.
///
/// The number of columns (rows if is_column is true) in the output array are govenered by the
/// number of windows that can be fit along x and y directions. Padding is applied along all 4
/// sides of the matrix with px defining the height of the padding along dim 0 and py defining the
/// width of the padding along dim 1.
///
/// The first column window is always at the top left corner of the input including padding. If a
/// window cannot fit before the end of the matrix + padding, it is skipped from the generated
/// matrix.
///
/// Padding can take a maximum value of window - 1 repectively for x and y.
///
/// For multiple channels (3rd and 4th dimension), the generated matrix contains the same number of
/// channels as the input matrix. Each channel of the output matrix corresponds to the same channel
/// of the input.
///
/// # Parameters
///
/// - `input` is the input image
/// - `wx` is the block window size along 0th-dimension between [1, input.dims[0] + px]
/// - `wy` is the block window size along 1st-dimension between [1, input.dims[1] + py]
/// - `sx` is the stride along 0th-dimension
/// - `sy` is the stride along 1st-dimension
/// - `px` is the padding along 0th-dimension between [0, wx). Padding is applied both before and after.
/// - `py` is the padding along 1st-dimension between [0, wy). Padding is applied both before and after.
/// - `is_column` specifies the layout for the unwrapped patch. If is_column is false, the unrapped patch is laid out as a row.
///
/// # Return Values
///
/// An Array with image windows as columns
///
/// # Examples
///
/// ```
/// A [5 5 1 1]
/// 10 15 20 25 30
/// 11 16 21 26 31
/// 12 17 22 27 32
/// 13 18 23 28 33
/// 14 19 24 29 34
///
/// // Window 3x3, strides 1x1, padding 0x0
/// unwrap(A, 3, 3, 1, 1, 0, 0, False) [9 9 1 1]
/// 10 11 12 15 16 17 20 21 22
/// 11 12 13 16 17 18 21 22 23
/// 12 13 14 17 18 19 22 23 24
/// 15 16 17 20 21 22 25 26 27
/// 16 17 18 21 22 23 26 27 28
/// 17 18 19 22 23 24 27 28 29
/// 20 21 22 25 26 27 30 31 32
/// 21 22 23 26 27 28 31 32 33
/// 22 23 24 27 28 29 32 33 34
///
/// // Window 3x3, strides 1x1, padding 1x1
/// unwrap(A, 3, 3, 1, 1, 1, 1, False) [9 25 1 1]
///  0  0  0  0  0  0 10 11 12 13  0 15 16 17 18  0 20 21 22 23  0 25 26 27 28
///  0  0  0  0  0 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29
///  0  0  0  0  0 11 12 13 14  0 16 17 18 19  0 21 22 23 24  0 26 27 28 29  0
///  0 10 11 12 13  0 15 16 17 18  0 20 21 22 23  0 25 26 27 28  0 30 31 32 33
/// 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34
/// 11 12 13 14  0 16 17 18 19  0 21 22 23 24  0 26 27 28 29  0 31 32 33 34  0
///  0 15 16 17 18  0 20 21 22 23  0 25 26 27 28  0 30 31 32 33  0  0  0  0  0
/// 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34  0  0  0  0  0
/// 16 17 18 19  0 21 22 23 24  0 26 27 28 29  0 31 32 33 34  0  0  0  0  0  0
/// ```
pub fn unwrap(input: &Array,
              wx: i64, wy: i64,
              sx: i64, sy: i64,
              px: i64, py: i64,
              is_column: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_unwrap(&mut temp as MutAfArray, input.get() as AfArray,
                                wx, wy, sx, sy, px, py, is_column as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Converts unwrapped image to an image
///
/// Wrap takes an unwrapped image (see unwrap()) and converts it back to an image.
///
/// The inputs to this function should be the same as the inputs used to generate the unwrapped
/// image.
///
/// # Parameters
///
/// - `input` is the output of unwrap function call
/// - `ox` is the 0th-dimension of output image
/// - `oy` is the 1st-dimension of output image
/// - `wx` is the block window size along 0th-dimension between
/// - `wy` is the block window size along 1st-dimension between
/// - `sx` is the stride along 0th-dimension
/// - `sy` is the stride along 1st-dimension
/// - `px` is the padding used along 0th-dimension between [0, wx).
/// - `py` is the padding used along 1st-dimension between [0, wy).
/// - `is_column` specifies the layout for the unwrapped patch. If is_column is false, the rows are treated as the patches
///
/// # Return Values
///
/// Image(Array) created from unwrapped Image(Array)
pub fn wrap(input: &Array,
            ox: i64, oy: i64, wx: i64, wy: i64,
            sx: i64, sy: i64, px: i64, py: i64,
            is_column: bool) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_wrap(&mut temp as MutAfArray, input.get() as AfArray,
                              ox, oy, wx, wy, sx, sy, px, py, is_column as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Summed area table of an Image
///
/// # Parameters
///
/// - `input` is the input image
///
/// # Return Values
///
/// Summed area table (a.k.a Integral Image) of the input image.
pub fn sat(input: &Array) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_sat(&mut temp as MutAfArray, input.get() as AfArray);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// RGB to YCbCr colorspace converter.
///
/// RGB (Red, Green, Blue) is the most common format used in computer imaging. RGB stores
/// individual values for red, green and blue, and hence the 3 values per pixel. A combination of
/// these three values produces the gamut of unique colors.
///
/// YCbCr is a family of color spaces used as a part of the color image pipeline in video and
/// digital photography systems where Y is luma component and Cb & Cr are the blue-difference and
/// red-difference chroma components.
///
/// Input array to this function should be of real data in the range [0,1].
///
/// # Parameters
///
/// - `input` is the input image in RGB color space
/// - `standard` is the target color space - [YCbCr standard](./enum.YCCStd.html)
///
/// # Return Values
///
/// Image(Array) in YCbCr color space
pub fn rgb2ycbcr(input: &Array, standard: YCCStd) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_rgb2ycbcr(&mut temp as MutAfArray, input.get() as AfArray,
                                   standard as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// YCbCr to RGB colorspace converter.
///
/// YCbCr is a family of color spaces used as a part of the color image pipeline in video and
/// digital photography systems where Y is luma component and Cb & Cr are the blue-difference and
/// red-difference chroma components.
///
/// RGB (Red, Green, Blue) is the most common format used in computer imaging. RGB stores
/// individual values for red, green and blue, and hence the 3 values per pixel. A combination of
/// these three values produces the gamut of unique colors.
///
/// Input array to this function should be of real data with the following range in their
/// respective channels.
///
/// - Y  −> [16,219]
/// - Cb −> [16,240]
/// - Cr −> [16,240]
///
/// # Parameters
///
/// - `input` is the input image in YCbCr color space
/// - `standard` is the [YCbCr standard](./enum.YCCStd.html) in which input image color space is
/// present.
///
/// # Return Values
///
/// Image(Array) in RGB color space
pub fn ycbcr2rgb(input: &Array, standard: YCCStd) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_ycbcr2rgb(&mut temp as MutAfArray, input.get() as AfArray,
                                   standard as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

/// Function to check if Image I/O is available
///
/// # Parameters
///
/// None
///
/// # Return Values
///
/// Return a boolean indicating if ArrayFire was compiled with Image I/O support
pub fn is_imageio_available() -> bool {
    unsafe {
        let mut temp: i32 = 0;
        af_is_image_io_available(&mut temp as *mut c_int);
        temp > 0 // Return boolean fla
    }
}

/// Transform input coordinates
///
/// The transform function uses a perspective transform matrix to transform input coordinates
/// (given as two dimensions) into a coordinates matrix.
///
/// The output is a 4x2 matrix, indicating the coordinates of the 4 bidimensional transformed
/// points.
///
/// # Parameters
///
/// - `tf` is the transformation matrix
/// - `d0` is the first input dimension
/// - `d1` is the second input dimension
///
/// # Return Values
///
/// Transformed coordinates
pub fn transform_coords(tf: &Array, d0: f32, d1: f32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_transform_coordinates(&mut temp as MutAfArray,
                                               tf.get() as AfArray,
                                               d0 as c_float, d1 as c_float);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
    }
}

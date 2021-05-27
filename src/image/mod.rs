use super::core::{
    af_array, dim_t, AfError, Array, BorderType, CannyThresholdType, ColorSpace, ConfidenceCCInput,
    Connectivity, DeconvInput, DiffusionEq, EdgeComputable, FloatingPoint, FluxFn,
    GrayRGBConvertible, HasAfEnum, ImageFilterType, ImageNativeType, InterpType, InverseDeconvAlgo,
    IterativeDeconvAlgo, MomentType, MomentsComputable, RealFloating, RealNumber, YCCStd,
    HANDLE_ERROR,
};

use libc::{c_char, c_double, c_float, c_int, c_uint};
use std::ffi::CString;

// unused functions from image.h header
// TODO add later when requested
// af_load_image_memory
// af_save_image_memory
// af_delete_image_memory

extern "C" {
    fn af_cast(out: *mut af_array, arr: af_array, aftype: c_uint) -> c_int;
    fn af_gradient(dx: *mut af_array, dy: *mut af_array, arr: af_array) -> c_int;
    fn af_load_image(out: *mut af_array, filename: *const c_char, iscolor: bool) -> c_int;
    fn af_save_image(filename: *const c_char, input: af_array) -> c_int;
    fn af_load_image_native(out: *mut af_array, filename: *const c_char) -> c_int;
    fn af_save_image_native(filename: *const c_char, input: af_array) -> c_int;

    fn af_resize(
        out: *mut af_array,
        input: af_array,
        odim0: dim_t,
        odim1: dim_t,
        method: c_uint,
    ) -> c_int;

    fn af_transform(
        out: *mut af_array,
        input: af_array,
        trans: af_array,
        odim0: dim_t,
        odim1: dim_t,
        method: c_uint,
        is_inverse: bool,
    ) -> c_int;

    fn af_rotate(
        out: *mut af_array,
        input: af_array,
        theta: c_float,
        crop: bool,
        method: c_uint,
    ) -> c_int;

    fn af_translate(
        out: *mut af_array,
        input: af_array,
        trans0: c_float,
        trans1: c_float,
        odim0: dim_t,
        odim1: dim_t,
        method: c_uint,
    ) -> c_int;

    fn af_scale(
        out: *mut af_array,
        input: af_array,
        scale0: c_float,
        scale1: c_float,
        odim0: dim_t,
        odim1: dim_t,
        method: c_uint,
    ) -> c_int;

    fn af_skew(
        out: *mut af_array,
        input: af_array,
        skew0: c_float,
        skew1: c_float,
        odim0: dim_t,
        odim1: dim_t,
        method: c_uint,
        is_inverse: bool,
    ) -> c_int;

    fn af_histogram(
        out: *mut af_array,
        input: af_array,
        nbins: c_uint,
        minval: c_double,
        maxval: c_double,
    ) -> c_int;

    fn af_dilate(out: *mut af_array, input: af_array, mask: af_array) -> c_int;
    fn af_dilate3(out: *mut af_array, input: af_array, mask: af_array) -> c_int;
    fn af_erode(out: *mut af_array, input: af_array, mask: af_array) -> c_int;
    fn af_erode3(out: *mut af_array, input: af_array, mask: af_array) -> c_int;
    fn af_regions(out: *mut af_array, input: af_array, conn: c_uint, aftype: c_uint) -> c_int;
    fn af_sobel_operator(dx: *mut af_array, dy: *mut af_array, i: af_array, ksize: c_uint)
        -> c_int;
    fn af_rgb2gray(
        out: *mut af_array,
        input: af_array,
        r: c_float,
        g: c_float,
        b: c_float,
    ) -> c_int;
    fn af_gray2rgb(
        out: *mut af_array,
        input: af_array,
        r: c_float,
        g: c_float,
        b: c_float,
    ) -> c_int;
    fn af_hist_equal(out: *mut af_array, input: af_array, hist: af_array) -> c_int;
    fn af_hsv2rgb(out: *mut af_array, input: af_array) -> c_int;
    fn af_rgb2hsv(out: *mut af_array, input: af_array) -> c_int;

    fn af_bilateral(
        out: *mut af_array,
        input: af_array,
        sp_sig: c_float,
        ch_sig: c_float,
        iscolor: bool,
    ) -> c_int;

    fn af_mean_shift(
        out: *mut af_array,
        input: af_array,
        sp_sig: c_float,
        ch_sig: c_float,
        iter: c_uint,
        iscolor: bool,
    ) -> c_int;

    fn af_medfilt(
        out: *mut af_array,
        input: af_array,
        wlen: dim_t,
        wwid: dim_t,
        etype: c_uint,
    ) -> c_int;

    fn af_medfilt1(out: *mut af_array, input: af_array, wlen: dim_t, etype: c_uint) -> c_int;

    fn af_minfilt(
        out: *mut af_array,
        input: af_array,
        wlen: dim_t,
        wwid: dim_t,
        etype: c_uint,
    ) -> c_int;

    fn af_maxfilt(
        out: *mut af_array,
        input: af_array,
        wlen: dim_t,
        wwid: dim_t,
        etype: c_uint,
    ) -> c_int;

    fn af_gaussian_kernel(
        out: *mut af_array,
        rows: c_int,
        cols: c_int,
        sigma_r: c_double,
        sigma_c: c_double,
    ) -> c_int;

    fn af_color_space(
        out: *mut af_array,
        input: af_array,
        tospace: c_uint,
        fromspace: c_uint,
    ) -> c_int;

    fn af_unwrap(
        out: *mut af_array,
        input: af_array,
        wx: dim_t,
        wy: dim_t,
        sx: dim_t,
        sy: dim_t,
        px: dim_t,
        py: dim_t,
        is_column: bool,
    ) -> c_int;

    fn af_wrap(
        out: *mut af_array,
        input: af_array,
        ox: dim_t,
        oy: dim_t,
        wx: dim_t,
        wy: dim_t,
        sx: dim_t,
        sy: dim_t,
        px: dim_t,
        py: dim_t,
        is_column: bool,
    ) -> c_int;

    fn af_sat(out: *mut af_array, input: af_array) -> c_int;

    fn af_ycbcr2rgb(out: *mut af_array, input: af_array, stnd: c_uint) -> c_int;
    fn af_rgb2ycbcr(out: *mut af_array, input: af_array, stnd: c_uint) -> c_int;
    fn af_is_image_io_available(out: *mut bool) -> c_int;
    fn af_transform_coordinates(
        out: *mut af_array,
        tf: af_array,
        d0: c_float,
        d1: c_float,
    ) -> c_int;

    fn af_moments(out: *mut af_array, input: af_array, moment: c_uint) -> c_int;
    fn af_moments_all(out: *mut c_double, input: af_array, moment: c_uint) -> c_int;

    fn af_canny(
        out: *mut af_array,
        input: af_array,
        thres_type: c_int,
        low: c_float,
        high: c_float,
        swindow: c_uint,
        is_fast: bool,
    ) -> c_int;
    fn af_anisotropic_diffusion(
        out: *mut af_array,
        input: af_array,
        dt: c_float,
        K: c_float,
        iters: c_uint,
        fftype: c_uint,
        diff_kind: c_uint,
    ) -> c_int;
    fn af_confidence_cc(
        out: *mut af_array,
        input: af_array,
        seedx: af_array,
        seedy: af_array,
        radius: c_uint,
        multiplier: c_uint,
        iterations: c_int,
        seg_val: c_double,
    ) -> c_int;
    fn af_iterative_deconv(
        out: *mut af_array,
        input: af_array,
        ker: af_array,
        iterations: c_uint,
        rfactor: c_float,
        algo: c_uint,
    ) -> c_int;
    fn af_inverse_deconv(
        out: *mut af_array,
        input: af_array,
        ker: af_array,
        gamma: c_float,
        algo: c_uint,
    ) -> c_int;
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
pub fn gradient<T>(input: &Array<T>) -> (Array<T>, Array<T>)
where
    T: HasAfEnum + FloatingPoint,
{
    unsafe {
        let mut dx: af_array = std::ptr::null_mut();
        let mut dy: af_array = std::ptr::null_mut();
        let err_val = af_gradient(
            &mut dx as *mut af_array,
            &mut dy as *mut af_array,
            input.get(),
        );
        HANDLE_ERROR(AfError::from(err_val));
        (dx.into(), dy.into())
    }
}

/// Load Image into Array
///
/// Only, Images with 8/16/32 bits per channel can be loaded using this function.
///
/// # Parameters
///
/// - `filename` is aboslute path of the image to be loaded.
/// - `is_color` indicates if the image file at given path is color or gray scale.
///
/// # Return Arrays
///
/// An Array with pixel values loaded from the image
#[allow(clippy::match_wild_err_arm)]
pub fn load_image<T>(filename: String, is_color: bool) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    let cstr_param = match CString::new(filename) {
        Ok(cstr) => cstr,
        Err(_) => panic!("CString creation from input filename failed"),
    };
    let trgt_type = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err1 = af_load_image(&mut temp as *mut af_array, cstr_param.as_ptr(), is_color);
        HANDLE_ERROR(AfError::from(err1));

        let mut img: af_array = std::ptr::null_mut();
        let err2 = af_cast(&mut img as *mut af_array, temp, trgt_type as c_uint);
        HANDLE_ERROR(AfError::from(err2));

        img.into()
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
#[allow(clippy::match_wild_err_arm)]
pub fn load_image_native<T>(filename: String) -> Array<T>
where
    T: HasAfEnum + ImageNativeType,
{
    let cstr_param = match CString::new(filename) {
        Ok(cstr) => cstr,
        Err(_) => panic!("CString creation from input filename failed"),
    };
    let trgt_type = T::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err1 = af_load_image_native(&mut temp as *mut af_array, cstr_param.as_ptr());
        HANDLE_ERROR(AfError::from(err1));

        let mut img: af_array = std::ptr::null_mut();
        let err2 = af_cast(&mut img as *mut af_array, temp, trgt_type as c_uint);
        HANDLE_ERROR(AfError::from(err2));

        img.into()
    }
}

/// Save an Array to an image file
///
/// # Parameters
///
/// - `filename` is the abolute path(includes filename) at which input Array is going to be saved
/// - `input` is the Array to be stored into the image file
#[allow(clippy::match_wild_err_arm)]
pub fn save_image<T>(filename: String, input: &Array<T>)
where
    T: HasAfEnum + RealNumber,
{
    let cstr_param = match CString::new(filename) {
        Ok(cstr) => cstr,
        Err(_) => panic!("CString creation from input filename failed"),
    };
    unsafe {
        let err_val = af_save_image(cstr_param.as_ptr(), input.get());
        HANDLE_ERROR(AfError::from(err_val));
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
#[allow(clippy::match_wild_err_arm)]
pub fn save_image_native<T>(filename: String, input: &Array<T>)
where
    T: HasAfEnum + ImageNativeType,
{
    let cstr_param = match CString::new(filename) {
        Ok(cstr) => cstr,
        Err(_) => panic!("CString creation from input filename failed"),
    };
    unsafe {
        let err_val = af_save_image_native(cstr_param.as_ptr(), input.get());
        HANDLE_ERROR(AfError::from(err_val));
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
pub fn resize<T: HasAfEnum>(
    input: &Array<T>,
    odim0: i64,
    odim1: i64,
    method: InterpType,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_resize(
            &mut temp as *mut af_array,
            input.get(),
            odim0 as dim_t,
            odim1 as dim_t,
            method as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
/// Transformed Array
pub fn transform<T: HasAfEnum>(
    input: &Array<T>,
    trans: &Array<f32>,
    odim0: i64,
    odim1: i64,
    method: InterpType,
    is_inverse: bool,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_transform(
            &mut temp as *mut af_array,
            input.get(),
            trans.get(),
            odim0 as dim_t,
            odim1 as dim_t,
            method as c_uint,
            is_inverse,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn rotate<T: HasAfEnum>(
    input: &Array<T>,
    theta: f64,
    crop: bool,
    method: InterpType,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_rotate(
            &mut temp as *mut af_array,
            input.get(),
            theta as c_float,
            crop,
            method as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn translate<T: HasAfEnum>(
    input: &Array<T>,
    trans0: f32,
    trans1: f32,
    odim0: i64,
    odim1: i64,
    method: InterpType,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_translate(
            &mut temp as *mut af_array,
            input.get(),
            trans0,
            trans1,
            odim0 as dim_t,
            odim1 as dim_t,
            method as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn scale<T: HasAfEnum>(
    input: &Array<T>,
    scale0: f32,
    scale1: f32,
    odim0: i64,
    odim1: i64,
    method: InterpType,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_scale(
            &mut temp as *mut af_array,
            input.get(),
            scale0,
            scale1,
            odim0 as dim_t,
            odim1 as dim_t,
            method as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn skew<T: HasAfEnum>(
    input: &Array<T>,
    skew0: f32,
    skew1: f32,
    odim0: i64,
    odim1: i64,
    method: InterpType,
    is_inverse: bool,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_skew(
            &mut temp as *mut af_array,
            input.get(),
            skew0,
            skew1,
            odim0 as dim_t,
            odim1 as dim_t,
            method as c_uint,
            is_inverse,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn histogram<T>(input: &Array<T>, nbins: u32, minval: f64, maxval: f64) -> Array<u32>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_histogram(
            &mut temp as *mut af_array,
            input.get(),
            nbins,
            minval,
            maxval,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn dilate<T>(input: &Array<T>, mask: &Array<T>) -> Array<T>
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_dilate(&mut temp as *mut af_array, input.get(), mask.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn erode<T>(input: &Array<T>, mask: &Array<T>) -> Array<T>
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_erode(&mut temp as *mut af_array, input.get(), mask.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn dilate3<T>(input: &Array<T>, mask: &Array<T>) -> Array<T>
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_dilate3(&mut temp as *mut af_array, input.get(), mask.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn erode3<T>(input: &Array<T>, mask: &Array<T>) -> Array<T>
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_erode3(&mut temp as *mut af_array, input.get(), mask.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn bilateral<T>(
    input: &Array<T>,
    spatial_sigma: f32,
    chromatic_sigma: f32,
    iscolor: bool,
) -> Array<T::AbsOutType>
where
    T: HasAfEnum + ImageFilterType,
    T::AbsOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_bilateral(
            &mut temp as *mut af_array,
            input.get(),
            spatial_sigma,
            chromatic_sigma,
            iscolor,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn mean_shift<T>(
    input: &Array<T>,
    spatial_sigma: f32,
    chromatic_sigma: f32,
    iter: u32,
    iscolor: bool,
) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_mean_shift(
            &mut temp as *mut af_array,
            input.get(),
            spatial_sigma,
            chromatic_sigma,
            iter,
            iscolor,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! filt_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `input` is the input image(Array)
        /// - `wlen` is the horizontal length of the filter
        /// - `hlen` is the vertical length of the filter
        /// - `etype` is enum of type [BorderType](./enum.BorderType.html)
        ///
        ///# Return Values
        ///
        /// An Array with filtered image data.
        pub fn $fn_name<T>(input: &Array<T>, wlen: u64, wwid: u64, etype: BorderType) -> Array<T>
        where
            T: HasAfEnum + ImageFilterType,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(
                    &mut temp as *mut af_array,
                    input.get(),
                    wlen as dim_t,
                    wwid as dim_t,
                    etype as c_uint,
                );
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

filt_func_def!("Median filter", medfilt, af_medfilt);
filt_func_def!(
    "Box filter with minimum as box operation",
    minfilt,
    af_minfilt
);
filt_func_def!(
    "Box filter with maximum as box operation",
    maxfilt,
    af_maxfilt
);

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
pub fn gaussian_kernel(rows: i32, cols: i32, sigma_r: f64, sigma_c: f64) -> Array<f32> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_gaussian_kernel(&mut temp as *mut af_array, rows, cols, sigma_r, sigma_c);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn color_space<T>(input: &Array<T>, tospace: ColorSpace, fromspace: ColorSpace) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_color_space(
            &mut temp as *mut af_array,
            input.get(),
            tospace as c_uint,
            fromspace as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn regions<OutType>(input: &Array<bool>, conn: Connectivity) -> Array<OutType>
where
    OutType: HasAfEnum + RealNumber,
{
    let otype = OutType::get_af_dtype();
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_regions(
            &mut temp as *mut af_array,
            input.get(),
            conn as c_uint,
            otype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn sobel<T>(input: &Array<T>, ker_size: u32) -> (Array<T::SobelOutType>, Array<T::SobelOutType>)
where
    T: HasAfEnum + ImageFilterType,
    T::SobelOutType: HasAfEnum,
{
    unsafe {
        let mut dx: af_array = std::ptr::null_mut();
        let mut dy: af_array = std::ptr::null_mut();
        let err_val = af_sobel_operator(
            &mut dx as *mut af_array,
            &mut dy as *mut af_array,
            input.get(),
            ker_size,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (dx.into(), dy.into())
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
pub fn hist_equal<T>(input: &Array<T>, hist: &Array<u32>) -> Array<T>
where
    T: HasAfEnum + RealNumber,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_hist_equal(&mut temp as *mut af_array, input.get(), hist.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

macro_rules! grayrgb_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        ///
        ///# Parameters
        ///
        /// - `r` is fraction of red channel to appear in output
        /// - `g` is fraction of green channel to appear in output
        /// - `b` is fraction of blue channel to appear in output
        ///
        ///#Return Values
        ///
        ///An Array with image data in target color space
        pub fn $fn_name<T>(input: &Array<T>, r: f32, g: f32, b: f32) -> Array<T>
        where
            T: HasAfEnum + GrayRGBConvertible,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, input.get(), r, g, b);
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

grayrgb_func_def!("Color(RGB) to Grayscale conversion", rgb2gray, af_rgb2gray);
grayrgb_func_def!("Grayscale to Color(RGB) conversion", gray2rgb, af_gray2rgb);

macro_rules! hsvrgb_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        pub fn $fn_name<T>(input: &Array<T>) -> Array<T>
        where
            T: HasAfEnum + RealFloating,
        {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, input.get());
                HANDLE_ERROR(AfError::from(err_val));
                temp.into()
            }
        }
    };
}

hsvrgb_func_def!("HSV to RGB color space conversion", hsv2rgb, af_hsv2rgb);
hsvrgb_func_def!("RGB to HSV color space conversion", rgb2hsv, af_rgb2hsv);

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
/// - `wx` is the block window size along 0th-dimension between \[1, input.dims\[0\] + px\]
/// - `wy` is the block window size along 1st-dimension between \[1, input.dims\[1\] + py\]
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
/// ```text
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
#[allow(clippy::too_many_arguments)]
pub fn unwrap<T: HasAfEnum>(
    input: &Array<T>,
    wx: i64,
    wy: i64,
    sx: i64,
    sy: i64,
    px: i64,
    py: i64,
    is_column: bool,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_unwrap(
            &mut temp as *mut af_array,
            input.get(),
            wx,
            wy,
            sx,
            sy,
            px,
            py,
            is_column,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
#[allow(clippy::too_many_arguments)]
pub fn wrap<T: HasAfEnum>(
    input: &Array<T>,
    ox: i64,
    oy: i64,
    wx: i64,
    wy: i64,
    sx: i64,
    sy: i64,
    px: i64,
    py: i64,
    is_column: bool,
) -> Array<T> {
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_wrap(
            &mut temp as *mut af_array,
            input.get(),
            ox,
            oy,
            wx,
            wy,
            sx,
            sy,
            px,
            py,
            is_column,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
pub fn sat<T>(input: &Array<T>) -> Array<T::AggregateOutType>
where
    T: HasAfEnum + RealNumber,
    T::AggregateOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_sat(&mut temp as *mut af_array, input.get());
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
/// Input array to this function should be of real data in the range \[0,1\].
///
/// # Parameters
///
/// - `input` is the input image in RGB color space
/// - `standard` is the target color space - [YCbCr standard](./enum.YCCStd.html)
///
/// # Return Values
///
/// Image(Array) in YCbCr color space
pub fn rgb2ycbcr<T>(input: &Array<T>, standard: YCCStd) -> Array<T>
where
    T: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_rgb2ycbcr(&mut temp as *mut af_array, input.get(), standard as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
/// - Y  −> \[16,219\]
/// - Cb −> \[16,240\]
/// - Cr −> \[16,240\]
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
pub fn ycbcr2rgb<T>(input: &Array<T>, standard: YCCStd) -> Array<T>
where
    T: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_ycbcr2rgb(&mut temp as *mut af_array, input.get(), standard as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
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
    let mut temp: bool = false;
    unsafe {
        af_is_image_io_available(&mut temp as *mut bool);
    }
    temp
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
pub fn transform_coords<T>(tf: &Array<T>, d0: f32, d1: f32) -> Array<T>
where
    T: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_transform_coordinates(&mut temp as *mut af_array, tf.get(), d0, d1);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Find Image moments
///
/// # Parameters
///
/// - `input` is the input image
/// - `moment` is the type of moment to be computed, takes a value of
/// [enum](./enum.MomentType.html)
///
/// # Return Values
///
/// Moments Array
pub fn moments<T>(input: &Array<T>, moment: MomentType) -> Array<f32>
where
    T: HasAfEnum + MomentsComputable,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_moments(&mut temp as *mut af_array, input.get(), moment as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Find Image moment for whole image
///
/// # Parameters
///
/// - `input` is the input image
/// - `moment` is the type of moment to be computed, takes a value of
/// [enum](./enum.MomentType.html)
///
/// # Return Values
///
/// Moment value of the whole image
pub fn moments_all<T>(input: &Array<T>, moment: MomentType) -> f64
where
    T: HasAfEnum + MomentsComputable,
{
    let mut temp: f64 = 0.0;
    unsafe {
        let err_val = af_moments_all(&mut temp as *mut c_double, input.get(), moment as c_uint);
        HANDLE_ERROR(AfError::from(err_val));
    }
    temp
}

/// One dimensional median filter on image
///
/// # Parameters
///
///  - `input` is the input image(Array)
///  - `wlen` is the horizontal length of the filter
///  - `etype` is enum of type [BorderType](./enum.BorderType.html)
///
/// # Return Values
///
/// An Array with filtered image data.
pub fn medfilt1<T>(input: &Array<T>, wlen: u64, etype: BorderType) -> Array<T>
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_medfilt1(
            &mut temp as *mut af_array,
            input.get(),
            wlen as dim_t,
            etype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Canny edge detection operator
///
/// The Canny edge detector is an edge detection operator that uses a multi-stage algorithm to detect a wide range of edges in images. A more in depth discussion on it can be found [here](https://en.wikipedia.org/wiki/Canny_edge_detector).
///
/// # Parameters
///
/// - `input` is the input image
/// - `threshold_type` helps determine if user set high threshold is to be used or not. It can take values defined by the enum [CannyThresholdType](./enum.CannyThresholdType.html)
/// - `low` is the lower threshold % of the maximum or auto-derived high
/// - `high` is the higher threshold % of maximum value in gradient image used in hysteresis procedure. This value is ignored if [CannyThresholdType::OTSU](./enum.CannyThresholdType.html) is chosen.
/// - `sobel_window` is the window size of sobel kernel for computing gradient direction and magnitude.
/// - `is_fast` indicates if L<SUB>1</SUB> norm(faster but less accurate) is used to compute image gradient magnitude instead of L<SUB>2</SUB> norm.
///
/// # Return Values
///
/// An Array of binary type [DType::B8](./enum.DType.html) indicating edges(All pixels with
/// non-zero values are edges).
pub fn canny<T>(
    input: &Array<T>,
    threshold_type: CannyThresholdType,
    low: f32,
    high: f32,
    sobel_window: u32,
    is_fast: bool,
) -> Array<bool>
where
    T: HasAfEnum + EdgeComputable,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_canny(
            &mut temp as *mut af_array,
            input.get(),
            threshold_type as c_int,
            low,
            high,
            sobel_window as c_uint,
            is_fast,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Anisotropic smoothing filter
///
/// Anisotropic diffusion algorithm aims at removing noise in the images
/// while preserving important features such as edges. The algorithm
/// essentially creates a scale space representation of the original
/// image, where image from previous step is used to create a new version
/// of blurred image using the diffusion process. Standard isotropic diffusion
/// methods such as gaussian blur, doesn't take into account the local
/// content(smaller neighborhood of current processing pixel) while removing
/// noise. Anisotropic diffusion uses the flux equations given below to
/// achieve that. Flux equation is the formula used by the diffusion process
/// to determine how much a pixel in neighborhood should contribute to
/// the blurring operation being done at the current pixel at a given iteration.
///
/// The flux function can be either exponential or quadratic.
///
/// <table>
/// <caption id="multi row">Available Flux Functions</caption>
/// <tr>
///     <td align="center" style="vertical-align:middle;">
///       AF_FLUX_QUADRATIC
///     </td>
///     <td align="center">
///       \begin{equation}
///         \frac{1}{1 + (\frac{\| \nabla I\|}{K})^2}
///       \end{equation}
///     </td>
/// </tr>
/// <tr>
///     <td align="center" style="vertical-align:middle;">
///       AF_FLUX_EXPONENTIAL
///     </td>
///     <td align="center">
///       \begin{equation}
///         \exp{-(\frac{\| \nabla I\|}{K})^2}
///       \end{equation}
///     </td>
/// </tr>
/// </table>
///
/// Please be cautious using the time step parameter to the function.
/// Appropriate time steps for solving this type of p.d.e. depend on
/// the dimensionality of the image and the order of the equation.
/// Stable values for most 2D and 3D functions are 0.125 and 0.0625,
/// respectively. The time step values are automatically constrained
/// to the stable value.
///
/// Another input parameter to be cautious about is the conductance
/// parameter, lower values strongly preserve image features and
/// vice-versa. For human vision, this value ranges from 0.5 to 2.0.
///
/// # Parameters
///
/// - `img` is the noisy input image
/// - `dt` is the timestep for diffusion equation
/// - `k` is the conductance parameter for diffusion
/// - `iters` is the number of iterations diffusion is performed
/// - `fftype` dictates the type of flux flow and it is an
///    [enum](./enum.DiffusionEq.html)
/// - `diff_kind` dictates the type of diffusion and it is an
///   [enum](./enum.FluxFn.html)
///
/// # Return Values
///
/// Returns an anisotropically smoothed and noise-free image
///
/// ### References
///
///  - Pietro Perona and Jitendra Malik, `Scale-space and edge detection
///    using anisotropic diffusion,` IEEE Transactions on Pattern Analysis
///    Machine Intelligence, vol. 12, pp. 629-639, 1990.
///  - R. Whitaker and X. Xue. `Variable-Conductance, Level-Set Curvature
///    for Image Denoising`, International Conference on Image Processing,
///    2001 pp. 142-145, Vol.3.
pub fn anisotropic_diffusion<T>(
    img: &Array<T>,
    dt: f32,
    k: f32,
    iters: u32,
    fftype: FluxFn,
    diff_kind: DiffusionEq,
) -> Array<T::AbsOutType>
where
    T: HasAfEnum + EdgeComputable,
    T::AbsOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_anisotropic_diffusion(
            &mut temp as *mut af_array,
            img.get(),
            dt,
            k,
            iters,
            fftype as c_uint,
            diff_kind as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Segment image based on similar pixel characteristics
///
/// This filter is similar to [regions](./fn.regions.html) with additional criteria for
/// segmentation. In regions, all connected pixels are considered to be a single component.
/// In this variation of connected components, pixels having similar pixel statistics of the
/// neighborhoods around a given set of seed points are grouped together.
///
/// The parameter `radius` determines the size of neighborhood around a seed point.
///
/// Mean and Variance are the pixel statistics that are computed across all neighborhoods around
/// the given set of seed points. The pixels which are connected to seed points and lie in the
/// confidence interval are grouped together. Given below is the confidence interval.
///
/// \begin{equation}
///     [\mu - \alpha * \sigma, \mu + \alpha * \sigma]
/// \end{equation}
/// where
///
/// - $ \mu $ is the mean of the pixels in the seed neighborhood
/// - $ \sigma^2 $ is the variance of the pixels in the seed neighborhood
/// - $ \alpha $ is the multiplier used to control the width of the confidence interval.
///
/// This filter follows an iterative approach for fine tuning the segmentation. An initial
/// segmenetation followed by a finite number `iterations` of segmentations are performed.
/// The user provided parameter `iterations` is only a request and the algorithm can prempt
/// the execution if variance approaches zero. The initial segmentation uses the mean and
/// variance calculated from the neighborhoods of all the seed points. For subsequent
/// segmentations, all pixels in the previous segmentation are used to re-calculate the mean
/// and variance (as opposed to using the pixels in the neighborhood of the seed point).
///
/// # Parameters
///
/// - `input` is the input image
/// - `seedx` contains the x coordinates of seeds in image coordinates
/// - `seedy` contains the y coordinates of seeds in image coordinates
/// - `radius` is the neighborhood region to be considered around each seed point
/// - `multiplier` controls the threshold range computed from the mean and variance of seed point neighborhoods
/// - `iterations` is the number of times the segmentation in performed
/// - `segmented_value` is the value to which output array valid pixels are set to
///
/// # Return Values
///
/// Segmented(based on pixel characteristics) image(Array) with regions surrounding the seed points
pub fn confidence_cc<InOutType>(
    input: &Array<InOutType>,
    seedx: &Array<u32>,
    seedy: &Array<u32>,
    radius: u32,
    multiplier: u32,
    iterations: u32,
    segmented_val: f64,
) -> Array<InOutType>
where
    InOutType: ConfidenceCCInput,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_confidence_cc(
            &mut temp as *mut af_array,
            input.get(),
            seedx.get(),
            seedy.get(),
            radius,
            multiplier,
            iterations as i32,
            segmented_val,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Iterative Deconvolution
///
/// The following table shows the iteration update equations of the respective
/// deconvolution algorithms.
///
/// <table>
/// <tr><th>Algorithm</th><th>Update Equation</th></tr>
/// <tr>
///     <td>LandWeber</td>
///     <td>
///         $ \hat{I}_{n} = \hat{I}_{n-1} + \alpha * P^T \otimes (I - P \otimes \hat{I}_{n-1}) $
///     </td>
/// </tr>
/// <tr>
///   <td>Richardson-Lucy</td>
///   <td>
///     $ \hat{I}_{n} = \hat{I}_{n-1} . ( \frac{I}{\hat{I}_{n-1} \otimes P} \otimes P^T ) $
///   </td>
/// </tr>
/// </table>
///
/// where
///
/// - $ I $ is the observed(input/blurred) image
/// - $ P $ is the point spread function
/// - $ P^T $ is the transpose of point spread function
/// - $ \hat{I}_{n} $ is the current iteration's updated image estimate
/// - $ \hat{I}_{n-1} $ is the previous iteration's image estimate
/// - $ \alpha $ is the relaxation factor
/// - $ \otimes $ indicates the convolution operator
///
/// The type of output Array from deconvolution will be of type f64 if
/// the input array type is f64. For other types, output type will be f32 type.
/// Should the caller want to save the image to disk or require the values of output
/// to be in a fixed range, that should be done by the caller explicitly.
pub fn iterative_deconv<T>(
    input: &Array<T>,
    kernel: &Array<f32>,
    iterations: u32,
    relaxation_factor: f32,
    algo: IterativeDeconvAlgo,
) -> Array<T::AbsOutType>
where
    T: DeconvInput,
    T::AbsOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_iterative_deconv(
            &mut temp as *mut af_array,
            input.get(),
            kernel.get(),
            iterations,
            relaxation_factor,
            algo as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Inverse deconvolution
///
/// This is a linear algorithm i.e. they are non-iterative in
/// nature and usually faster than iterative deconvolution algorithms.
///
/// Depending on the values passed on to `algo` of type enum [InverseDeconvAlgo](./enum.inverse_deconv_algo.html),
/// different equations are used to compute the final result.
///
/// #### Tikhonov's Deconvolution Method:
///
/// The update equation for this algorithm is as follows:
///
/// <div>
/// \begin{equation}
/// \hat{I}_{\omega} = \frac{ I_{\omega} * P^{*}_{\omega} } { |P_{\omega}|^2 + \gamma }
/// \end{equation}
/// </div>
///
/// where
///
/// - $ I_{\omega} $ is the observed(input/blurred) image in frequency domain
/// - $ P_{\omega} $ is the point spread function in frequency domain
/// - $ \gamma $ is a user defined regularization constant
///
/// The type of output Array from deconvolution will be double if the input array type is double.
/// Otherwise, it will be float in rest of the cases. Should the caller want to save the image to
/// disk or require the values of output to be in a fixed range, that should be done by the caller
/// explicitly.
pub fn inverse_deconv<T>(
    input: &Array<T>,
    kernel: &Array<f32>,
    gamma: f32,
    algo: InverseDeconvAlgo,
) -> Array<T::AbsOutType>
where
    T: DeconvInput,
    T::AbsOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_inverse_deconv(
            &mut temp as *mut af_array,
            input.get(),
            kernel.get(),
            gamma,
            algo as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

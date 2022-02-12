use super::core::{
    af_array, af_features, dim_t, AfError, Array, HasAfEnum, HomographyType, ImageFilterType,
    MatchType, RealFloating, HANDLE_ERROR,
};

use libc::{c_float, c_int, c_uint};
use std::mem;

// af_sift and af_gloh uses patented algorithms, so didn't add them
// they are NOT built using installer builds

extern "C" {
    fn af_create_features(feat: *mut af_features, num: dim_t) -> c_int;
    fn af_retain_features(feat: *mut af_features, feat: af_features) -> c_int;
    fn af_get_features_num(num: *mut dim_t, feat: af_features) -> c_int;
    fn af_get_features_xpos(out: *mut af_array, feat: af_features) -> c_int;
    fn af_get_features_ypos(out: *mut af_array, feat: af_features) -> c_int;
    fn af_get_features_score(out: *mut af_array, feat: af_features) -> c_int;
    fn af_get_features_orientation(out: *mut af_array, feat: af_features) -> c_int;
    fn af_get_features_size(out: *mut af_array, feat: af_features) -> c_int;
    fn af_release_features(feat: af_features) -> c_int;

    fn af_fast(
        out: *mut af_features,
        input: af_array,
        thr: c_float,
        arc_len: c_uint,
        non_max: bool,
        feature_ratio: c_float,
        edge: c_uint,
    ) -> c_int;

    fn af_harris(
        out: *mut af_features,
        input: af_array,
        m: c_uint,
        r: c_float,
        s: c_float,
        bs: c_uint,
        k: c_float,
    ) -> c_int;

    fn af_orb(
        out: *mut af_features,
        desc: *mut af_array,
        arr: af_array,
        fast_thr: c_float,
        max_feat: c_uint,
        scl_fctr: c_float,
        levels: c_uint,
        blur_img: bool,
    ) -> c_int;

    fn af_hamming_matcher(
        idx: *mut af_array,
        dist: *mut af_array,
        query: af_array,
        train: af_array,
        dist_dim: dim_t,
        n_dist: c_uint,
    ) -> c_int;

    fn af_nearest_neighbour(
        idx: *mut af_array,
        dist: *mut af_array,
        q: af_array,
        t: af_array,
        dist_dim: dim_t,
        n_dist: c_uint,
        dist_type: c_int,
    ) -> c_int;

    fn af_match_template(
        out: *mut af_array,
        search_img: af_array,
        template_img: af_array,
        mtype: c_uint,
    ) -> c_int;

    fn af_susan(
        feat: *mut af_features,
        i: af_array,
        r: c_uint,
        d: c_float,
        g: c_float,
        f: c_float,
        e: c_uint,
    ) -> c_int;

    fn af_dog(out: *mut af_array, i: af_array, r1: c_int, r2: c_int) -> c_int;

    fn af_homography(
        H: *mut af_array,
        inliers: *mut c_int,
        x_src: af_array,
        y_src: af_array,
        x_dst: af_array,
        y_dst: af_array,
        htype: c_uint,
        inlier_thr: c_float,
        iterations: c_uint,
        otype: c_uint,
    ) -> c_int;
}

/// A set of Array objects (usually, used in Computer vision context)
///
/// `Features` struct is used by computer vision functions
/// to return the outcome of their operation. Typically, such output
/// has the following Arrays:
///
/// - X positions of the features
/// - Y positions of the features
/// - Scores of the features
/// - Orientations of the features
/// - Sizes of the features
///
/// ## Sharing Across Threads
///
/// While sharing this object with other threads, there is no need to wrap
/// this in an Arc object unless only one such object is required to exist.
/// The reason being that ArrayFire's internal details that are pointed to
/// by the features handle are appropriately reference counted in thread safe
/// manner. However, if these features are to be edited, then please do wrap
/// the object using a Mutex or Read-Write lock.
pub struct Features {
    feat: af_features,
}

unsafe impl Send for Features {}
unsafe impl Sync for Features {}

macro_rules! feat_func_def {
    ($doc_str: expr, $fn_name: ident, $ffi_name: ident) => {
        #[doc=$doc_str]
        pub fn $fn_name(&self) -> Array<f32> {
            unsafe {
                let mut temp: af_array = std::ptr::null_mut();
                let err_val = $ffi_name(&mut temp as *mut af_array, self.feat);
                HANDLE_ERROR(AfError::from(err_val));

                let temp_array: Array<f32> = temp.into();
                let retained = temp_array.clone();
                mem::forget(temp_array);

                retained
            }
        }
    };
}

impl Features {
    /// Create and return an object of type Features
    ///
    /// This object is basically a bunch of Arrays.
    pub fn new(n: u64) -> Self {
        unsafe {
            let mut temp: af_features = std::ptr::null_mut();
            let err_val = af_create_features(&mut temp as *mut af_features, n as dim_t);
            HANDLE_ERROR(AfError::from(err_val));
            Self { feat: temp }
        }
    }

    /// Get total number of features found
    pub fn num_features(&self) -> i64 {
        let mut temp: i64 = 0;
        unsafe {
            let err_val = af_get_features_num(
                &mut temp as *mut dim_t,
                self.feat as *const dim_t as af_features,
            );
            HANDLE_ERROR(AfError::from(err_val));
        }
        temp
    }

    feat_func_def!("Get x coordinates Array", xpos, af_get_features_xpos);
    feat_func_def!("Get y coordinates Array", ypos, af_get_features_ypos);
    feat_func_def!("Get score Array", score, af_get_features_score);
    feat_func_def!(
        "Get orientation Array",
        orientation,
        af_get_features_orientation
    );
    feat_func_def!("Get features size Array", size, af_get_features_size);
}

impl Clone for Features {
    fn clone(&self) -> Self {
        unsafe {
            let mut temp: af_features = std::ptr::null_mut();
            let ret_val = af_retain_features(&mut temp as *mut af_features, self.feat);
            HANDLE_ERROR(AfError::from(ret_val));
            Self { feat: temp }
        }
    }
}

impl Drop for Features {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_features(self.feat);
            HANDLE_ERROR(AfError::from(ret_val));
        }
    }
}

/// Fast feature detector
///
/// A circle of radius 3 pixels, translating into a total of 16 pixels, is checked for sequential
/// segments of pixels much brighter or much darker than the central one. For a pixel p to be
/// considered a feature, there must exist a sequential segment of arc_length pixels in the circle
/// around it such that all are greather than (p + thr) or smaller than (p - thr). After all
/// features in the image are detected, if nonmax is true, the non-maximal suppression is applied,
/// checking all detected features and the features detected in its 8-neighborhood and discard it
/// if its score is non maximal.
///
/// # Parameters
///
/// - `input` - the input image Array
/// - `thr` - FAST threshold for which pixel of the circle around the center pixel is considered to
/// be greater or smaller
/// - `arc_len` - length of arc (or sequential segment) to be tested, must be within range [9-16]
/// - `non_max` - performs non-maximal supression if true
/// - `feat_ratio` - maximum ratio of features to detect, the maximum number of features is
/// calculated by `feature_ratio * num of elements`. The maximum number of features is not based on
/// the score, instead, features detected after the limit is reached are discarded.
/// - `edge` - is the length of the edges in the image to be discarded by FAST(minimum is 3, as the
/// radius of the circle)
///
/// # Return Values
///
/// This function returns an object of struct [Features](./struct.Features.html) containing Arrays
/// for x and y coordinates and score, while array oreientation is set to 0 as FAST does not
/// compute orientation. Size is set to 1 as FAST does not compute multiple scales.
pub fn fast<T>(
    input: &Array<T>,
    thr: f32,
    arc_len: u32,
    non_max: bool,
    feat_ratio: f32,
    edge: u32,
) -> Features
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_features = std::ptr::null_mut();
        let err_val = af_fast(
            &mut temp as *mut af_features,
            input.get(),
            thr,
            arc_len,
            non_max,
            feat_ratio,
            edge,
        );
        HANDLE_ERROR(AfError::from(err_val));
        Features { feat: temp }
    }
}

/// Harris corner detector.
///
/// Compute corners using the Harris corner detector approach. For each pixel, a small window is
/// used to calculate the determinant and trace of such a window, from which a response is
/// calculated. Pixels are considered corners if they are local maximas and have a high positive
/// response.
///
/// # Parameters
///
/// - `input` is the array containing a grayscale image (color images are not supported)
/// - `max_corners` is the maximum number of corners to keep, only retains those with highest Harris responses
/// - `min_response` is the minimum response in order for a corner to be retained, only used if max_corners = 0
/// - `sigma` is the standard deviation of a circular window (its dimensions will be calculated according to the standard deviation), the covariation matrix will be calculated to a circular neighborhood of this standard deviation (only used when block_size == 0, must be >= 0.5f and <= 5.0f)
/// - `block_size` is square window size, the covariation matrix will be calculated to a square neighborhood of this size (must be >= 3 and <= 31)
/// - `k_thr` is the Harris constant, usually set empirically to 0.04f (must be >= 0.01f)
///
/// # Return Values
///
/// This function returns an object of struct [Features](./struct.Features.html) containing Arrays
/// for x and y coordinates and score, while array oreientation & size are set to 0 & 1,
/// respectively, since harris doesn't compute that information
pub fn harris<T>(
    input: &Array<T>,
    max_corners: u32,
    min_response: f32,
    sigma: f32,
    block_size: u32,
    k_thr: f32,
) -> Features
where
    T: HasAfEnum + RealFloating,
{
    unsafe {
        let mut temp: af_features = std::ptr::null_mut();
        let err_val = af_harris(
            &mut temp as *mut af_features,
            input.get(),
            max_corners,
            min_response,
            sigma,
            block_size,
            k_thr,
        );
        HANDLE_ERROR(AfError::from(err_val));
        Features { feat: temp }
    }
}

/// ORB feature descriptor
///
/// Extract ORB descriptors from FAST features that hold higher Harris responses. FAST does not
/// compute orientation, thus, orientation of features is calculated using the intensity centroid.
/// As FAST is also not multi-scale enabled, a multi-scale pyramid is calculated by downsampling
/// the input image multiple times followed by FAST feature detection on each scale.
///
/// # Parameters
///
/// - `input` - the input image Array
/// - `fast_thr` - FAST threshold for which a pixel of the circle around the central pixel is
/// considered to be brighter or darker
/// - `max_feat` - maximum number of features to hold
/// - `scl_fctr` - factor to downsample the input image, meaning that each level with hold prior
/// level dimensions divided by `scl_fctr`
/// - `levels` - number of levels to be computed for the image pyramid
/// - `blur_img` - blur image with a Gaussian filter with sigma=2 before computing descriptors to
/// increase robustness against noise if true
///
/// # Return Values
///
/// This function returns a tuple of [`Features`](./struct.Features.html) and [`Array`](./struct.Array.html). The features objects composed of Arrays for x and y coordinates, score, orientation and size of selected features. The Array object is a two dimensional Array of size Nx8 where N is number of selected features.
pub fn orb<T>(
    input: &Array<T>,
    fast_thr: f32,
    max_feat: u32,
    scl_fctr: f32,
    levels: u32,
    blur_img: bool,
) -> (Features, Array<T>)
where
    T: HasAfEnum + RealFloating,
{
    unsafe {
        let mut f: af_features = std::ptr::null_mut();
        let mut d: af_array = std::ptr::null_mut();
        let err_val = af_orb(
            &mut f as *mut af_features,
            &mut d as *mut af_array,
            input.get(),
            fast_thr,
            max_feat,
            scl_fctr,
            levels,
            blur_img,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (Features { feat: f }, d.into())
    }
}

/// Hamming feature matcher
///
/// Calculates Hamming distances between two 2-dimensional arrays containing features, one of the
/// arrays containing the training data and the other the query data. One of the dimensions of the
/// both arrays must be equal among them, identifying the length of each feature. The other
/// dimension indicates the total number of features in each of the training and query arrays. Two
/// 1-dimensional arrays are created as results, one containg the smallest N distances of the query
/// array and another containing the indices of these distances in the training array. The
/// resulting 1-dimensional arrays have length equal to the number of features contained in the
/// query array.
///
/// # Parameters
///
/// - `query` - Array containing the data to be queried
/// - `train` - Array containing the data to be used as training data
/// - `dist_dims` - indicates the dimension to analyze for distance (the dimension indicated here
/// must be of equal length for both query and train arrays)
/// - `n_dist` - is the number of smallest distances to return (currently, only values <= 256 are supported)
///
///
/// # Return Values
///
/// This function returns a tuple of [Array](./struct.Array.html)'s.
///
/// First Array is an array of MxN size, where M is equal to the number of query features and N is
/// equal to n_dist. The value at position IxJ indicates the index of the Jth smallest distance to
/// the Ith query value in the train data array. the index of the Ith smallest distance of the Mth
/// query.
///
/// Second Array is an array of MxN size, where M is equal to the number of query features and N is
/// equal to n_dist. The value at position IxJ indicates the Hamming distance of the Jth smallest
/// distance to the Ith query value in the train data array.
pub fn hamming_matcher<T>(
    query: &Array<T>,
    train: &Array<T>,
    dist_dims: i64,
    n_dist: u32,
) -> (Array<u32>, Array<T::AggregateOutType>)
where
    T: HasAfEnum + ImageFilterType,
    T::AggregateOutType: HasAfEnum,
{
    unsafe {
        let mut idx: af_array = std::ptr::null_mut();
        let mut dist: af_array = std::ptr::null_mut();
        let err_val = af_hamming_matcher(
            &mut idx as *mut af_array,
            &mut dist as *mut af_array,
            query.get(),
            train.get(),
            dist_dims,
            n_dist,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (idx.into(), dist.into())
    }
}

/// Nearest Neighbour.
///
/// Calculates nearest distances between two 2-dimensional arrays containing features based on the
/// type of distance computation chosen. Currently, AF_SAD (sum of absolute differences), AF_SSD
/// (sum of squared differences) and AF_SHD (hamming distance) are supported. One of the arrays
/// containing the training data and the other the query data. One of the dimensions of the both
/// arrays must be equal among them, identifying the length of each feature. The other dimension
/// indicates the total number of features in each of the training and query arrays. Two
/// 1-dimensional arrays are created as results, one containg the smallest N distances of the query
/// array and another containing the indices of these distances in the training array. The resulting
/// 1-dimensional arrays have length equal to the number of features contained in the query array.
///
/// # Parameters
///
/// - `query` is the array containing the data to be queried
/// - `train` is the array containing the data used as training data
/// - `dist_dim` indicates the dimension to analyze for distance (the dimension indicated here must be of equal length for both query and train arrays)
/// - `n_dist` is the number of smallest distances to return (currently, only values <= 256 are supported)
/// - `dist_type` is the distance computation type. Currently [`MatchType::SAD`](./enum.MatchType.html), [`MatchType::SSD`](./enum.MatchType.html), and [`MatchType::SHD`](./enum.MatchType.html) are supported.
///
/// # Return Values
///
/// A tuple of Arrays.
///
/// The first Array is is an array of MxN size, where M is equal to the number of query features
/// and N is equal to `n_dist`. The value at position IxJ indicates the index of the Jth smallest
/// distance to the Ith query value in the train data array. the index of the Ith smallest distance
/// of the Mth query.
///
/// The second Array is is an array of MxN size, where M is equal to the number of query features
/// and N is equal to `n_dist`. The value at position IxJ indicates the distance of the Jth smallest
/// distance to the Ith query value in the train data array based on the `dist_type` chosen.
pub fn nearest_neighbour<T>(
    query: &Array<T>,
    train: &Array<T>,
    dist_dim: i64,
    n_dist: u32,
    dist_type: MatchType,
) -> (Array<u32>, Array<T::AggregateOutType>)
where
    T: HasAfEnum + ImageFilterType,
    T::AggregateOutType: HasAfEnum,
{
    unsafe {
        let mut idx: af_array = std::ptr::null_mut();
        let mut dist: af_array = std::ptr::null_mut();
        let err_val = af_nearest_neighbour(
            &mut idx as *mut af_array,
            &mut dist as *mut af_array,
            query.get(),
            train.get(),
            dist_dim,
            n_dist,
            dist_type as c_int,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (idx.into(), dist.into())
    }
}

/// Image matching
///
/// Template matching is an image processing technique to find small patches of an image which
/// match a given template image. A more in depth discussion on the topic can be found
/// [here](https://en.wikipedia.org/wiki/Template_matching).
///
/// # Parameters
///
/// - `search_img` is an array with image data
/// - `template_img` is the template we are looking for in the image
/// - `mtype` is metric that should be used to calculate the disparity between window in the image and the template image. It can be one of the values defined by the enum [MatchType](./enum.MatchType.html).
/// # Return Values
///
/// This function returns an Array with disparity values for the window starting at corresponding pixel position.
pub fn match_template<T>(
    search_img: &Array<T>,
    template_img: &Array<T>,
    mtype: MatchType,
) -> Array<T::AbsOutType>
where
    T: HasAfEnum + ImageFilterType,
    T::AbsOutType: HasAfEnum,
{
    match mtype {
        MatchType::NCC | MatchType::ZNCC | MatchType::SHD => HANDLE_ERROR(AfError::ERR_ARG),
        _ => (), // Do nothing valid matching type
    };
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_match_template(
            &mut temp as *mut af_array,
            search_img.get(),
            template_img.get(),
            mtype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// SUSAN corner detector.
///
/// SUSAN is an acronym standing for Smallest Univalue Segment Assimilating Nucleus. This method
/// places a circular disc over the pixel to be tested (a.k.a nucleus) to compute the corner
/// measure of that corresponding pixel. The region covered by the circular disc is M, and a pixel
/// in this region is represented by m⃗ ∈M where m⃗ 0 is the nucleus. Every pixel in the region is
/// compared to the nucleus using the following comparison function:
///
/// c(m⃗ )=e^−((I(m⃗)−I(m⃗_0))/t)^6
///
/// where t is radius of the region, I is the brightness of the pixel.
///
/// Response of SUSAN operator is given by the following equation:
///
/// R(M) = g−n(M) if n(M) < g
///
/// R(M) = 0 otherwise,
///
/// where n(M)=∑c(m⃗) m⃗∈M, g is named the geometric threshold and n is the number of pixels in the
/// mask which are within t of the nucleus.
///
/// Importance of the parameters, t and g is explained below:
///
/// - t determines how similar points have to be to the nucleusbefore they are considered to be a
/// part of the univalue segment
/// - g determines the minimum size of the univalue segment. For a large enough g, SUSAN operator
/// becomes an edge dectector.
///
/// # Parameters
///
/// - `input` is input grayscale/intensity image
/// - `radius` is the nucleus radius for each pixel neighborhood
/// - `diff_thr` is intensity difference threshold a.k.a **t** from equations in description
/// - `geom_thr` is the geometric threshold
/// - `feature_ratio` is maximum number of features that will be returned by the function
/// - `edge` indicates how many pixels width area should be skipped for corner detection
///
/// # Return Values
/// An object of type [Features](./struct.Features.html) composed of arrays for x and y coordinates, score, orientation and size of selected features.
pub fn susan<T>(
    input: &Array<T>,
    radius: u32,
    diff_thr: f32,
    geom_thr: f32,
    feature_ratio: f32,
    edge: u32,
) -> Features
where
    T: HasAfEnum + ImageFilterType,
{
    unsafe {
        let mut temp: af_features = std::ptr::null_mut();
        let err_val = af_susan(
            &mut temp as *mut af_features,
            input.get(),
            radius,
            diff_thr,
            geom_thr,
            feature_ratio,
            edge,
        );
        HANDLE_ERROR(AfError::from(err_val));
        Features { feat: temp }
    }
}

/// Difference of Gaussians.
///
/// Given an image, this function computes two different versions of smoothed input image using the
/// difference smoothing parameters and subtracts one from the other and returns the result.
///
/// # Parameters
///
/// - `input` is the input image
/// - `radius1` is the radius of the first gaussian kernel
/// - `radius2` is the radius of the second gaussian kernel
///
/// # Return Values
///
/// Difference of smoothed inputs - An Array.
pub fn dog<T>(input: &Array<T>, radius1: i32, radius2: i32) -> Array<T::AbsOutType>
where
    T: HasAfEnum + ImageFilterType,
    T::AbsOutType: HasAfEnum,
{
    unsafe {
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_dog(&mut temp as *mut af_array, input.get(), radius1, radius2);
        HANDLE_ERROR(AfError::from(err_val));
        temp.into()
    }
}

/// Homography estimation
///
/// Homography estimation find a perspective transform between two sets of 2D points.
/// Currently, two methods are supported for the estimation, RANSAC (RANdom SAmple Consensus)
/// and LMedS (Least Median of Squares). Both methods work by randomly selecting a subset
/// of 4 points of the set of source points, computing the eigenvectors of that set and
/// finding the perspective transform. The process is repeated several times, a maximum of
/// times given by the value passed to the iterations arguments for RANSAC (for the CPU
/// backend, usually less than that, depending on the quality of the dataset, but for CUDA
/// and OpenCL backends the transformation will be computed exactly the amount of times
/// passed via the iterations parameter), the returned value is the one that matches the
/// best number of inliers, which are all of the points that fall within a maximum L2
/// distance from the value passed to the inlier_thr argument.
///
/// # Parameters
///
/// - `x_src` is the x coordinates of the source points.
/// - `y_src` is the y coordinates of the source points.
/// - `x_dst` is the x coordinates of the destination points.
/// - `y_dst` is the y coordinates of the destination points.
/// - `htype` can be AF_HOMOGRAPHY_RANSAC, for which a RANdom SAmple Consensus will be used to evaluate the homography quality (e.g., number of inliers), or AF_HOMOGRAPHY_LMEDS, which will use Least Median of Squares method to evaluate homography quality
/// - `inlier_thr` - if htype is AF_HOMOGRAPHY_RANSAC, this parameter will five the maximum L2-distance for a point to be considered an inlier.
/// - `iterations` is the maximum number of iterations when htype is AF_HOMOGRAPHY_RANSAC and backend is CPU,if backend is CUDA or OpenCL, iterations is the total number of iterations, an iteration is a selection of 4 random points for which the homography is estimated and evaluated for number of inliers.
/// - `otype` is the array type for the homography output.
///
/// # Return Values
///
/// Returns a tuple of Array and int.
///
/// - `H` is a 3x3 array containing the estimated homography.
/// - `inliers` is the number of inliers that the homography was estimated to comprise, in the case that htype is AF_HOMOGRAPHY_RANSAC, a higher inlier_thr value will increase the estimated inliers. Note that if the number of inliers is too low, it is likely that a bad homography will be returned.
pub fn homography<OutType>(
    x_src: &Array<f32>,
    y_src: &Array<f32>,
    x_dst: &Array<f32>,
    y_dst: &Array<f32>,
    htype: HomographyType,
    inlier_thr: f32,
    iterations: u32,
) -> (Array<OutType>, i32)
where
    OutType: HasAfEnum + RealFloating,
{
    let otype = OutType::get_af_dtype();
    unsafe {
        let mut inliers: i32 = 0;
        let mut temp: af_array = std::ptr::null_mut();
        let err_val = af_homography(
            &mut temp as *mut af_array,
            &mut inliers as *mut c_int,
            x_src.get(),
            y_src.get(),
            x_dst.get(),
            y_dst.get(),
            htype as c_uint,
            inlier_thr,
            iterations,
            otype as c_uint,
        );
        HANDLE_ERROR(AfError::from(err_val));
        (temp.into(), inliers)
    }
}

#[cfg(test)]
mod tests {
    use crate::randu;

    #[test]
    #[should_panic]
    fn check_invalid_matchtype() {
        crate::core::set_device(0);
        let a = randu!(f32; 10, 10);
        let b = randu!(f32; 2, 2);
        super::match_template(&a, &b, crate::MatchType::NCC);
        super::match_template(&a, &b, crate::MatchType::ZNCC);
        super::match_template(&a, &b, crate::MatchType::SHD);
    }
}

extern crate libc;

use array::Array;
use defines::{AfError, HomographyType, MatchType};
use util::HasAfEnum;
use self::libc::{c_void, uint8_t, c_uint, c_int, c_float, c_double, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;
type MutFeat    = *mut *mut self::libc::c_void;
type Feat       = *const self::libc::c_void;

// af_sift and af_gloh uses patented algorithms, so didn't add them
// they are built using installer builds

#[allow(dead_code)]
extern {
    fn af_create_features(feat: MutFeat, num: DimT) -> c_int;
    fn af_retain_features(feat: MutFeat, feat: Feat) -> c_int;
    fn af_get_features_num(num: *mut DimT, feat: Feat) -> c_int;
    fn af_get_features_xpos(out: MutAfArray, feat: Feat) -> c_int;
    fn af_get_features_ypos(out: MutAfArray, feat: Feat) -> c_int;
    fn af_get_features_score(out: MutAfArray, feat: Feat) -> c_int;
    fn af_get_features_orientation(out: MutAfArray, feat: Feat) -> c_int;
    fn af_get_features_size(out: MutAfArray, feat: Feat) -> c_int;
    fn af_release_features(feat: *mut c_void) -> c_int;

    fn af_fast(out: MutFeat, input: AfArray, thr: c_float, arc_len: c_uint, non_max: c_int,
               feature_ratio: c_float, edge: c_uint) -> c_int;

    fn af_harris(out: MutFeat, input: AfArray, m: c_uint, r: c_float, s: c_float, bs: c_uint, k: c_float) -> c_int;

    fn af_orb(out: MutFeat, desc: MutAfArray, arr: AfArray, fast_thr: c_float, max_feat: c_uint,
              scl_fctr: c_float, levels: c_uint, blur_img: c_int) -> c_int;

    fn af_hamming_matcher(idx: MutAfArray, dist: MutAfArray,
                          query: AfArray, train: AfArray,
                          dist_dim: DimT, n_dist: c_uint) -> c_int;

    fn af_nearest_neighbour(idx: MutAfArray, dist: MutAfArray, q: AfArray, t: AfArray,
                            dist_dim: DimT, n_dist: c_uint, dist_type: c_int) -> c_int;

    fn af_match_template(out: MutAfArray, search_img: AfArray, template_img: AfArray,
                         mtype: uint8_t) -> c_int;

    fn af_susan(feat: MutFeat, i: AfArray, r: c_uint, d: c_float, g: c_float, f: c_float, e: c_uint) -> c_int;

    fn af_dog(out: MutAfArray, i: AfArray, r1: c_int, r2: c_int) -> c_int;

    fn af_homography(H: MutAfArray, inliers: *mut c_int, x_src: AfArray, y_src: AfArray,
                     x_dst: AfArray, y_dst: AfArray, htype: c_int, inlier_thr: c_float,
                     iterations: c_uint, otype: c_int) -> c_int;

    fn af_gloh(out: MutFeat, desc: MutAfArray, input: AfArray, n_layers: c_uint,
               contrast_thr: c_float, edge_thr: c_float, init_sigma: c_float,
               double_input: c_double, intensity_scale: c_float, feature_ratio: c_float) -> c_int;
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
pub struct Features {
    feat: i64,
}

macro_rules! feat_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        pub fn $fn_name(&self) -> Result<Array, AfError> {
            unsafe {
                let mut temp: i64 = 0;
                let err_val = $ffi_name(&mut temp as MutAfArray, self.feat as Feat);
                match err_val {
                    0 => Ok(Array::from(temp)),
                    _ => Err(AfError::from(err_val)),
                }
            }
        }
    )
}

impl Features {
    #[allow(unused_mut)]
    pub fn new(n: u64) -> Result<Features, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_create_features(&mut temp as *mut c_longlong as MutFeat,
                                             n as DimT);
            match err_val {
                0 => Ok(Features {feat: temp}),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    /// Get total number of features found
    pub fn num_features(&self) -> Result<i64, AfError> {
        unsafe {
            let mut temp: i64 = 0;
            let err_val = af_get_features_num(&mut temp as *mut DimT,
                                              self.feat as *const c_longlong as Feat);
            match err_val {
                0 => Ok(temp),
                _ => Err(AfError::from(err_val)),
            }
        }
    }

    feat_func_def!(xpos, af_get_features_xpos);
    feat_func_def!(ypos, af_get_features_ypos);
    feat_func_def!(score, af_get_features_score);
    feat_func_def!(orientation, af_get_features_orientation);
    feat_func_def!(size, af_get_features_size);

    pub fn get(&self) -> i64 {
        self.feat
    }
}

impl Clone for Features {
    fn clone(&self) -> Features {
        unsafe {
            let mut temp: i64 = 0;
            let ret_val = af_retain_features(&mut temp as *mut c_longlong as MutFeat,
                                             self.feat as *const c_longlong as Feat);
            match ret_val {
                0 => Features {feat: temp},
                _ => panic!("Weak copy of Features failed with error code: {}", ret_val),
            }
        }
    }
}

impl Drop for Features {
    fn drop(&mut self) {
        unsafe {
            let ret_val = af_release_features(self.feat as *mut c_longlong as *mut c_void);
            match ret_val {
                0 => (),
                _ => panic!("Weak copy of Features failed with error code: {}", ret_val),
            }
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
#[allow(unused_mut)]
pub fn fast(input: &Array, thr: f32, arc_len: u32,
            non_max: bool, feat_ratio: f32, edge: u32) -> Result<Features, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_fast(&mut temp as *mut c_longlong as MutFeat,
                input.get() as AfArray, thr as c_float, arc_len as c_uint,
                non_max as c_int, feat_ratio as c_float, edge as c_uint);
        match err_val {
            0 => Ok(Features {feat: temp}),
            _ => Err(AfError::from(err_val)),
        }
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
#[allow(unused_mut)]
pub fn harris(input: &Array, max_corners: u32, min_response: f32, sigma: f32, block_size: u32, k_thr: f32) -> Result<Features, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_harris(&mut temp as *mut c_longlong as MutFeat,
                                input.get() as AfArray, max_corners as c_uint,
                                min_response as c_float, sigma as c_float, block_size as c_uint,
                                k_thr as c_float);
        match err_val {
            0 => Ok(Features {feat: temp}),
            _ => Err(AfError::from(err_val)),
        }
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
#[allow(unused_mut)]
pub fn orb(input: &Array, fast_thr: f32, max_feat: u32,
           scl_fctr: f32, levels: u32, blur_img: bool) -> Result<(Features, Array), AfError> {
    unsafe {
        let mut f: i64 = 0;
        let mut d: i64 = 0;
        let err_val = af_orb(&mut f as *mut c_longlong as MutFeat, &mut d as MutAfArray,
               input.get() as AfArray, fast_thr as c_float,
               max_feat as c_uint, scl_fctr as c_float, levels as c_uint, blur_img as c_int);
        match err_val {
            0 => Ok((Features {feat: f}, Array::from(d))),
            _ => Err(AfError::from(err_val)),
        }
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
/// - `n_dist` - is the number of smallest distances to return (currently, only 1 is supported)
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
#[allow(unused_mut)]
pub fn hamming_matcher(query: &Array, train: &Array,
                       dist_dims: i64, n_dist: u32) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut idx: i64 = 0;
        let mut dist:i64 = 0;
        let err_val = af_hamming_matcher(&mut idx as MutAfArray, &mut dist as MutAfArray,
                           query.get() as AfArray, train.get() as AfArray,
                           dist_dims as DimT, n_dist as c_uint);
        match err_val {
            0 => Ok((Array::from(idx), Array::from(dist))),
            _ => Err(AfError::from(err_val)),
        }
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
/// - `n_dist` is the number of smallest distances to return (currently, only 1 is supported)
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
#[allow(unused_mut)]
pub fn nearest_neighbour(query: &Array, train: &Array, dist_dim: i64, n_dist: u32, dist_type: MatchType) -> Result<(Array, Array), AfError> {
    unsafe {
        let mut idx: i64 = 0;
        let mut dist: i64 = 0;
        let err_val = af_nearest_neighbour(&mut idx as MutAfArray, &mut dist as MutAfArray,
                                           query.get() as AfArray, train.get() as AfArray,
                                           dist_dim as DimT, n_dist as c_uint, dist_type as c_int);
        match err_val {
            0 => Ok((Array::from(idx), Array::from(dist))),
            _ => Err(AfError::from(err_val)),
        }
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
#[allow(unused_mut)]
pub fn match_template(search_img: &Array, template_img: &Array,
                      mtype: MatchType) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_match_template(&mut temp as MutAfArray,
                          search_img.get() as AfArray, template_img.get() as AfArray,
                          mtype as uint8_t);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
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
#[allow(unused_mut)]
pub fn susan(input: &Array, radius: u32, diff_thr: f32, geom_thr: f32, feature_ratio: f32, edge: u32) -> Result<Features, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_susan(&mut temp as *mut c_longlong as MutFeat,
                               input.get() as AfArray, radius as c_uint,
                               diff_thr as c_float, geom_thr as c_float, feature_ratio as c_float,
                               edge as c_uint);
        match err_val {
            0 => Ok(Features {feat: temp}),
            _ => Err(AfError::from(err_val)),
        }
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
#[allow(unused_mut)]
pub fn dog(input: &Array, radius1: i32, radius2: i32) -> Result<Array, AfError> {
    unsafe {
        let mut temp: i64 = 0;
        let err_val = af_dog(&mut temp as MutAfArray, input.get() as AfArray,
                             radius1 as c_int, radius2 as c_int);
        match err_val {
            0 => Ok(Array::from(temp)),
            _ => Err(AfError::from(err_val)),
        }
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
pub fn homography<OutType: HasAfEnum>(x_src: &Array, y_src: &Array,
                                      x_dst: &Array, y_dst: &Array,
                                      htype: HomographyType, inlier_thr: f32,
                                      iterations: u32) -> Result<(Array, i32), AfError> {
    unsafe {
        let otype = OutType::get_af_dtype();
        let mut inliers: i32 = 0;
        let mut temp: i64    = 0;
        let err_val = af_homography(&mut temp as MutAfArray, &mut inliers as *mut c_int,
                                    x_src.get() as AfArray, y_src.get() as AfArray,
                                    x_dst.get() as AfArray, y_dst.get() as AfArray,
                                    htype as c_int, inlier_thr as c_float,
                                    iterations as c_uint, otype as c_int);
        match err_val {
            0 => Ok( (Array::from(temp), inliers) ),
            _ => Err(AfError::from(err_val)),
        }
    }
}

extern crate libc;

use super::Array as Array;
use super::MatchType as MatchType;
use self::libc::{c_void, uint8_t, c_uint, c_int, c_float, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;
type MutFeat    = *mut *mut self::libc::c_void;
type Feat       = *const self::libc::c_void;

#[allow(dead_code)]
extern {
    fn af_create_features(feat: MutFeat, num: DimT);
    fn af_retain_features(feat: MutFeat, feat: Feat);
    fn af_get_features_num(num: *mut DimT, feat: Feat);
    fn af_get_features_xpos(out: MutAfArray, feat: Feat);
    fn af_get_features_ypos(out: MutAfArray, feat: Feat);
    fn af_get_features_score(out: MutAfArray, feat: Feat);
    fn af_get_features_orientation(out: MutAfArray, feat: Feat);
    fn af_get_features_size(out: MutAfArray, feat: Feat);
    fn af_release_features(feat: *mut c_void);

    fn af_fast(out: MutFeat, input: AfArray, thr: c_float, arc_len: c_uint, non_max: c_int,
               feature_ratio: c_float, edge: c_uint);

    fn af_orb(out: MutFeat, desc: MutAfArray, arr: AfArray, fast_thr: c_float, max_feat: c_uint,
              scl_fctr: c_float, levels: c_uint, blur_img: c_int);

    fn af_hamming_matcher(idx: MutAfArray, dist: MutAfArray,
                          query: AfArray, train: AfArray,
                          dist_dim: DimT, n_dist: c_uint);

    fn af_match_template(out: MutAfArray, search_img: AfArray, template_img: AfArray,
                         mtype: uint8_t);
}

pub struct Features {
    feat: i64,
}

macro_rules! feat_func_def {
    ($fn_name: ident, $ffi_name: ident) => (
        pub fn $fn_name(&self) -> Array {
            unsafe {
                let mut temp: i64 = 0;
                $ffi_name(&mut temp as MutAfArray, self.feat as Feat);
                Array {handle: temp}
            }
        }
    )
}

impl Features {
    #[allow(unused_mut)]
    pub fn new(n: u64) -> Features {
        unsafe {
            let mut temp: i64 = 0;
            af_create_features(&mut temp as *mut c_longlong as MutFeat,
                               n as DimT);
            Features {feat: temp}
        }
    }

    pub fn num_features(&self) -> i64 {
        unsafe {
            let mut temp: i64 = 0;
            af_get_features_num(&mut temp as *mut DimT,
                                self.feat as *const c_longlong as Feat);
            temp
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
            af_retain_features(&mut temp as *mut c_longlong as MutFeat,
                               self.feat as *const c_longlong as Feat);
            Features {feat: temp}
        }
    }
}

impl Drop for Features {
    fn drop(&mut self) {
        unsafe {
            af_release_features(self.feat as *mut c_longlong as *mut c_void);
        }
    }
}

#[allow(unused_mut)]
pub fn fast(input: &Array, thr: f32, arc_len: u32,
            non_max: bool, feat_ratio: f32, edge: u32) -> Features {
    unsafe {
        let mut temp: i64 = 0;
        af_fast(&mut temp as *mut c_longlong as MutFeat,
                input.get() as AfArray, thr as c_float, arc_len as c_uint,
                non_max as c_int, feat_ratio as c_float, edge as c_uint);
        Features {feat: temp}
    }
}

#[allow(unused_mut)]
pub fn orb(input: &Array, fast_thr: f32, max_feat: u32,
           scl_fctr: f32, levels: u32, blur_img: bool) -> (Features, Array) {
    unsafe {
        let mut f: i64 = 0;
        let mut d: i64 = 0;
        af_orb(&mut f as *mut c_longlong as MutFeat, &mut d as MutAfArray,
               input.get() as AfArray, fast_thr as c_float,
               max_feat as c_uint, scl_fctr as c_float, levels as c_uint, blur_img as c_int);
        (Features {feat: f}, Array {handle: d})
    }
}

#[allow(unused_mut)]
pub fn hamming_matcher(query: &Array, train: &Array,
                       dist_dims: i64, n_dist: u32) -> (Array, Array) {
    unsafe {
        let mut idx: i64 = 0;
        let mut dist:i64 = 0;
        af_hamming_matcher(&mut idx as MutAfArray, &mut dist as MutAfArray,
                           query.get() as AfArray, train.get() as AfArray,
                           dist_dims as DimT, n_dist as c_uint);
        (Array {handle: idx}, Array {handle: dist})
    }
}

#[allow(unused_mut)]
pub fn match_template(search_img: &Array, template_img: &Array, mtype: MatchType) -> Array {
    unsafe {
        let mut temp: i64 = 0;
        af_match_template(&mut temp as MutAfArray,
                          search_img.get() as AfArray, template_img.get() as AfArray,
                          mtype as uint8_t);
        Array {handle: temp}
    }
}

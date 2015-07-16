extern crate libc;

use array::Array;
use defines::AfError;
use defines::MatchType;
use self::libc::{c_void, uint8_t, c_uint, c_int, c_float, c_longlong};

type MutAfArray = *mut self::libc::c_longlong;
type AfArray    = self::libc::c_longlong;
type DimT       = self::libc::c_longlong;
type MutFeat    = *mut *mut self::libc::c_void;
type Feat       = *const self::libc::c_void;

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

    fn af_orb(out: MutFeat, desc: MutAfArray, arr: AfArray, fast_thr: c_float, max_feat: c_uint,
              scl_fctr: c_float, levels: c_uint, blur_img: c_int) -> c_int;

    fn af_hamming_matcher(idx: MutAfArray, dist: MutAfArray,
                          query: AfArray, train: AfArray,
                          dist_dim: DimT, n_dist: c_uint) -> c_int;

    fn af_match_template(out: MutAfArray, search_img: AfArray, template_img: AfArray,
                         mtype: uint8_t) -> c_int;
}

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

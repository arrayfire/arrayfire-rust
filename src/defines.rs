extern crate num;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use self::num::Complex;

/// Error codes
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AfError {
    /// The function returned successfully
    SUCCESS            =   0,
    // 100-199 Errors in environment
    /// The system or device ran out of memory
    ERR_NO_MEM         = 101,
    /// There was an error in the device driver
    ERR_DRIVER         = 102,
    /// There was an error with the runtime environment
    ERR_RUNTIME        = 103,
    // 200-299 Errors in input parameters
    /// The input array is not a valid Array object
    ERR_INVALID_ARRAY  = 201,
    /// One of the function arguments is incorrect
    ERR_ARG            = 202,
    /// The size is incorrect
    ERR_SIZE           = 203,
    /// The type is not suppported by this function
    ERR_TYPE           = 204,
    /// The type of the input arrays are not compatible
    ERR_DIFF_TYPE      = 205,
    /// Function does not support GFOR / batch mode
    ERR_BATCH          = 207,
    /// Input does not belong to the current device
    ERR_DEVICE         = 208,
    // 300-399 Errors for missing software features
    /// The option is not supported
    ERR_NOT_SUPPORTED  = 301,
    /// This build of ArrayFire does not support this feature
    ERR_NOT_CONFIGURED = 302,
    // 400-499 Errors for missing hardware features
    /// This device does not support double
    ERR_NO_DBL         = 401,
    /// This build of ArrayFire was not built with graphics or this device does
    /// not support graphics
    ERR_NO_GFX         = 402,
    // 900-999 Errors from upstream libraries and runtimes
    /// There was an internal error either in ArrayFire or in a project
    /// upstream
    ERR_INTERNAL       = 998,
    /// Unknown Error
    ERR_UNKNOWN        = 999
}

/// Compute/Acceleration Backend
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Backend {
    /// Default backend order: OpenCL -> CUDA -> CPU
    DEFAULT = 0,
    /// CPU a.k.a sequential algorithms
    CPU     = 1,
    /// CUDA Compute Backend
    CUDA    = 2,
    /// OpenCL Compute Backend
    OPENCL  = 4
}

impl Display for Backend {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let text = match *self {
            Backend::OPENCL  => "OpenCL",
            Backend::CUDA    => "Cuda",
            Backend::CPU     => "CPU",
            Backend::DEFAULT => "Default",
        };
        write!(f, "{}", text)
    }
}

impl Display for AfError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.description())
    }
}

impl Error for AfError {
    fn description(&self) -> &str {
        match *self {
            AfError::SUCCESS            => "Function returned successfully",
            AfError::ERR_NO_MEM         => "System or Device ran out of memory",
            AfError::ERR_DRIVER         => "Error in the device driver",
            AfError::ERR_RUNTIME        => "Error with the runtime environment",
            AfError::ERR_INVALID_ARRAY  => "Iput Array is not a valid object",
            AfError::ERR_ARG            => "One of the function arguments is incorrect",
            AfError::ERR_SIZE           => "Size is incorrect",
            AfError::ERR_TYPE           => "Type is not suppported by this function",
            AfError::ERR_DIFF_TYPE      => "Type of the input arrays are not compatible",
            AfError::ERR_BATCH          => "Function does not support GFOR / batch mode",
            AfError::ERR_DEVICE         => "Input does not belong to the current device",
            AfError::ERR_NOT_SUPPORTED  => "Unsupported operation/parameter option",
            AfError::ERR_NOT_CONFIGURED => "This build of ArrayFire does not support this feature",
            AfError::ERR_NO_DBL         => "This device does not support double",
            AfError::ERR_NO_GFX         => "This build of ArrayFire has no graphics support",
            AfError::ERR_INTERNAL       => "Error either in ArrayFire or in a project upstream",
            AfError::ERR_UNKNOWN        => "Unknown Error",
        }
    }
}

/// Types of Array data type
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DType {
    /// 32 bit float
    F32 = 0,
    /// 32 bit complex float
    C32 = 1,
    /// 64 bit float
    F64 = 2,
    /// 64 bit complex float
    C64 = 3,
    /// 8 bit boolean
    B8  = 4,
    /// 32 bit signed integer
    S32 = 5,
    /// 32 bit unsigned integer
    U32 = 6,
    /// 8 bit unsigned integer
    U8  = 7,
    /// 64 bit signed integer
    S64 = 8,
    /// 64 bit unsigned integer
    U64 = 9,
    /// 16 bit signed integer
    S16 = 10,
    /// 16 bit unsigned integer
    U16 = 11,
}

/// Dictates the interpolation method to be used by a function
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InterpType {
    /// Nearest Neighbor interpolation method
    NEAREST = 0,
    /// Linear interpolation method
    LINEAR  = 1,
    /// Bilinear interpolation method
    BILINEAR= 2,
    /// Cubic interpolation method
    CUBIC   = 3,
    /// Floor indexed
    LOWER   = 4,
    /// Linear interpolation with cosine smoothing
    LINEAR_COSINE   = 5,
    /// Bilinear interpolation with cosine smoothing
    BILINEAR_COSINE = 6,
    /// Bicubic interpolation
    BICUBIC         = 7,
    /// Cubic interpolation with Catmull-Rom splines
    CUBIC_SPLINE    = 8,
    /// Bicubic interpolation with Catmull-Rom splines
    BICUBIC_SPLINE  = 9
}

/// Helps determine how to pad kernels along borders
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BorderType {
    /// Pad using zeros
    ZERO = 0,
    /// Pad using mirrored values along border
    SYMMETRIC = 1,
}

/// Used by `regions` function to identify type of connectivity
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Connectivity {
    /// North-East-South-West (N-E-S-W) connectivity from given pixel/point
    FOUR = 4,
    /// N-NE-E-SE-S-SW-W-NW connectivity from given pixel/point
    EIGHT = 8
}

/// Helps determine the size of output of convolution
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConvMode {
    /// Default convolution mode where output size is same as input size
    DEFAULT = 0,
    /// Output of convolution is expanded based on signal and filter sizes
    EXPAND  = 1,
}

/// Helps determine if convolution is in Spatial or Frequency domain
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConvDomain {
    /// ArrayFire chooses whether the convolution will be in spatial domain or frequency domain
    AUTO     = 0,
    /// Convoltion in spatial domain
    SPATIAL  = 1,
    /// Convolution in frequency domain
    FREQUENCY= 2,
}

/// Error metric used by `matchTemplate` function
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatchType {
    /// Sum of Absolute Differences
    SAD = 0,
    /// Zero-mean Sum of Absolute Differences
    ZSAD= 1,
    /// Locally scaled Sum of Absolute Differences
    LSAD= 2,
    /// Sum of Squared Differences
    SSD = 3,
    /// Zero-mean Sum of Squared Differences
    ZSSD= 4,
    /// Localy scaled Sum of Squared Differences
    LSSD= 5,
    /// Normalized Cross Correlation
    NCC = 6,
    /// Zero-mean Normalized Cross Correlation
    ZNCC= 7,
    /// Sum of Hamming Distances
    SHD = 8,
}

/// Identify the color space of given image(Array)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSpace {
    /// Grayscale color space
    GRAY = 0,
    /// Red-Green-Blue color space
    RGB  = 1,
    /// Hue-Saturation-value color space
    HSV  = 2,
}

/// Helps determine the type of a Matrix
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MatProp {
    /// Default (no-op)
    NONE,
    /// Data needs to be transposed
    TRANS,
    /// Data needs to be conjugate transposed
    CTRANS,
    /// Matrix is upper triangular
    UPPER,
    /// Matrix is lower triangular
    LOWER,
    /// Matrix diagonal has unitary values
    DIAGUNIT,
    /// Matrix is symmetric
    SYM,
    /// Matrix is positive definite
    POSDEF,
    /// Matrix is orthogonal
    ORTHOG,
    /// Matrix is tri-diagonal
    TRIDIAG,
    /// Matrix is block-diagonal
    BLOCKDIAG,
}

/// Norm type
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NormType {
    /// Treats input as a vector and return sum of absolute values
    VECTOR_1    = 0,
    /// Treats input as vector and return max of absolute values
    VECTOR_INF  = 1,
    /// Treats input as vector and returns euclidean norm
    VECTOR_2    = 2,
    /// Treats input as vector and returns the p-norm
    VECTOR_P    = 3,
    /// Return the max of column sums
    MATRIX_1    = 4,
    /// Return the max of row sums
    MATRIX_INF  = 5,
    /// Returns the max singular value (Currently not supported)
    MATRIX_2    = 6,
    /// Returns Lpq-norm
    MATRIX_L_PQ = 7,
}

/// Dictates what color map is used for Image rendering
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMap {
    /// Default color map is grayscale range [0-1]
    DEFAULT = 0,
    /// Visible spectrum color map
    SPECTRUM= 1,
    /// Colors
    COLORS  = 2,
    /// Red hue map
    RED     = 3,
    /// Mood color map
    MOOD    = 4,
    /// Heat color map
    HEAT    = 5,
    /// Blue hue map
    BLUE    = 6,
}

/// YCbCr Standards
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum YCCStd {
    /// ITU-R BT.601 (formerly CCIR 601) standard
    YCC_601 = 601,
    /// ITU-R BT.709 standard
    YCC_709 = 709,
    /// ITU-R BT.2020 standard
    YCC_2020 = 2020,
}

/// Homography type
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HomographyType {
    /// RANdom SAmple Consensus algorithm
    RANSAC = 0,
    /// Least Median of Squares
    LMEDS  = 1,
}

/// Plotting markers
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MarkerType {
    /// No marker
    NONE     = 0,
    /// Pointer marker
    POINT    = 1,
    /// Hollow circle marker
    CIRCLE   = 2,
    /// Hollow Square marker
    SQUARE   = 3,
    /// Hollow Triangle marker
    TRIANGLE = 4,
    /// Cross-hair marker
    CROSS    = 5,
    /// Plus symbol marker
    PLUS     = 6,
    /// Start symbol marker
    STAR     = 7
}

/// Image moment types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MomentType {
    /// Central moment of order (0 + 0)
    M00 = 1,    // 1<<0
    /// Central moment of order (0 + 1)
    M01 = 2,    // 1<<1
    /// Central moment of order (1 + 0)
    M10 = 4,    // 1<<2
    /// Central moment of order (1 + 1)
    M11 = 8,    // 1<<3
    /// All central moments of order (0,0), (0,1), (1,0) and (1,1)
    FIRST_ORDER = 1<<0 | 1<<1 | 1<<2 | 1<<3
}

/// Sparse storage format type
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SparseFormat {
    /// Dense format
    DENSE = 0,
    /// Compressed sparse row format
    CSR   = 1,
    /// Compressed sparse coloumn format
    CSC   = 2,
    /// Coordinate list (row, coloumn, value) tuples.
    COO   = 3
}

/// Binary operation types for generalized scan functions
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOp {
    /// Addition operation
    ADD = 0,
    /// Multiplication operation
    MUL = 1,
    /// Minimum operation
    MIN = 2,
    /// Maximum operation
    MAX = 3
}

/// Random engine types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RandomEngineType {
    ///Philox variant with N=4, W=32 and Rounds=10
    PHILOX_4X32_10      = 100,
    ///Threefry variant with N=2, W=32 and Rounds=16
    THREEFRY_2X32_16    = 200,
    ///Mersenne variant with MEXP = 11213
    MERSENNE_GP11213    = 300
}

/// Default Philon RandomEngine that points to [PHILOX_4X32_10](./enum.RandomEngineType.html)
pub const PHILOX   : RandomEngineType = RandomEngineType::PHILOX_4X32_10;
/// Default Threefry RandomEngine that points to [THREEFRY_2X32_16](./enum.RandomEngineType.html)
pub const THREEFRY : RandomEngineType = RandomEngineType::THREEFRY_2X32_16;
/// Default Mersenne RandomEngine that points to [MERSENNE_GP11213](./enum.RandomEngineType.html)
pub const MERSENNE : RandomEngineType = RandomEngineType::MERSENNE_GP11213;
/// Default RandomEngine that defaults to [PHILOX](./constant.PHILOX.html)
pub const DEFAULT_RANDOM_ENGINE : RandomEngineType = PHILOX;

/// Scalar value types
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scalar {
    /// 32 bit float
    F32(f32),
    /// 32 bit complex float
    C32(Complex<f32>),
    /// 64 bit float
    F64(f64),
    /// 64 bit complex float
    C64(Complex<f64>),
    /// 8 bit boolean
    B8(bool),
    /// 32 bit signed integer
    S32(i32),
    /// 32 bit unsigned integer
    U32(u32),
    /// 8 bit unsigned integer
    U8(u8),
    /// 64 bit signed integer
    S64(i64),
    /// 64 bit unsigned integer
    U64(u64),
    /// 16 bit signed integer
    S16(i16),
    /// 16 bit unsigned integer
    U16(u16),
}

/// Canny edge detector threshold operations types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CannyThresholdType {
    /// User has to define canny thresholds manually
    MANUAL = 0,
    /// Determine canny algorithm high threshold using Otsu algorithm automatically
    OTSU = 1,
}

/// Anisotropic diffusion flux equation types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiffusionEq {
    /// Quadratic flux function
    QUADRATIC = 1,
    /// Exponential flux function
    EXPONENTIAL = 2,
    /// Default flux function, a.k.a exponential
    DEFAULT = 0,
}

/// Diffusion equation types
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FluxFn {
    /// Quadratic flux function
    GRADIENT = 1,
    /// Modified curvature diffusion equation
    MCDE = 2,
    /// Default diffusion method, Gradient
    DEFAULT = 0,
}

/// topk function ordering
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TopkFn {
    /// Top k min values
    MIN = 1,
    /// Top k max values
    MAX = 2,
    /// Default option(max)
    DEFAULT = 0,
}

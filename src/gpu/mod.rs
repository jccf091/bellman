mod error;

pub use self::error::*;

#[cfg(feature = "gpu")]
mod locks;

#[cfg(feature = "gpu")]
pub use self::locks::*;

#[cfg(feature = "gpu")]
mod sources;

#[cfg(feature = "gpu")]
pub use self::sources::*;

#[cfg(feature = "gpu")]
mod utils;

#[cfg(feature = "gpu")]
pub use self::utils::*;

#[cfg(feature = "gpu")]
mod fft;

#[cfg(feature = "gpu")]
pub use self::fft::*;

#[cfg(feature = "gpu")]
mod multiexp;

#[cfg(feature = "gpu")]
pub use self::multiexp::*;

#[cfg(not(feature = "gpu"))]
mod nogpu;

#[cfg(not(feature = "gpu"))]
pub use self::nogpu::*;


#[cfg(feature = "gpu")]	
use rust_gpu_tools::opencl;
#[cfg(feature = "gpu")]	
lazy_static::lazy_static! {	
    pub static ref GPU_DEVICES: Vec<opencl::Device> = get_devices();
}
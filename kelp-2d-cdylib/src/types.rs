use interoptopus::{ffi_type, patterns::slice::FFISlice};
use kelp_2d::{BlendMode, InstanceData, KelpError, KelpTextureId};

/// The main return type for unit returning functions with error handling
#[ffi_type(patterns(ffi_error))]
#[repr(C)]
pub enum FFIError {
    Success = 0,
    Null = 1,
    Panic = 2,
    // Kelp API specific errors
    NoCurrentFrame = 100,
    SwapchainError = 101,
    InvalidTextureId = 102,
    InvalidBindGroupId = 103,
    InvalidPipelineId = 104,
    NoAdapter = 105,
    NoDevice = 106,
    // Kelp FFI specific errors
    KelpAlreadyInitialised = 200,
    KelpNotInitialised = 201,
}

impl Default for FFIError {
    fn default() -> Self {
        Self::Success
    }
}

impl interoptopus::patterns::result::FFIError for FFIError {
    const SUCCESS: Self = Self::Success;
    const NULL: Self = Self::Null;
    const PANIC: Self = Self::Panic;
}

impl From<KelpError> for FFIError {
    fn from(error: KelpError) -> Self {
        match error {
            KelpError::NoCurrentFrame => FFIError::NoCurrentFrame,
            KelpError::SwapchainError(_) => FFIError::SwapchainError,
            KelpError::InvalidTextureId => FFIError::InvalidTextureId,
            KelpError::InvalidBindGroupId => FFIError::InvalidBindGroupId,
            KelpError::InvalidPipelineId => FFIError::InvalidPipelineId,
            KelpError::NoAdapter => FFIError::NoAdapter,
            KelpError::NoDevice(_) => FFIError::NoDevice,
        }
    }
}

/// A batch of instances to be added to a render pass
#[ffi_type]
#[repr(C)]
pub struct InstanceBatch<'a> {
    pub texture: KelpTextureId,
    pub smooth: bool,
    pub blend_mode: BlendMode,
    pub instances: FFISlice<'a, InstanceData>,
}

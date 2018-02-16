// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Grafiska
//!
//! Grafiska provides a small and simple to use API for doing 3D graphics.
//!
//! ## Resource Management
//!
//! Instead of pointers, resource creation functions return a 32-bit
//! number which uniquely identifies the resource object.
//!
//! The 32-bit resource id is split into a 16-bit pool index in the lower bits,
//! and a 16-bit 'unique counter' in the upper bits. The index allows fast
//! pool lookups, and combined with the unique-mask it allows to detect
//! 'dangling accesses' (trying to use an object which no longer exists, and
//! its pool slot has been reused for a new object)
//!
//! The resource ids are wrapped into a struct so that the compiler
//! can complain when the wrong resource type is used.
//!
//! ## History
//!
//! Grafiska started life as a port of [Sokol](https://github.com/floooh/sokol/)
//! to Rust.

#![warn(missing_docs)]
#![deny(trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]
// For now ...
#![allow(unused_variables, dead_code)]

#[macro_use]
extern crate bitflags;

use std::os;

#[allow(unused_imports)]
use std::ptr;

#[cfg(feature = "gl")]
mod opengl;

#[cfg(feature = "metal")]
mod metal;

#[cfg(feature = "gl")]
use opengl as backend;

#[cfg(feature = "metal")]
use metal as backend;

mod pool;

/// A buffer resource handle.
///
/// Buffers contain vertex and index data.
#[derive(Debug, Copy, Clone, Default)]
pub struct Buffer {
    /// The ID of the underlying buffer resource.
    id: u32,
}

/// An image resource handle.
///
/// Images represent textures and render targets.
#[derive(Debug, Copy, Clone, Default)]
pub struct Image {
    /// The ID of the underlying image resource.
    id: u32,
}

/// A shader resource handle.
#[derive(Debug, Copy, Clone, Default)]
pub struct Shader {
    /// The ID of the underlying shader resource.
    id: u32,
}

/// A pipeline resource handle.
///
/// Pipelines handle vertex layouts, shader, and render states.
#[derive(Debug, Copy, Clone, Default)]
pub struct Pipeline {
    /// The ID of the underlying pipeline resource.
    id: u32,
}

/// A pass resource handle.
///
/// Passes manage render passes and actions on render targets,
/// like clear or MSAA resolve operations.
#[derive(Debug, Copy, Clone, Default)]
pub struct Pass {
    /// The ID of the underlying pass resource.
    id: u32,
}

#[allow(dead_code, missing_docs)]
const INVALID_ID: u32 = 0;
#[allow(missing_docs)]
pub const NUM_SHADER_STAGES: usize = 2;
#[allow(missing_docs)]
pub const NUM_INFLIGHT_FRAMES: usize = 2;
#[allow(missing_docs)]
pub const MAX_COLOR_ATTACHMENTS: usize = 4;
#[allow(missing_docs)]
pub const MAX_SHADERSTAGE_BUFFERS: usize = 4;
#[allow(missing_docs)]
pub const MAX_SHADERSTAGE_IMAGES: usize = 12;
#[allow(missing_docs)]
pub const MAX_SHADERSTAGE_UBS: usize = 4;
#[allow(missing_docs)]
pub const MAX_UB_MEMBERS: usize = 16;
#[allow(missing_docs)]
pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
/// Maximum number of mipmap levels.
pub const MAX_MIPMAPS: usize = 16;
#[allow(missing_docs)]
pub const MAX_TEXTUREARRAY_LAYERS: usize = 128;
#[allow(missing_docs)]
pub const CUBEFACE_NUM: usize = 6;

/// Optional renderer features.
///
/// Not all features are fully supported by all of the rendering backends.
///
/// Use [`query_feature()`] to check at run-time whether or not the feature
/// is supported:
///
/// ```no_run
/// # use grafiska::*;
/// let grfx = Context::new(Config::default());
/// if grfx.query_feature(Feature::Instancing) {
///     // ...
/// }
/// ```
///
/// [`query_feature()`]: fn.query_feature.html
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Feature {
    Instancing,
    TextureCompressionDXT,
    TextureCompressionPVRTC,
    TextureCompressionATC,
    TextureCompressionETC2,
    TextureFloat,
    TextureHalfFloat,
    OriginBottomLeft,
    OriginTopLeft,
    MSAARenderTargets,
    #[allow(non_camel_case_types)]
    PackedVertexFormat_10_2,
    MultipleRenderTarget,
    ImageType3D,
    ImageTypeArray,
}

/// The current state of a resource in its resource pool.
///
/// Resources start in the INITIAL state, which means the
/// pool slot is unoccupied and can be allocated. When a resource is
/// created, first an id is allocated, and the resource pool slot
/// is set to state ALLOC. After allocation, the resource is
/// initialized, which may result in the VALID or FAILED state. The
/// reason why allocation and initialization are separate is because
/// some resource types (e.g. buffers and images) might be asynchronously
/// initialized by the user application. If a resource which is not
/// in the VALID state is attempted to be used for rendering, rendering
/// operations will silently be dropped.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResourceState {
    /// The resource slot is unoccupied and can be allocated.
    Initial,
    /// An ID has been allocated, but the resource has not yet been initialized.
    Alloc,
    /// The resource has been allocated and initialized.
    Valid,
    /// Initializing the resource failed.
    Failed,
}

impl Default for ResourceState {
    fn default() -> Self {
        ResourceState::Initial
    }
}

/// A resource usage hint describing the update strategy of
/// buffers and images. This is used in the [`BufferDesc`]
/// and [`ImageDesc`] `usage` members when creating buffers
/// and images.
///
/// The rendering backends use this hint to prevent that the
/// CPU needs to wait for the GPU when attempting to update
/// a resource that might be currently accessed by the GPU.
///
/// Resource content is updated with the function [`update_buffer()`] for
/// buffer objects, and [`update_image()`] for image objects. Only
/// one update is allowed per frame and resource object. The
/// application must update all data required for rendering (this
/// means that the update data can be smaller than the resource size,
/// if only a part of the overall resource size is used for rendering,
/// you only need to make sure that the data that *is* used is valid.
///
/// The default usage is `Usage::Immutable`.
///
/// [`BufferDesc`]: struct.BufferDesc.html
/// [`ImageDesc`]: struct.ImageDesc.html
/// [`update_buffer()`]: fn.update_buffer.html
/// [`update_image()`]: fn.update_image.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Usage {
    /// The resource will never be updated with new data, instead, the
    /// data content of the resource must be provided on creation.
    Immutable = 1,
    /// The resource will be updated infrequently with new data. This could
    /// range from "once after creation", to "quite often, but not every
    /// frame."
    Dynamic = 2,
    /// The resource will be updated each frame with new content.
    Stream = 3,
}

impl Default for Usage {
    fn default() -> Self {
        Usage::Immutable
    }
}

/// Indicates whether a buffer contains vertex or index data.
///
/// Used in the [`BufferDesc`] `type` member when creating a buffer.
///
/// The default value is `BufferType::VertexBuffer`.
///
/// [`BufferDesc`]: struct.BufferDesc.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BufferType {
    /// Vertex data.
    VertexBuffer,
    /// Index data.
    IndexBuffer,
}

impl Default for BufferType {
    fn default() -> Self {
        BufferType::VertexBuffer
    }
}

/// Indicates whether indexed rendering (fetching vertex-indices from an
/// index buffer) is used, and if yes, the index data type (16- or 32-bits).
///
/// This is used in the [`PipelineDesc`] `index_type` member when creating a
/// pipeline object.
///
/// [`PipelineDesc`]: struct.PipelineDesc.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum IndexType {
    /// Index data is 16 bit.
    UInt16,
    /// Index data is 32 bit.
    UInt32,
}

/// Indicates the basic image type (2D texture, cube map, 3D texture, or
/// array of 2D textures).
///
/// 3D and array textures are not supported on the GLES2 / WebGL backend.
///
/// The image type is used in the [`ImageDesc`] `type` member when creating
/// an image.
///
/// The default image type when creating an image is `ImageType::Texture2D`.
///
/// [`ImageDesc`]: struct.ImageDesc.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ImageType {
    /// A 2D texture.
    Texture2D,
    /// A cube map.
    Cube,
    /// A 3D texture.
    Texture3D,
    /// An array of 2D textures.
    Array,
}

impl Default for ImageType {
    fn default() -> Self {
        ImageType::Texture2D
    }
}

/// There are 2 shader stages: vertex and fragment.
///
/// Each shader stage consists of:
///
/// * One slot for a shader function, provided as source
///   or byte code.
/// * `MAX_SHADERSTAGE_UBS` slots for uniform blocks.
/// * `MAX_SHADERSTAGE_IMAGES` slots for images used as textures
///   by the shader function.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    /// Vertex shader stage.
    VS,
    /// Fragment shader stage.
    FS,
}

/// A common subset of useful and widely supported pixel formats.
///
/// The pixel format enum is mainly used when creating an image object
/// in the [`ImageDesc`] `pixel_format` member.
///
/// The default pixel format when creating an image is `PixelFormat::RGBA8`.
///
/// [`ImageDesc`]: struct.ImageDesc.html
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    None,
    RGBA8,
    RGB8,
    RGBA4,
    R5G6B5,
    R5G5B5A1,
    R10G10B10A2,
    RGBA32F,
    RGBA16F,
    R32F,
    R16F,
    L8,
    DXT1,
    DXT3,
    DXT5,
    Depth,
    DepthStencil,
    PVRTC2_RGB,
    PVRTC4_RGB,
    PVRTC2_RGBA,
    PVRTC4_RGBA,
    ETC2_RGB8,
    ETC2_SRGB8,
}

impl Default for PixelFormat {
    fn default() -> Self {
        PixelFormat::RGBA8
    }
}

impl PixelFormat {
    /// Return `true` if pixel format is a compressed format.
    pub fn is_compressed_pixel_format(self) -> bool {
        match self {
            PixelFormat::DXT1
            | PixelFormat::DXT3
            | PixelFormat::DXT5
            | PixelFormat::PVRTC2_RGB
            | PixelFormat::PVRTC4_RGB
            | PixelFormat::PVRTC2_RGBA
            | PixelFormat::PVRTC4_RGBA
            | PixelFormat::ETC2_RGB8
            | PixelFormat::ETC2_SRGB8 => true,
            _ => false,
        }
    }

    /// Return `true` if pixel format is a valid render target color format.
    pub fn is_valid_rendertarget_color_format(self) -> bool {
        match self {
            PixelFormat::RGBA8
            | PixelFormat::R10G10B10A2
            | PixelFormat::RGBA32F
            | PixelFormat::RGBA16F => true,
            _ => false,
        }
    }

    /// Return `true` if pixel format is a valid render target color format.
    pub fn is_valid_rendertarget_depth_format(self) -> bool {
        match self {
            PixelFormat::Depth | PixelFormat::DepthStencil => true,
            _ => false,
        }
    }

    /// Return `true` if pixel format is a depth-stencil format.
    pub fn is_depth_stencil_format(self) -> bool {
        self == PixelFormat::DepthStencil
    }

    /// Return the bytes per pixel for a pixel format.
    pub fn bytesize(self) -> usize {
        match self {
            PixelFormat::RGBA32F => 16,
            PixelFormat::RGBA16F => 8,
            PixelFormat::RGBA8 | PixelFormat::R10G10B10A2 | PixelFormat::R32F => 4,
            PixelFormat::RGB8 => 3,
            PixelFormat::R5G5B5A1
            | PixelFormat::R5G6B5
            | PixelFormat::RGBA4
            | PixelFormat::R16F => 2,
            PixelFormat::L8 => 1,
            _ => unreachable!(),
        }
    }

    /// Return row pitch for an image.
    pub fn row_pitch(self, width: usize) -> usize {
        match self {
            PixelFormat::DXT1 | PixelFormat::ETC2_RGB8 | PixelFormat::ETC2_SRGB8 => {
                let pitch = ((width + 3) / 4) * 8;
                if pitch < 8 {
                    8
                } else {
                    pitch
                }
            }
            PixelFormat::DXT3 | PixelFormat::DXT5 => {
                let pitch = ((width + 3) / 4) * 16;
                if pitch < 16 {
                    16
                } else {
                    pitch
                }
            }
            PixelFormat::PVRTC4_RGB | PixelFormat::PVRTC4_RGBA => {
                let block_size = 4 * 4;
                let bpp = 4;
                let width_blocks = ::std::cmp::max(2, width / 4);
                width_blocks * ((block_size * bpp) / 8)
            }
            PixelFormat::PVRTC2_RGB | PixelFormat::PVRTC2_RGBA => {
                let block_size = 8 * 4;
                let bpp = 2;
                let width_blocks = ::std::cmp::max(2, width / 4);
                width_blocks * ((block_size * bpp) / 8)
            }
            _ => width * PixelFormat::bytesize(self),
        }
    }

    /// Return pitch of a 2D subimage / texture slice.
    pub fn surface_pitch(self, width: usize, height: usize) -> usize {
        let num_rows = match self {
            PixelFormat::DXT1
            | PixelFormat::DXT3
            | PixelFormat::DXT5
            | PixelFormat::ETC2_RGB8
            | PixelFormat::ETC2_SRGB8
            | PixelFormat::PVRTC2_RGB
            | PixelFormat::PVRTC2_RGBA
            | PixelFormat::PVRTC4_RGB
            | PixelFormat::PVRTC4_RGBA => ((height + 3) / 4),
            _ => height,
        };
        ::std::cmp::max(1, num_rows) * PixelFormat::row_pitch(self, width)
    }
}

/// A common subset of 3D primitive types supported across all 3D
/// APIs.
///
/// This is used in the [`PipelineDesc`] `primitive_type` member when
/// creating a pipeline object.
///
/// The default primitive type is `PrimitiveType::Triangles`.
///
/// [`PipelineDesc`]: struct.PipelineDesc.html
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Points,
    Lines,
    LineStrip,
    Triangles,
    TriangleStrip,
}

impl Default for PrimitiveType {
    fn default() -> Self {
        PrimitiveType::Triangles
    }
}

/// The filter mode when sampling a texture image.
///
/// This is used in the [`ImageDesc`] `min_filter` and `mag_filter`
/// members when creating an image object.
///
/// The default filter mode is `Filter::Nearest`.
///
/// [`ImageDesc`]: struct.ImageDesc.html
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Filter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::Nearest
    }
}

/// The texture coordinates wrapping mode when sampling a texture
/// image.
///
/// This is used in the [`ImageDesc`] `wrap_u`, `wrap_v`, and `wrap_w`
/// members when creating an image.
///
/// The default wrap mode is `Wrap::Repeat`.
///
/// [`ImageDesc`]: struct.ImageDesc.html
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Wrap {
    Repeat,
    ClampToEdge,
    MirroredRepeat,
}

impl Default for Wrap {
    fn default() -> Self {
        Wrap::Repeat
    }
}

/// The data type of a vertex component.
///
/// This is used to describe the layout of vertex data when creating
/// a pipeline object.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VertexFormat {
    Float,
    Float2,
    Float3,
    Float4,
    Byte4,
    Byte4N,
    UByte4,
    UByte4N,
    Short2,
    Short2N,
    Short4,
    Short4N,
    UInt10N2,
}

impl VertexFormat {
    /// Size in bytes for a vertex format.
    pub fn bytesize(self) -> usize {
        match self {
            VertexFormat::Float => 4,
            VertexFormat::Float2 => 8,
            VertexFormat::Float3 => 12,
            VertexFormat::Float4 => 16,
            VertexFormat::Byte4 => 4,
            VertexFormat::Byte4N => 4,
            VertexFormat::UByte4 => 4,
            VertexFormat::UByte4N => 4,
            VertexFormat::Short2 => 4,
            VertexFormat::Short2N => 4,
            VertexFormat::Short4 => 8,
            VertexFormat::Short4N => 8,
            VertexFormat::UInt10N2 => 4,
        }
    }
}

/// Defines whether the input pointer of a vertex input stream is
/// advanced 'per vertex' or 'per instance'.
///
/// The default step function is `VertexStep::PerVertex`.
/// `VertexStep::PerInstance` is used with instanced rendering.
///
/// The vertex step is part of the vertex layout definition when creating
/// pipeline objects.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VertexStep {
    PerVertex,
    PerInstance,
}

impl Default for VertexStep {
    fn default() -> Self {
        VertexStep::PerVertex
    }
}

/// The data type of a uniform block member.
///
/// This is used to describe the internal layout of uniform blocks
/// when creating a shader object.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum UniformType {
    Float,
    Float2,
    Float3,
    Float4,
    Mat4,
}

impl Default for UniformType {
    fn default() -> Self {
        UniformType::Float
    }
}

impl UniformType {
    /// Return the byte size of a shader uniform.
    pub fn bytesize(self, count: usize) -> usize {
        match self {
            UniformType::Float => 4 * count,
            UniformType::Float2 => 8 * count,
            UniformType::Float3 => 12 * count, // FIXME: std140???
            UniformType::Float4 => 16 * count,
            UniformType::Mat4 => 64 * count,
        }
    }
}

/// The face-culling mode.
///
/// This is used in the [`PipelineDesc`] `rasterizer`'s
/// `cull_mode` member when creating a pipeline object.
///
/// The default cull mode is `CullMode::None`.
///
/// [`PipelineDesc`]: struct.PipelineDesc.html
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CullMode {
    None,
    Front,
    Back,
}

impl Default for CullMode {
    fn default() -> Self {
        CullMode::None
    }
}

/// The vertex-winding rule that determines a front-facing
/// primitive.
///
/// This is used in the [`PipelineDesc`] `rasterizer`'s
/// `face_winding` member when creating a pipeline object.
///
/// The default winding is `FaceWinding::CW` (clockwise).
///
/// [`PipelineDesc`]: struct.PipelineDesc.html
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FaceWinding {
    /// Counter-clockwise.
    CCW,
    /// Clockwise.
    CW,
}

impl Default for FaceWinding {
    fn default() -> Self {
        FaceWinding::CW
    }
}

/// The compare function for depth and stencil ref tests.
///
/// This is used when creating pipeline objects.
///
/// The default comparison function for depth and stencil tests
/// is `CompareFunc::Always`.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CompareFunc {
    Never,
    Less,
    Equal,
    LessEqual,
    Greater,
    NotEqual,
    GreaterEqual,
    Always,
}

impl Default for CompareFunc {
    fn default() -> Self {
        CompareFunc::Always
    }
}

/// The operation performed on a currently stored stencil alue
/// when a comparison test passes or fails.
///
/// This is used when creating a pipeline object.
///
/// The default value is `StencilOp::Keep`.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    IncrClamp,
    DecrClamp,
    Invert,
    IncrWrap,
    DecrWrap,
}

impl Default for StencilOp {
    fn default() -> Self {
        StencilOp::Keep
    }
}

/// The source and destination factors in blending operations.
///
/// This is used when creating a pipeline object.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    BlendColor,
    OneMinusBlendColor,
    BlendAlpha,
    OneMinusBlendAlpha,
}

/// Describes how the source and destination values are combined in
/// the fragment blending operation.
///
/// It is used when creating a pipeline object.
///
/// The default value is `BlendOp::Add`.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BlendOp {
    Add,
    Subtract,
    ReverseSubtract,
}

impl Default for BlendOp {
    fn default() -> Self {
        BlendOp::Add
    }
}

bitflags! {
    /// Selects the color channels when writing a fragment color to the
    /// framebuffer.
    ///
    /// This is used in the [`PipelineDesc`]'s `blend`'s `color_write_mask`
    /// member when creating a pipeline object.
    ///
    /// The default color mask is `ColorMask::RGBA`, which writes all color
    /// channels.
    ///
    /// [`PipelineDesc`]: struct.PipelineDesc.html
    #[allow(missing_docs)]
    #[repr(C)]
    pub struct ColorMask: u32 {
        /// None
        const NONE = 0x10;
        /// Red
        const R = 1;
        /// Green
        const G = 1 << 1;
        /// Blue
        const B = 1 << 2;
        /// Alpha
        const A = 1 << 3;
        /// Red, green and blue.
        const RGB
            = Self::R.bits |
              Self::G.bits |
              Self::B.bits;
        /// Red, green, blue, alpha. All channels.
        const RGBA
            = Self::RGB.bits |
              Self::A.bits;
    }
}

impl Default for ColorMask {
    fn default() -> Self {
        ColorMask::RGBA
    }
}

/// Defines what action should be performed at the start of a render pass.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    /// Clear the render target image.
    Clear,
    /// Load the previous content of the render target image.
    Load,
    /// Leave the render target image content undefined.
    DontCare,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ColorAttachmentAction {
    pub action: Action,
    pub val: [f32; 4usize],
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct DepthAttachmentAction {
    pub action: Action,
    pub val: f32,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct StencilAttachmentAction {
    pub action: Action,
    pub val: u8,
}

/// The actions to be performed at the start of a rendering pass
/// in the functions [`begin_pass()`] and [`begin_default_pass()`].
///
/// A separate action and clear values can be defined for each
/// color attachment and for the depth-stencil attachment.
///
/// [`begin_pass()`]: fn.begin_pass.html
/// [`begin_default_pass()`]: fn.begin_default_pass.html
#[allow(missing_docs)]
#[derive(Debug)]
pub struct PassAction {
    pub colors: [ColorAttachmentAction; MAX_COLOR_ATTACHMENTS],
    pub depth: DepthAttachmentAction,
    pub stencil: StencilAttachmentAction,
}

/// The resource binding slots of the render pipeline.
///
/// This is passed to `apply_draw_state()`.
///
/// A draw state contains:
///
/// * 1 pipeline object.
/// * 1..N vertex buffers.
/// * 0..1 index buffers.
/// * 0..N vertex shader stage images.
/// * 0..N fragment shader stage images.
///
/// The max number of vertex buffer and shader stage images are defined
/// by the `MAX_SHADERSTAGE_BUFFERS` and `MAX_SHADERSTAGE_IMAGES`
/// configuration constants.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub struct DrawState {
    /// The pipeline to be drawn.
    pub pipeline: Pipeline,
    pub vertex_buffers: [Buffer; MAX_SHADERSTAGE_BUFFERS],
    pub index_buffer: Option<Buffer>,
    pub vs_images: [Image; MAX_SHADERSTAGE_IMAGES],
    pub fs_images: [Image; MAX_SHADERSTAGE_IMAGES],
}

/// Configuration values for the library.
///
/// It is used as a parameter to the `setup()` call.
#[derive(Debug)]
pub struct Config {
    /// Defaults to 128.
    pub buffer_pool_size: usize,
    /// Defaults to 128.
    pub image_pool_size: usize,
    /// Defaults to 32.
    pub shader_pool_size: usize,
    /// Defaults to 64.
    pub pipeline_pool_size: usize,
    /// Defaults to 16.
    pub pass_pool_size: usize,
    #[cfg(feature = "gl")]
    /// If this is true, the backend will operate in "GLES2 fallback mode" even
    /// when compiled for GLES3. This is useful for falling back to traditional
    /// WebGL if a browser doesn't support a WebGL2 context.
    pub gl_force_gles2: bool,
    #[cfg(feature = "metal")]
    /// A pointer to the `MTLDevice` object.
    pub mtl_device: *const os::raw::c_void,
    #[cfg(feature = "metal")]
    /// A C callback function to obtain the `MTLRenderPassDescriptor` for the
    /// current frame when rendering to the default framebuffer. Will be called in
    /// `begin_default_pass()`.
    pub mtl_renderpass_descriptor_cb: Option<unsafe extern "C" fn() -> *const os::raw::c_void>,
    #[cfg(feature = "metal")]
    /// A C callback function to obtain a `MTLDrawable` for the current frame when
    /// rendering to the default framebuffer. Will be called in `end_pass()` of the
    /// default pass.
    pub mtl_drawable_cb: Option<unsafe extern "C" fn() -> *const os::raw::c_void>,
    #[cfg(feature = "metal")]
    /// The size of the global uniform buffer in bytes. This must be big enough to hold all
    /// the uniform block updates for a single frame. The default value is 4MByte (4 * 1024 * 1024).
    pub mtl_global_uniform_buffer_size: usize,
    #[cfg(feature = "metal")]
    /// The number of slots in the sampler cache. The Metal backend will share texture samples
    /// with the same state in this cache. The default value is 64.
    pub mtl_sampler_cache_size: usize,
    #[cfg(feature = "d3d11")]
    /// A pointer to the `ID3D11Device` object. This must have been created before
    /// `setup()` is called.
    pub d3d11_device: *const os::raw::c_void,
    #[cfg(feature = "d3d11")]
    /// A pointer to the `ID3D11DeviceContext` object.
    pub d3d11_device_context: *const os::raw::c_void,
    #[cfg(feature = "d3d11")]
    /// A C callback function to obtain a pointer to the current
    /// `ID3D11RenderTargetView` object of the default framebuffer. This function
    /// will be called in `begin_pass` when rendering to the default framebuffer.
    pub d3d11_render_target_view_cb: Option<unsafe extern "C" fn() -> *const os::raw::c_void>,
    #[cfg(feature = "d3d11")]
    /// A C callback function to obtain a pointer to the current
    /// `ID3D11DepthStencilView` object of the default framebuffer. This function
    /// will be called in `begin_pass` when rendering to the default framebuffer.
    pub d3d11_depth_stencil_view_cb: Option<unsafe extern "C" fn() -> *const os::raw::c_void>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            buffer_pool_size: 128,
            image_pool_size: 128,
            shader_pool_size: 32,
            pipeline_pool_size: 64,
            pass_pool_size: 16,
            #[cfg(feature = "gl")]
            gl_force_gles2: false,
            #[cfg(feature = "metal")]
            mtl_device: ptr::null::<os::raw::c_void>(),
            #[cfg(feature = "metal")]
            mtl_renderpass_descriptor_cb: None,
            #[cfg(feature = "metal")]
            mtl_drawable_cb: None,
            #[cfg(feature = "metal")]
            mtl_global_uniform_buffer_size: 4 * 1024 * 1024,
            #[cfg(feature = "metal")]
            mtl_sampler_cache_size: 64,
            #[cfg(feature = "d3d11")]
            d3d11_device: ptr::null::<os::raw::c_void>(),
            #[cfg(feature = "d3d11")]
            d3d11_device_context: ptr::null::<os::raw::c_void>(),
            #[cfg(feature = "d3d11")]
            d3d11_render_target_view_cb: None,
            #[cfg(feature = "d3d11")]
            d3d11_depth_stencil_view_cb: None,
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct BufferDesc {
    pub size: usize,
    pub buffer_type: BufferType,
    pub usage: Usage,
    pub content: Vec<u8>,
    #[cfg(feature = "gl")]
    pub gl_buffers: [u32; NUM_INFLIGHT_FRAMES],
    #[cfg(feature = "metal")]
    pub metal_buffers: [*const os::raw::c_void; NUM_INFLIGHT_FRAMES],
    #[cfg(feature = "d3d11")]
    pub d3d11_buffers: *const os::raw::c_void,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct SubimageContent<'c> {
    pub content: &'c [u8],
}

/// The content of an image by way of a 2D array of [`SubimageContent`] structs.
///
/// The first array dimension is the cubemap face and the second is the mipmap
/// level.
///
/// [`SubimageContent`]: struct.SubimageContent.html
#[allow(missing_docs)]
#[derive(Debug)]
pub struct ImageContent<'c> {
    pub subimage: [[SubimageContent<'c>; CUBEFACE_NUM]; MAX_MIPMAPS],
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ImageDesc<'c> {
    pub image_type: ImageType,
    pub render_target: bool,
    pub width: usize,
    pub height: usize,
    pub depth_or_layers: u32, // In Sokol, this is a union.
    pub num_mipmaps: usize,
    pub usage: Usage,
    pub pixel_format: PixelFormat,
    pub sample_count: usize,
    pub min_filter: Filter,
    pub mag_filter: Filter,
    pub wrap_u: Wrap,
    pub wrap_v: Wrap,
    pub wrap_w: Wrap,
    pub max_anisotropy: u32,
    pub min_lod: f32,
    pub max_lod: f32,
    pub content: ImageContent<'c>,
    #[cfg(feature = "gl")]
    pub gl_textures: [u32; NUM_INFLIGHT_FRAMES],
    #[cfg(feature = "metal")]
    pub mtl_textures: [*const os::raw::c_void; NUM_INFLIGHT_FRAMES],
    #[cfg(feature = "d3d11")]
    pub d3d11_texture: *const os::raw::c_void,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ShaderUniformDesc {
    pub name: &'static str,
    pub uniform_type: UniformType,
    pub array_count: u32,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ShaderUniformBlockDesc {
    pub size: u32,
    pub uniforms: [ShaderUniformDesc; MAX_UB_MEMBERS],
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ShaderImageDesc {
    pub name: &'static str,
    pub image_type: ImageType,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ShaderStageDesc {
    pub source: &'static str,
    pub byte_code: *const u8,
    pub byte_code_size: u32,
    pub entry: &'static str,
    pub uniform_blocks: [ShaderUniformBlockDesc; MAX_SHADERSTAGE_UBS],
    pub images: [ShaderImageDesc; MAX_SHADERSTAGE_IMAGES],
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct ShaderDesc {
    pub vs: ShaderStageDesc,
    pub fs: ShaderStageDesc,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct VertexAttrDesc {
    pub name: &'static str,
    pub sem_name: &'static str,
    pub sem_index: u32,
    pub offset: u32,
    pub format: VertexFormat,
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct VertexLayoutDesc {
    pub stride: u32,
    pub step_func: VertexStep,
    pub step_rate: u32,
    pub attrs: [VertexAttrDesc; MAX_VERTEX_ATTRIBUTES],
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct StencilState {
    pub fail_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub compare_func: CompareFunc,
}

impl Default for StencilState {
    fn default() -> Self {
        StencilState {
            fail_op: StencilOp::default(),
            depth_fail_op: StencilOp::default(),
            pass_op: StencilOp::default(),
            compare_func: CompareFunc::default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct DepthStencilState {
    pub stencil_front: StencilState,
    pub stencil_back: StencilState,
    pub depth_compare_func: CompareFunc,
    pub depth_write_enabled: bool,
    pub stencil_enabled: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: ColorMask,
    pub stencil_ref: u8,
}

impl Default for DepthStencilState {
    fn default() -> Self {
        DepthStencilState {
            stencil_front: StencilState::default(),
            stencil_back: StencilState::default(),
            depth_compare_func: CompareFunc::default(),
            depth_write_enabled: false,
            stencil_enabled: false,
            stencil_read_mask: 0,
            stencil_write_mask: ColorMask::default(),
            stencil_ref: 0,
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct BlendState {
    pub enabled: bool,
    pub src_factor_rgb: BlendFactor,
    pub dst_factor_rgb: BlendFactor,
    pub op_rgb: BlendOp,
    pub src_factor_alpha: BlendFactor,
    pub dst_factor_alpha: BlendFactor,
    pub op_alpha: BlendOp,
    pub color_write_mask: ColorMask,
    pub color_attachment_count: u32,
    pub color_format: PixelFormat,
    pub depth_format: PixelFormat,
    pub blend_color: [f32; 4usize],
}

impl Default for BlendState {
    fn default() -> Self {
        BlendState {
            enabled: false,
            src_factor_rgb: BlendFactor::One,
            dst_factor_rgb: BlendFactor::Zero,
            op_rgb: BlendOp::Add,
            src_factor_alpha: BlendFactor::One,
            dst_factor_alpha: BlendFactor::Zero,
            op_alpha: BlendOp::Add,
            color_write_mask: ColorMask::RGBA,
            color_attachment_count: 1,
            color_format: PixelFormat::RGBA8,
            depth_format: PixelFormat::DepthStencil,
            blend_color: [0.0, 0.0, 0.0, 0.0],
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct RasterizerState {
    pub alpha_to_coverage_enabled: bool,
    pub cull_mode: CullMode,
    pub face_winding: FaceWinding,
    pub sample_count: u32,
    pub depth_bias: f32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

impl Default for RasterizerState {
    fn default() -> Self {
        RasterizerState {
            alpha_to_coverage_enabled: false,
            cull_mode: CullMode::default(),
            face_winding: FaceWinding::default(),
            sample_count: 0,
            depth_bias: 0f32,
            depth_bias_slope_scale: 0f32,
            depth_bias_clamp: 0f32,
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug)]
pub struct PipelineDesc {
    pub vertex_layouts: [VertexLayoutDesc; MAX_SHADERSTAGE_BUFFERS],
    pub shader: Shader,
    pub primitive_type: PrimitiveType,
    pub index_type: Option<IndexType>,
    pub depth_stencil: DepthStencilState,
    pub blend: BlendState,
    pub rasterizer: RasterizerState,
}

/// An attachment for the [`PassDesc`].
///
/// An attachment consists of an image and two additional
/// indices describing which subimage the pass will render.
///
/// [`PassDesc`]: struct.PassDesc.html
#[derive(Debug)]
pub struct AttachmentDesc {
    /// The image to render.
    pub image: Image,
    /// The mip level to render.
    pub mip_level: usize,
    /// If the image is a cube map, array texture or 3D texture,
    /// the face index, array layer or depth slice to render.
    pub index: usize,
}

/// Creation parameters for a [`Pass`] object.
///
/// This is used as an argument to the `make_pass()` function.
///
/// Pass images must fulfill the following requirements:
///
/// * Must be created as a render target (`ImageDesc.render_target` must be `true`).
/// * All images must be the same size.
/// * All images must have the same sample count.
/// * All color attachment images must have the same pixel format.
///
/// [`Pass`]: struct.Pass.html
#[derive(Debug)]
pub struct PassDesc {
    /// Up to `MAX_COLOR_ATTACHMENTS` color attachments.
    pub color_attachments: [AttachmentDesc; MAX_COLOR_ATTACHMENTS],
    /// An optional depth-stencil attachment.
    pub depth_stencil_attachment: Option<AttachmentDesc>,
}

/// Internal state of a grafiska context.
pub struct Context {
    buffer_pool: pool::Pool<backend::Buffer>,
    image_pool: pool::Pool<backend::Image>,
    shader_pool: pool::Pool<backend::Shader>,
    pipeline_pool: pool::Pool<backend::Pipeline>,
    pass_pool: pool::Pool<backend::Pass>,
    frame_index: u32,
    current_pass: Option<Pass>,
    current_pipeline: Option<Pipeline>,
    backend: backend::Backend,
}

impl Context {
    /// Initialize the Grafiska library.
    ///
    /// This must be performed after creating a window and a 3D API
    /// context/device.
    pub fn new(desc: Config) -> Self {
        Context {
            buffer_pool: pool::Pool::<backend::Buffer>::new(desc.buffer_pool_size),
            image_pool: pool::Pool::<backend::Image>::new(desc.image_pool_size),
            shader_pool: pool::Pool::<backend::Shader>::new(desc.shader_pool_size),
            pipeline_pool: pool::Pool::<backend::Pipeline>::new(desc.pipeline_pool_size),
            pass_pool: pool::Pool::<backend::Pass>::new(desc.pass_pool_size),
            frame_index: 1,
            current_pass: None,
            current_pipeline: None,
            backend: backend::Backend::default(),
        }
    }

    /// Shutdown the Grafiska library.
    pub fn shutdown(&mut self) {
        unimplemented!()
    }

    /// Test to see if a feature is supported by the rendering backend.
    pub fn query_feature(&self, feature: Feature) -> bool {
        self.backend.query_feature(feature)
    }

    /// If you call directly into the underlying 3D API, this must be called
    /// prior to using Grafiska functions again.
    pub fn reset_state_cache(&mut self) {
        self.backend.reset_state_cache();
    }

    /// Create a `Buffer` resource object.
    pub fn make_buffer(&mut self, desc: BufferDesc) -> Buffer {
        unimplemented!();
    }

    /// Create an `Image` resource object.
    pub fn make_image(&mut self, desc: ImageDesc) -> Image {
        unimplemented!();
    }

    /// Create a `Shader` resource object.
    pub fn make_shader(&mut self, desc: ShaderDesc) -> Shader {
        unimplemented!();
    }

    /// Create a `Pipeline` resource object.
    pub fn make_pipeline(&mut self, desc: PipelineDesc) -> Pipeline {
        unimplemented!();
    }

    /// Create a `Pass` resource object.
    pub fn make_pass(&mut self, desc: PassDesc) -> Pass {
        unimplemented!();
    }

    /// Destroy a `Buffer` resource object.
    pub fn destroy_buffer(&mut self, buf: Buffer) {
        unimplemented!();
    }

    /// Destroy an `Image` resource object.
    pub fn destroy_image(&mut self, img: Image) {
        unimplemented!();
    }

    /// Destroy a `Shader` resource object.
    pub fn destroy_shader(&mut self, shd: Shader) {
        unimplemented!();
    }

    /// Destroy a `Pipeline` resource object.
    pub fn destroy_pipeline(&mut self, pip: Pipeline) {
        unimplemented!();
    }

    /// Destroy a `Pass` resource object.
    pub fn destroy_pass(&mut self, pass: Pass) {
        unimplemented!();
    }

    /// Update the content of a buffer resource.
    ///
    /// The resource must have been created with `USAGE_DYNAMIC` or
    /// `USAGE_STREAM`.
    pub fn update_buffer(&mut self, buf: Buffer, data_ptr: *const os::raw::c_void, data_size: u32) {
        unimplemented!();
    }

    /// Update the content of an image resource.
    ///
    /// The resource must have been created with `USAGE_DYNAMIC` or
    /// `USAGE_STREAM`.
    pub fn update_image(&mut self, img: Image, data: ImageContent) {
        unimplemented!();
    }

    /// Start rendering to the default framebuffer.
    pub fn begin_default_pass(&mut self, pass_action: &PassAction, width: u32, height: u32) {
        unimplemented!();
    }

    /// Start rendering to an offscreen framebuffer.
    pub fn begin_pass(&mut self, pass: Pass, pass_action: &PassAction) {
        unimplemented!();
    }

    /// Set a new viewport rectangle.
    ///
    /// This must be called from within a rendering pass.
    ///
    /// Starting a render pass will reset the viewport to the size of the
    /// framebuffer used in the new pass.
    pub fn apply_viewport(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        origin_top_left: bool,
    ) {
        unimplemented!();
    }

    /// Set a new scissor rectangle.
    ///
    /// This must be called from within a rendering pass.
    pub fn apply_scissor_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        origin_top_left: bool,
    ) {
        unimplemented!();
    }

    /// Update the resource bindings for the next draw call.
    ///
    /// Fill a [`DrawState`] struct with the resource bindings for the next draw
    /// call (one pipeline object, 1..N vertex buffers, 0 or 1 index buffer,
    /// 0..N image objects to use as textures each on the vertex and fragment
    /// shader stages.
    ///
    /// [`DrawState`]: struct.DrawState.html
    pub fn apply_draw_state(&mut self, ds: DrawState) {
        unimplemented!();
    }

    /// Update shader uniform data.
    pub fn apply_uniform_block(
        &mut self,
        stage: ShaderStage,
        ub_index: u32,
        data: *const os::raw::c_void,
        num_bytes: u32,
    ) {
        unimplemented!();
    }

    /// Kick off a draw call.
    ///
    /// This uses the resource bindings that were supplied to `apply_draw_state()`
    /// as well as uniform blocks supplied via `apply_uniform_block()`.
    pub fn draw(&mut self, base_element: u32, num_elements: u32, num_instances: u32) {
        unimplemented!();
    }

    /// Finish the current rendering pass.
    ///
    /// If the render target is an MSAA render target, then an MSAA resolve will
    /// occur here.
    pub fn end_pass(&mut self) {
        unimplemented!();
    }

    /// Finish rendering the current frame.
    pub fn commit(&mut self) {
        unimplemented!();
    }

    /// Allocate, without initialization, a `Buffer` resource handle.
    ///
    /// The buffer must subsequently be initialized with [`init_buffer()`].
    ///
    /// [`init_buffer()`]: fn.init_buffer.html
    pub fn alloc_buffer(&mut self) -> Buffer {
        unimplemented!();
    }

    /// Allocate, without initialization, an `Image` resource handle.
    ///
    /// The image must subsequently be initialized with [`init_image()`].
    ///
    /// [`init_image()`]: fn.init_image.html
    pub fn alloc_image(&mut self) -> Image {
        unimplemented!();
    }

    /// Allocate, without initialization, a `Shader` resource handle.
    ///
    /// The shader must subsequently be initialized with [`init_shader()`].
    ///
    /// [`init_shader()`]: fn.init_shader.html
    pub fn alloc_shader(&mut self) -> Shader {
        unimplemented!();
    }

    /// Allocate, without initialization, a `Pipeline` resource handle.
    ///
    /// The pipeline must subsequently be initialized with [`init_pipeline()`].
    ///
    /// [`init_pipeline()`]: fn.init_pipeline.html
    pub fn alloc_pipeline(&mut self) -> Pipeline {
        unimplemented!();
    }

    /// Allocate, without initialization, a `Pass` resource handle.
    ///
    /// The pass must subsequently be initialized with [`init_pass()`].
    ///
    /// [`init_pass()`]: fn.init_pass.html
    pub fn alloc_pass(&mut self) -> Pass {
        unimplemented!();
    }

    /// Initialize an allocated `Buffer` resource handle.
    pub fn init_buffer(&mut self, buf_id: Buffer, desc: BufferDesc) {
        unimplemented!();
    }

    /// Initialize an allocated `Image` resource handle.
    pub fn init_image(&mut self, img_id: Image, desc: ImageDesc) {
        unimplemented!();
    }

    /// Initialize an allocated `Shader` resource handle.
    pub fn init_shader(&mut self, shd_id: Shader, desc: ShaderDesc) {
        unimplemented!();
    }

    /// Initialize an allocated `Pipeline` resource handle.
    pub fn init_pipeline(&mut self, pip_id: Pipeline, desc: PipelineDesc) {
        unimplemented!();
    }

    /// Initialize an allocated `Pass` resource handle.
    pub fn init_pass(&mut self, pass_id: Pass, desc: PassDesc) {
        unimplemented!();
    }

    /// Helper function for creating a `VertexAttrDesc` with a name.
    pub fn named_attr(
        &mut self,
        name: &'static str,
        offset: u32,
        format: VertexFormat,
    ) -> VertexAttrDesc {
        unimplemented!();
    }

    /// Helper function for creating a `VertexAttrDesc` using a semantic name and index.
    pub fn sem_attr(
        &mut self,
        sem_name: &'static str,
        sem_index: u32,
        offset: u32,
        format: VertexFormat,
    ) -> VertexAttrDesc {
        unimplemented!();
    }

    /// Helper function for creating a `ShaderUniformDesc`.
    pub fn named_uniform(
        &mut self,
        name: &'static str,
        uniform_type: UniformType,
        array_count: u32,
    ) -> ShaderUniformDesc {
        unimplemented!();
    }

    /// Helper function for creating a `ShaderImageDesc`.
    pub fn named_image(&mut self, name: &'static str, image_type: ImageType) -> ShaderImageDesc {
        unimplemented!();
    }
}

impl Drop for Context {
    /// Shutdown the Grafiska library at the end of your program.
    fn drop(&mut self) {
        self.shutdown()
    }
}

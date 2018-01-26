// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::metal_sys::*;
use super::super::*;

impl Action {
    /// Convert this action to the Metal equivalent `MTLLoadAction`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_load_action(self) -> MTLLoadAction {
        match self {
            Action::Clear => MTLLoadAction::Clear,
            Action::Load => MTLLoadAction::Load,
            Action::DontCare => MTLLoadAction::Clear,
        }
    }
}

impl BlendFactor {
    /// Convert this blend factor to the Metal equivalent `MTLBlendFactor`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_blend_factor(self) -> MTLBlendFactor {
        match self {
            BlendFactor::Zero => MTLBlendFactor::Zero,
            BlendFactor::One => MTLBlendFactor::One,
            BlendFactor::SrcColor => MTLBlendFactor::SourceColor,
            BlendFactor::OneMinusSrcColor => MTLBlendFactor::OneMinusSourceColor,
            BlendFactor::SrcAlpha => MTLBlendFactor::SourceAlpha,
            BlendFactor::OneMinusSrcAlpha => MTLBlendFactor::OneMinusSourceAlpha,
            BlendFactor::DstColor => MTLBlendFactor::DestinationColor,
            BlendFactor::OneMinusDstColor => MTLBlendFactor::OneMinusDestinationColor,
            BlendFactor::DstAlpha => MTLBlendFactor::DestinationAlpha,
            BlendFactor::OneMinusDstAlpha => MTLBlendFactor::OneMinusDestinationAlpha,
            BlendFactor::SrcAlphaSaturated => MTLBlendFactor::SourceAlphaSaturated,
            BlendFactor::BlendColor => MTLBlendFactor::BlendColor,
            BlendFactor::OneMinusBlendColor => MTLBlendFactor::OneMinusBlendColor,
            BlendFactor::BlendAlpha => MTLBlendFactor::BlendAlpha,
            BlendFactor::OneMinusBlendAlpha => MTLBlendFactor::OneMinusBlendAlpha,
        }
    }
}

impl BlendOp {
    /// Convert this blend operation to the Metal equivalent `MTLBlendOperation`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_blend_op(self) -> MTLBlendOperation {
        match self {
            BlendOp::Add => MTLBlendOperation::Add,
            BlendOp::Subtract => MTLBlendOperation::Subtract,
            BlendOp::ReverseSubtract => MTLBlendOperation::ReverseSubtract,
        }
    }
}

impl ColorMask {
    /// Convert this color mask to the Metal equivalent `MTLColorWriteMask`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_color_write_mask(self) -> MTLColorWriteMask {
        let mut m = MTLColorWriteMask::MTLColorWriteMaskNone;
        if self.contains(ColorMask::R) {
            m |= MTLColorWriteMask::MTLColorWriteMaskRed;
        }
        if self.contains(ColorMask::G) {
            m |= MTLColorWriteMask::MTLColorWriteMaskGreen;
        }
        if self.contains(ColorMask::B) {
            m |= MTLColorWriteMask::MTLColorWriteMaskBlue;
        }
        if self.contains(ColorMask::A) {
            m |= MTLColorWriteMask::MTLColorWriteMaskAlpha;
        }
        m
    }
}

impl CompareFunc {
    /// Convert this comparison function to the Metal equivalent `MTLCompareFunction`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_compare_func(self) -> MTLCompareFunction {
        match self {
            CompareFunc::Never => MTLCompareFunction::Never,
            CompareFunc::Less => MTLCompareFunction::Less,
            CompareFunc::Equal => MTLCompareFunction::Equal,
            CompareFunc::LessEqual => MTLCompareFunction::LessEqual,
            CompareFunc::Greater => MTLCompareFunction::Greater,
            CompareFunc::NotEqual => MTLCompareFunction::NotEqual,
            CompareFunc::GreaterEqual => MTLCompareFunction::GreaterEqual,
            CompareFunc::Always => MTLCompareFunction::Always,
        }
    }
}

impl CullMode {
    /// Convert this cull mode to the Metal equivalent `MTLCullMode`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_cull_mode(self) -> MTLCullMode {
        match self {
            CullMode::None => MTLCullMode::None,
            CullMode::Front => MTLCullMode::Front,
            CullMode::Back => MTLCullMode::Back,
        }
    }
}

impl FaceWinding {
    /// Convert this face winding to the Metal equivalent `MTLWinding`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_winding(self) -> MTLWinding {
        match self {
            FaceWinding::CW => MTLWinding::Clockwise,
            FaceWinding::CCW => MTLWinding::CounterClockwise,
        }
    }
}

impl Filter {
    /// Convert this filter to the Metal equivalent `MTLSamplerMinMagFilter`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_minmag_filter(self) -> MTLSamplerMinMagFilter {
        match self {
            Filter::Nearest | Filter::NearestMipmapNearest | Filter::NearestMipmapLinear => {
                MTLSamplerMinMagFilter::Nearest
            }
            Filter::Linear | Filter::LinearMipmapNearest | Filter::LinearMipmapLinear => {
                MTLSamplerMinMagFilter::Linear
            }
        }
    }

    /// Convert this filter to the Metal equivalent `MTLSamplerMinFilter`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_mip_filter(self) -> MTLSamplerMipFilter {
        match self {
            Filter::Nearest | Filter::Linear => MTLSamplerMipFilter::NotMipmapped,
            Filter::NearestMipmapNearest | Filter::LinearMipmapNearest => {
                MTLSamplerMipFilter::Nearest
            }
            Filter::NearestMipmapLinear | Filter::LinearMipmapLinear => MTLSamplerMipFilter::Linear,
        }
    }
}

impl ImageType {
    /// Convert this image type to the Metal equivalent `MTLTextureType`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_texture_type(self) -> MTLTextureType {
        match self {
            ImageType::Texture2D => MTLTextureType::D2,
            ImageType::Cube => MTLTextureType::Cube,
            ImageType::Texture3D => MTLTextureType::D3,
            ImageType::Array => MTLTextureType::D2Array,
        }
    }
}

impl IndexType {
    /// Get the size in bytes of an element of this index type.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_index_size(self) -> usize {
        match self {
            IndexType::None => 0,
            IndexType::UInt16 => 2,
            IndexType::UInt32 => 4,
        }
    }

    /// Convert this index type to the Metal equivalent `MTLIndexType`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_index_type(self) -> MTLIndexType {
        match self {
            IndexType::None => unreachable!(),
            IndexType::UInt16 => MTLIndexType::UInt16,
            IndexType::UInt32 => MTLIndexType::UInt32,
        }
    }
}

impl PixelFormat {
    /// Return `true` if this pixel format represents PVR texture compression.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_is_pvrtc(self) -> bool {
        match self {
            PixelFormat::PVRTC2_RGB
            | PixelFormat::PVRTC2_RGBA
            | PixelFormat::PVRTC4_RGB
            | PixelFormat::PVRTC4_RGBA => true,
            _ => false,
        }
    }

    /// Convert this pixel format to the Metal equivalent `MTLPixelFormat`.
    /// for a texture format.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_texture_format(self) -> MTLPixelFormat {
        match self {
            PixelFormat::RGBA8 => MTLPixelFormat::RGBA8Unorm,
            PixelFormat::R10G10B10A2 => MTLPixelFormat::RGB10A2Unorm,
            PixelFormat::RGBA32F => MTLPixelFormat::RGBA32Float,
            PixelFormat::RGBA16F => MTLPixelFormat::RGBA16Float,
            PixelFormat::R32F => MTLPixelFormat::R32Float,
            PixelFormat::R16F => MTLPixelFormat::R16Float,
            PixelFormat::L8 => MTLPixelFormat::R8Unorm,
            #[cfg(feature = "metal_macos")]
            PixelFormat::DXT1 => MTLPixelFormat::BC1_RGBA,
            #[cfg(feature = "metal_macos")]
            PixelFormat::DXT3 => MTLPixelFormat::BC2_RGBA,
            #[cfg(feature = "metal_macos")]
            PixelFormat::DXT5 => MTLPixelFormat::BC3_RGBA,
            #[cfg(feature = "metal_ios")]
            PixelFormat::PVRTC2_RGB => MTLPixelFormat::PVRTC_RGB_2BPP,
            #[cfg(feature = "metal_ios")]
            PixelFormat::PVRTC4_RGB => MTLPixelFormat::PVRTC_RGB_4BPP,
            #[cfg(feature = "metal_ios")]
            PixelFormat::PVRTC2_RGBA => MTLPixelFormat::PVRTC_RGBA_2BPP,
            #[cfg(feature = "metal_ios")]
            PixelFormat::PVRTC4_RGBA => MTLPixelFormat::PVRTC_RGBA_4BPP,
            #[cfg(feature = "metal_ios")]
            PixelFormat::ETC2_RGB8 => MTLPixelFormat::ETC2_RGB8,
            #[cfg(feature = "metal_ios")]
            PixelFormat::ETC2_SRGB8 => MTLPixelFormat::ETC2_RGB8_sRGB,
            _ => MTLPixelFormat::Invalid,
        }
    }

    /// Convert this pixel format to the Metal equivalent `MTLPixelFormat`
    /// for the render target color format.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_rendertarget_color_format(self) -> MTLPixelFormat {
        match self {
            PixelFormat::RGBA8 => MTLPixelFormat::BGRA8Unorm, // Not a bug!
            PixelFormat::RGBA32F => MTLPixelFormat::RGBA32Float,
            PixelFormat::RGBA16F => MTLPixelFormat::RGBA16Float,
            PixelFormat::R10G10B10A2 => MTLPixelFormat::RGB10A2Unorm,
            _ => MTLPixelFormat::Invalid,
        }
    }

    /// Convert this pixel format to the Metal equivalent `MTLPixelFormat`
    /// for the render target depth format.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_rendertarget_depth_format(self) -> MTLPixelFormat {
        match self {
            PixelFormat::Depth => MTLPixelFormat::Depth32Float,
            PixelFormat::DepthStencil => {
                // Note: Depth24_Stencil8 isn't universally supported!
                MTLPixelFormat::Depth32Float_Stencil8
            }
            _ => MTLPixelFormat::Invalid,
        }
    }

    /// Convert this pixel format to the Metal equivalent `MTLPixelFormat`
    /// for the render target stencil format.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_rendertarget_stencil_format(self) -> MTLPixelFormat {
        match self {
            PixelFormat::DepthStencil => MTLPixelFormat::Depth32Float_Stencil8,
            _ => MTLPixelFormat::Invalid,
        }
    }
}

impl PrimitiveType {
    /// Convert this primitive type to the Metal equivalent `MTLPrimitiveType`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_primitive_type(self) -> MTLPrimitiveType {
        match self {
            PrimitiveType::Points => MTLPrimitiveType::Point,
            PrimitiveType::Lines => MTLPrimitiveType::Line,
            PrimitiveType::LineStrip => MTLPrimitiveType::LineStrip,
            PrimitiveType::Triangles => MTLPrimitiveType::Triangle,
            PrimitiveType::TriangleStrip => MTLPrimitiveType::TriangleStrip,
        }
    }
}

impl Usage {
    /// Convert this usage to the Metal equivalent `MTLResourceOptions`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_buffer_resource_options(self) -> MTLResourceOptions {
        match self {
            Usage::Immutable => MTLResourceOptions::StorageModeShared,
            Usage::Dynamic | Usage::Stream => {
                if cfg!(feature = "metal_macos") {
                    MTLResourceOptions::CPUCacheModeWriteCombined
                        | MTLResourceOptions::StorageModeManaged
                } else {
                    MTLResourceOptions::CPUCacheModeWriteCombined
                }
            }
        }
    }
}

impl VertexFormat {
    /// Convert this vertex format to the Metal equivalent `MTLVertexFormat`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_vertex_format(self) -> MTLVertexFormat {
        match self {
            VertexFormat::Float => MTLVertexFormat::Float,
            VertexFormat::Float2 => MTLVertexFormat::Float2,
            VertexFormat::Float3 => MTLVertexFormat::Float3,
            VertexFormat::Float4 => MTLVertexFormat::Float4,
            VertexFormat::Byte4 => MTLVertexFormat::Char4,
            VertexFormat::Byte4N => MTLVertexFormat::Char4Normalized,
            VertexFormat::UByte4 => MTLVertexFormat::UChar4,
            VertexFormat::UByte4N => MTLVertexFormat::UChar4Normalized,
            VertexFormat::Short2 => MTLVertexFormat::Short2,
            VertexFormat::Short2N => MTLVertexFormat::Short2Normalized,
            VertexFormat::Short4 => MTLVertexFormat::Short4,
            VertexFormat::Short4N => MTLVertexFormat::Short4Normalized,
            VertexFormat::UInt10N2 => MTLVertexFormat::UInt1010102Normalized,
        }
    }
}

impl VertexStep {
    /// Convert this vertex step function to the Metal equivalent `MTLVertexStepFunction`.
    ///
    /// This is only present when the `metal_macos` or `metal_ios` feature
    /// is enabled.
    pub fn mtl_step_function(self) -> MTLVertexStepFunction {
        match self {
            VertexStep::PerVertex => MTLVertexStepFunction::PerVertex,
            VertexStep::PerInstance => MTLVertexStepFunction::PerInstance,
        }
    }
}

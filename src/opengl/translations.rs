// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::gleam::gl;
use super::super::*;

impl BufferType {
    /// Convert this buffer type to the OpenGL equivalent.
    ///
    /// This is only present when the `gl` feature is enabled.
    pub fn gl_buffer_target(self) -> gl::GLenum {
        match self {
            BufferType::VertexBuffer => gl::ARRAY_BUFFER,
            BufferType::IndexBuffer => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

impl ImageType {
    /// Convert this image type to the OpenGL equivalent.
    ///
    /// This is only present when the `gl` feature is enabled.
    pub fn gl_texture_target(self) -> gl::GLenum {
        match self {
            ImageType::Texture2D => gl::TEXTURE_2D,
            ImageType::Cube => gl::TEXTURE_CUBE_MAP,
            #[cfg(not(feature = "gles2"))]
            ImageType::Texture3D => gl::TEXTURE_3D,
            #[cfg(not(feature = "gles2"))]
            ImageType::Array => gl::TEXTURE_2D_ARRAY,
            #[cfg(feature = "gles2")]
            _ => unreachable!(),
        }
    }
}

impl ShaderStage {
    /// Convert this shader stage to the OpenGL equivalent.
    ///
    /// This is only present when the `gl` feature is enabled.
    pub fn gl_shader_stage(self) -> gl::GLenum {
        match self {
            ShaderStage::VS => gl::VERTEX_SHADER,
            ShaderStage::FS => gl::FRAGMENT_SHADER,
        }
    }
}

impl Usage {
    /// Convert this usage flag to the OpenGL equivalent.
    ///
    /// This is only present when the `gl` feature is enabled.
    pub fn gl_usage(self) -> gl::GLenum {
        match self {
            Usage::Immutable => gl::STATIC_DRAW,
            Usage::Dynamic => gl::DYNAMIC_DRAW,
            Usage::Stream => gl::STREAM_DRAW,
        }
    }
}

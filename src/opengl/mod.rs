// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gleam;

mod translations;
pub use self::translations::*;

use opengl::gleam::gl::types::{GLint, GLuint, GLenum};

/// GL backend buffer resource.
pub struct Buffer {
    slot: ::pool::Slot,
    size: usize,
    buffer_type: ::BufferType,  // Renamed from sokol field 'type' because type is a keyword.
    usage: ::Usage,
    upd_frame_index: u32,
    // num_slots: usize,
    active_slot: usize,
    gl_buf: Vec<GLuint>,
    ext_buffers: bool,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
            slot: ::pool::Slot::default(),
            size: 0,
            buffer_type: ::BufferType::default(),  // Renamed from sokol field 'type' because type is a keyword.
            usage: ::Usage::default(),
            upd_frame_index: 0,
            active_slot: 0,
            gl_buf: Vec::<GLuint>::with_capacity(::NUM_INFLIGHT_FRAMES),
            ext_buffers: false,
        }
    }
}

/// GL backend image resource
pub struct Image {
    slot: ::pool::Slot,
    image_type: ::ImageType,
    render_target: bool,
    width: usize,
    height: usize,
    depth: usize,
    num_mipmaps: usize,
    usage: ::Usage,
    pixel_format: ::PixelFormat,
    sample_count: usize,
    min_filter: ::Filter,
    mag_filter: ::Filter,
    wrap_u: ::Wrap,
    wrap_v: ::Wrap,
    wrap_w: ::Wrap,
    max_anisotropy: u32,  // TODO: Or usize?
    gl_target: GLenum,
    gl_depth_render_buffer: GLuint,
    gl_msaa_render_buffer: GLuint,
    upd_frame_index: u32,
    num_slots: usize,
    active_slot: usize,
    gl_tex: Vec<GLuint>,
    ext_textures: bool,
}

impl Default for Image {
    fn default() -> Self {
        let mut gl_tex = Vec::<GLuint>::with_capacity(::NUM_INFLIGHT_FRAMES);
        gl_tex.resize(::NUM_INFLIGHT_FRAMES, 0);
        Image {
            slot: ::pool::Slot::default(),
            image_type: ::ImageType::default(),
            render_target: false,
            width: 0,
            height: 0,
            depth: 0,
            num_mipmaps: 0,
            usage: ::Usage::default(),
            pixel_format: ::PixelFormat::default(),
            sample_count: 0,
            min_filter: ::Filter::default(),
            mag_filter: ::Filter::default(),
            wrap_u: ::Wrap::default(),
            wrap_v: ::Wrap::default(),
            wrap_w: ::Wrap::default(),
            max_anisotropy: 0,
            gl_target: 0,
            gl_depth_render_buffer: 0,
            gl_msaa_render_buffer: 0,
            upd_frame_index: 0,
            num_slots: 0,
            active_slot: 0,
            gl_tex: gl_tex,
            ext_textures: false,
        }
    }
}

struct Uniform {
    gl_loc: GLint,
    uniform_type: ::UniformType,
    count: u8,
    offset: u16,
}

impl Default for Uniform {
    fn default() -> Self {
        Uniform {
            gl_loc: 0,
            uniform_type: ::UniformType::Float,
            count: 0,
            offset: 0,
        }
    }
}

struct UniformBlock {
    uniforms: Vec<Uniform>,
}

impl Default for UniformBlock {
    fn default() -> Self {
        UniformBlock {
            uniforms: Vec::<Uniform>::with_capacity(::MAX_UB_MEMBERS)
        }
    }
}

pub struct ShaderStage {
    uniform_blocks: Vec<UniformBlock>,
    images: Vec<Image>,
}

impl Default for ShaderStage {
    fn default() -> Self {
        ShaderStage {
            uniform_blocks: Vec::<UniformBlock>::with_capacity(::MAX_SHADERSTAGE_UBS),
            images: Vec::<Image>::with_capacity(::MAX_SHADERSTAGE_IMAGES),
        }
    }
}

pub struct Shader {
    slot: ::pool::Slot,
    gl_prog: GLuint,
    stage: Vec<ShaderStage>,
}

impl Default for Shader {
    fn default() -> Self {
        let mut stage = Vec::<ShaderStage>::with_capacity(::NUM_INFLIGHT_FRAMES);
        for i in 0..::NUM_INFLIGHT_FRAMES {
            stage.push(ShaderStage::default());
        }
        Shader {
            slot: ::pool::Slot::default(),
            gl_prog: 0,
            stage: stage,
        }
    }
}

struct GlAttr {
    vb_index: i8,  // -1 if attr is not enabled
    divisor: i8,  // -1 if not initialized
    stride: i8,
    size: i8,
    normalized: i8,
    offset: u8,
    attr_type: GLenum,
}

impl Default for GlAttr {
    fn default() -> Self {
        GlAttr {
            vb_index: -1,
            divisor: -1,
            stride: 0,
            size: 0,
            normalized: 0,
            offset: 0,
            attr_type: 0,
        }
    }
}

pub struct Pipeline {
    slot: ::pool::Slot,
    shader: Shader, // TODO why was this a pointer?
    shader_id: ::Shader,
    primitive_type: ::PrimitiveType,
    index_type: ::IndexType,
    vertex_layout_valid: Vec<bool>,
    color_attachment_count: usize,
    color_format: ::PixelFormat,
    depth_format: ::PixelFormat,
    sample_count: usize,
    gl_attrs: Vec<GlAttr>,
    depth_stencil: ::DepthStencilState,
    blend: ::BlendState,
    rast: ::RasterizerState,
}

impl Default for Pipeline {
    fn default() -> Self {
        let mut stage = Vec::<GLuint>::with_capacity(::NUM_INFLIGHT_FRAMES);
        stage.resize(::NUM_SHADER_STAGES, 0);
        Pipeline {
            slot: ::pool::Slot::default(),
            shader: Shader::default(), // TODO why was this a pointer?
            shader_id: ::Shader { id: 0 },
            primitive_type: ::PrimitiveType::default(),
            index_type: ::IndexType::UInt16,
            vertex_layout_valid: Vec::with_capacity(::MAX_SHADERSTAGE_BUFFERS),
            color_attachment_count: 0,
            color_format: ::PixelFormat::default(),
            depth_format: ::PixelFormat::default(),
            sample_count: 0,
            gl_attrs: Vec::with_capacity(::MAX_VERTEX_ATTRIBUTES),
            depth_stencil: ::DepthStencilState::default(),
            blend: ::BlendState::default(),
            rast: ::RasterizerState::default(),
        }
    }
}

#[derive(Default)]
pub struct Attachment {
    image: Image, // TODO why was this a pointer
    image_id: ::Image,
    mip_level: usize, // TODO was an int, does this need to be signed?
    slice: usize, // TODO was an int, does this need to be signed?
    gl_msaa_resolve_buffer: GLuint,
}

pub struct Pass {
    slot: ::pool::Slot,
    gl_fb: GLuint,
    color_atts: Vec<Attachment>,
    ds_att: Attachment
}

impl Default for Pass {
    fn default() -> Self {
        Pass {
            slot: ::pool::Slot::default(),
            gl_fb: 0,
            color_atts: Vec::<Attachment>::with_capacity(::MAX_COLOR_ATTACHMENTS),
            ds_att: Attachment::default(),
        }
    }
}

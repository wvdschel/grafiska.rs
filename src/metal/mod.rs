// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate metal_rs as metal_sys;
use self::metal_sys::*;

mod backend;
pub use self::backend::Backend;

mod translations;
pub use self::translations::*;

#[derive(Debug, Default)]
pub struct Buffer {
    slot: ::pool::Slot,
    size: usize,
    buffer_type: ::BufferType, // Renamed from sokol field 'type' because type is a keyword.
    usage: ::Usage,
    upd_frame_index: u32,
    num_slots: usize,
    active_slot: usize,
    mtl_buf: [u32; ::NUM_INFLIGHT_FRAMES],
}

#[derive(Debug, Default)]
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
    max_anisotropy: u32, // TODO: Or usize?
    upd_frame_index: u32,
    num_slots: usize,
    active_slot: usize,
    mtl_tex: [u32; ::NUM_INFLIGHT_FRAMES],
    mtl_depth_tex: u32,
    mtl_msaa_tex: u32,
    mtl_sampler_state: u32,
}

#[derive(Debug, Default)]
pub struct UniformBlock {
    size: usize,
}

#[derive(Debug, Default)]
pub struct ShaderImage {
    image_type: ::ImageType,
}

#[derive(Debug, Default)]
pub struct ShaderStage {
    num_uniform_blocks: usize,
    num_images: usize,
    uniform_blocks: [UniformBlock; ::MAX_SHADERSTAGE_UBS],
    images: [ShaderImage; ::MAX_SHADERSTAGE_IMAGES],
    mtl_lib: u32,
    mt_func: u32,
}

#[derive(Debug, Default)]
pub struct Shader {
    slot: ::pool::Slot,
    stage: [ShaderStage; ::NUM_SHADER_STAGES],
}

#[derive(Debug)]
pub struct Pipeline {
    slot: ::pool::Slot,
    shader: Shader, // TODO why was this a pointer?
    shader_id: ::Shader,
    vertex_layout_valid: [bool; ::MAX_SHADERSTAGE_BUFFERS],
    color_attachment_count: usize,
    color_format: ::PixelFormat,
    depth_format: ::PixelFormat,
    sample_count: usize,
    depth_bias: f32,
    depth_bias_slope_scale: f32,
    depth_bias_clamp: f32,
    mtl_prim_type: MTLPrimitiveType,
    index_type: ::IndexType,
    mtl_index_size: u32,
    mtl_index_type: MTLIndexType,
    mtl_cull_mode: MTLCullMode,
    mtl_winding: MTLWinding,
    blend_color: [f32; 4],
    mtl_stencil_ref: u32,
    mtl_rps: u32,
    mtl_dss: u32,
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            slot: ::pool::Slot::default(),
            shader: Shader::default(),
            shader_id: ::Shader::default(),
            vertex_layout_valid: Default::default(),
            color_attachment_count: 0,
            color_format: ::PixelFormat::default(),
            depth_format: ::PixelFormat::default(),
            sample_count: 0,
            depth_bias: 0.0f32,
            depth_bias_slope_scale: 0.0f32,
            depth_bias_clamp: 0.0f32,
            mtl_prim_type: MTLPrimitiveType::Point,
            index_type: ::IndexType::UInt16,
            mtl_index_size: 0,
            mtl_index_type: MTLIndexType::UInt16,
            mtl_cull_mode: MTLCullMode::None,
            mtl_winding: MTLWinding::Clockwise,
            blend_color: Default::default(),
            mtl_stencil_ref: 0,
            mtl_rps: 0,
            mtl_dss: 0,
        }
    }
}

#[derive(Debug, Default)]
pub struct Attachment {
    image: Image,
    image_id: ::Image,
    mip_level: u32,
    slice: u32,
}

#[derive(Debug, Default)]
pub struct Pass {
    slot: ::pool::Slot,
    num_color_atts: u32,
    color_atts: [Attachment; ::MAX_COLOR_ATTACHMENTS],
    ds_att: Attachment,
}

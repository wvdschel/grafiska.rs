// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use opengl::gleam::gl::types::{GLenum, GLint, GLuint};
use opengl::*;
use std::collections::HashSet;
use std::os;

use {Config, Feature, ShaderStage};

#[derive(Default)]
pub struct Backend {
    in_pass: bool,
    default_framebuffer: GLuint,
    cur_pass_width: usize,
    cur_pass_height: usize,
    curr_pass: PassResource, // TODO why was this a pointer?
    cur_pass_id: ::Pass,
    cache: ContextCache,
    features: HashSet<::Feature>,
    ext_anisotropic: bool,
    max_anisotropy: GLint,
    #[cfg(not(feature = "gles2"))]
    vao: GLuint,
}

impl Backend {
    pub fn new(_desc: Config) -> Self {
        Backend::default()
    }

    pub fn query_feature(&self, feature: Feature) -> bool {
        unimplemented!()
    }

    pub fn reset_state_cache(&mut self) {
        unimplemented!()
    }

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

    pub fn apply_uniform_block(
        &mut self,
        stage: ShaderStage,
        ub_index: u32,
        data: *const os::raw::c_void,
        num_bytes: u32,
    ) {
        unimplemented!();
    }

    pub fn draw(&mut self, base_element: u32, num_elements: u32, num_instances: u32) {
        unimplemented!();
    }

    pub fn end_pass(&mut self) {
        unimplemented!();
    }

    pub fn commit(&mut self) {
        unimplemented!();
    }
}

struct CacheAttribute {
    gl_attr: GlAttr,
    gl_vbuf: GLuint,
}

struct ContextCache {
    ds: ::DepthStencilState,
    blend: ::BlendState,
    rast: ::RasterizerState,
    polygon_offset_enabled: bool,
    attrs: Vec<CacheAttribute>,
    cur_gl_ib: GLuint,
    cur_primitive_type: GLenum,
    cur_index_type: GLenum,
    cur_pipeline: PipelineResource, // TODO why was this a pointer?
    cur_pipeline_id: ::Pipeline,
}

impl Default for ContextCache {
    fn default() -> Self {
        ContextCache {
            ds: ::DepthStencilState::default(),
            blend: ::BlendState::default(),
            rast: ::RasterizerState::default(),
            polygon_offset_enabled: false,
            attrs: Vec::with_capacity(::MAX_VERTEX_ATTRIBUTES),
            cur_gl_ib: 0,
            cur_primitive_type: 0,
            cur_index_type: 0,
            cur_pipeline: PipelineResource::default(),
            cur_pipeline_id: ::Pipeline::default(),
        }
    }
}

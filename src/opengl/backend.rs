use opengl::gleam::gl::types::{GLint, GLuint, GLenum};
use opengl::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct Backend {
    in_pass: bool,
    default_framebuffer: GLuint,
    cur_pass_width: usize,
    cur_pass_height: usize,
    curr_pass: Pass, // TODO why was this a pointer?
    cur_pass_id: ::Pass,
    cache: ContextCache,
    features: HashSet<::Feature>,
    ext_anisotropic: bool,
    max_anisotropy: GLint,
    #[cfg(not(feature = "gles2"))]
    vao: GLuint,
}

impl Backend {
    pub fn query_feature(&self, feature: ::Feature) -> bool {
        unimplemented!()
    }

    pub fn reset_state_cache(&mut self) {
        unimplemented!()
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
    cur_pipeline: Pipeline, // TODO why was this a pointer?
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
            cur_pipeline: Pipeline::default(),
            cur_pipeline_id: ::Pipeline::default(),
        }
    }
}

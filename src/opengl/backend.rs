// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use opengl::gleam::gl::types::{GLenum, GLint, GLuint};
use opengl::gleam::gl::{self, Gl};
use opengl::*;
use std::collections::HashSet;
use std::os;

use {Config, Feature, ShaderStage};

const GL_TEXTURE_MAX_ANISOTROPY_EXT: GLuint = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: GLuint = 0x84FF;

pub struct Backend {
    in_pass: bool,
    force_gles2: bool,
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
    gl: std::rc::Rc<Gl>,
}

impl Backend {
    #[allow(unsafe_code)]
    pub fn new(desc: Config) -> Self {
        #[cfg(any(feature = "gles2", feature = "gles3"))]
        let gl = unsafe { gl::GlesFns::load_with(|symbol| desc.load_gl_symbol.lookup(symbol)) };
        #[cfg(not(any(feature = "gles2", feature = "gles3")))]
        let gl = unsafe {
            if desc.gl_force_gles2 {
                gl::GlesFns::load_with(|symbol| desc.load_gl_symbol.lookup(symbol))
            } else {
                gl::GlFns::load_with(|symbol| desc.load_gl_symbol.lookup(symbol))
            }
        };

        let mut res = Backend {
            in_pass: false,
            force_gles2: desc.gl_force_gles2,
            default_framebuffer: gl.get_integer_v(gl::FRAMEBUFFER_BINDING) as GLuint,
            cur_pass_width: 0,
            cur_pass_height: 0,
            curr_pass: PassResource::default(),
            cur_pass_id: ::Pass::default(),
            cache: ContextCache::default(),
            features: HashSet::<::Feature>::new(),
            ext_anisotropic: false,
            max_anisotropy: 0,
            #[cfg(not(feature = "gles2"))]
            vao: gl::INVALID_VALUE,
            gl: gl,
        };

        res.reset_state_cache();
        res.init_gl_features();

        res
    }

    /* Private helper methods */

    #[cfg(feature = "gles2")]
    fn init_gl_features(&mut self) {
        self.features.insert(Feature::OriginBottomLeft);

        let extensions = self.gl.get_string(gl::EXTENSIONS);
        for extension in extensions.split_whitespace() {
            match extension {
                "_instanced_arrays" => {
                    self.features.insert(Feature::Instancing);
                }
                "_texture_float" => {
                    self.features.insert(Feature::TextureFloat);
                }
                "_texture_half_float" => {
                    self.features.insert(Feature::TextureHalfFloat);
                }
                "_texture_filter_anisotropic" => {
                    self.ext_anisotropic = true;
                }
                "_texture_compression_s3tc"
                | "_compressed_texture_s3tc"
                | "texture_compression_dxt1" => {
                    self.features.insert(Feature::TextureCompressionDXT);
                }
                "_texture_compression_pvrtc" | "_compressed_texture_pvrtc" => {
                    self.features.insert(Feature::TextureCompressionPVRTC);
                }
                "_compressed_texture_atc" => {
                    self.features.insert(Feature::TextureCompressionATC);
                }
                &_ => {}
            }
        }

        self.max_anisotropy = 1;
        if self.ext_anisotropic {
            self.max_anisotropy = self.gl.get_integer_v(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT);
        }
    }

    #[cfg(feature = "gles3")]
    fn init_gl_features(&mut self) {
        self.features.insert(Feature::OriginBottomLeft);
        self.features.insert(Feature::Instancing);
        self.features.insert(Feature::TextureHalfFloat);
        self.features.insert(Feature::TextureFloat);
        self.features.insert(Feature::MSAARenderTargets);
        self.features.insert(Feature::PackedVertexFormat_10_2);
        self.features.insert(Feature::MultipleRenderTarget);
        self.features.insert(Feature::ImageType3D);
        self.features.insert(Feature::ImageTypeArray);

        let extensions = self.gl.get_string(gl::EXTENSIONS);
        for extension in extensions.split_whitespace() {
            match extension {
                "_texture_filter_anisotropic" => {
                    self.ext_anisotropic = true;
                }
                "_texture_compression_s3tc"
                | "_compressed_texture_s3tc"
                | "texture_compression_dxt1" => {
                    self.features.insert(Feature::TextureCompressionDXT);
                }
                "_texture_compression_pvrtc" | "_compressed_texture_pvrtc" => {
                    self.features.insert(Feature::TextureCompressionPVRTC);
                }
                "_compressed_texture_atc" => {
                    self.features.insert(Feature::TextureCompressionATC);
                }
                &_ => {}
            }
        }

        self.max_anisotropy = 1;
        if self.ext_anisotropic {
            self.max_anisotropy = self.gl.get_integer_v(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT);
        }
    }

    #[cfg(feature = "glcore33")]
    fn init_gl_features(&mut self) {
        self.features.insert(Feature::OriginBottomLeft);
        self.features.insert(Feature::Instancing);
        self.features.insert(Feature::TextureHalfFloat);
        self.features.insert(Feature::TextureFloat);
        self.features.insert(Feature::MSAARenderTargets);
        self.features.insert(Feature::PackedVertexFormat_10_2);
        self.features.insert(Feature::MultipleRenderTarget);
        self.features.insert(Feature::ImageType3D);
        self.features.insert(Feature::ImageTypeArray);

        let num_ext = self.gl.get_integer_v(gl::NUM_EXTENSIONS);
        for i in 0..num_ext {
            let extension = self.gl.get_string_i(gl::EXTENSIONS, i as GLuint);
            if extension == "_texture_compression_s3tc" {
                // TODO
                self.features.insert(Feature::TextureCompressionDXT);
            } else if extension == "_texture_filter_anisotropic" {
                self.ext_anisotropic = true; // TODO make this a feature?
            }
        }

        self.max_anisotropy = 1;
        if self.ext_anisotropic {
            self.max_anisotropy = self.gl.get_integer_v(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT);
        }
    }

    #[cfg(not(feature = "gles2"))]
    fn reset_vao(&mut self) {
        if !self.force_gles2 {
            if self.vao == gl::INVALID_VALUE {
                let vertex_arrays = self.gl.gen_vertex_arrays(1);
                self.vao = vertex_arrays[0];
            }
            self.gl.bind_vertex_array(self.vao);
        }
    }

    #[cfg(feature = "gles2")]
    fn reset_vao(&mut self) {}

    /* Public interface methods */

    pub fn query_feature(&self, feature: Feature) -> bool {
        unimplemented!()
    }

    pub fn reset_state_cache(&mut self) {
        self.reset_vao();
        self.cache = ContextCache::default();

        self.gl.bind_buffer(gl::ARRAY_BUFFER, 0);
        self.gl.bind_buffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        for i in 0..::MAX_VERTEX_ATTRIBUTES {
            self.gl.disable_vertex_attrib_array(i as u32);
        }

        /* depth-stencil state */
        self.gl.enable(gl::DEPTH_TEST);
        self.gl.depth_func(gl::ALWAYS);
        self.gl.depth_mask(false);
        self.gl.disable(gl::STENCIL_TEST);
        self.gl.stencil_func(gl::ALWAYS, 0, 0);
        self.gl.stencil_op(gl::KEEP, gl::KEEP, gl::KEEP);
        self.gl.stencil_mask(0);

        /* blend state */
        self.gl.disable(gl::BLEND);
        self.gl
            .blend_func_separate(gl::ONE, gl::ZERO, gl::ONE, gl::ZERO);
        self.gl.blend_equation_separate(gl::FUNC_ADD, gl::FUNC_ADD);
        self.gl.color_mask(true, true, true, true);
        self.gl.blend_color(0.0, 0.0, 0.0, 0.0);

        /* rasterizer state */
        self.gl.polygon_offset(0.0, 0.0);
        self.gl.disable(gl::POLYGON_OFFSET_FILL);
        self.gl.disable(gl::CULL_FACE);
        self.gl.front_face(gl::CW);
        self.gl.cull_face(gl::BACK);
        self.gl.enable(gl::SCISSOR_TEST);
        self.gl.disable(gl::SAMPLE_ALPHA_TO_COVERAGE);
        self.gl.enable(gl::DITHER);
        self.gl.disable(gl::POLYGON_OFFSET_FILL);

        if cfg!(feature = "glcore33") {
            self.gl.enable(gl::MULTISAMPLE);
            self.gl.enable(gl::PROGRAM_POINT_SIZE);
        }
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
            cur_primitive_type: gl::TRIANGLES,
            cur_index_type: 0,
            cur_pipeline: PipelineResource::default(),
            cur_pipeline_id: ::Pipeline::default(),
        }
    }
}

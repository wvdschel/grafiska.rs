// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::os;

use {Config, Feature, ShaderStage};

pub struct Backend {}

impl Backend {
    pub fn new(desc: Config) -> Self {
        Backend {}
    }

    pub fn query_feature(&self, feature: Feature) -> bool {
        match feature {
            Feature::Instancing
            | Feature::TextureFloat
            | Feature::OriginTopLeft
            | Feature::MSAARenderTargets
            | Feature::PackedVertexFormat_10_2
            | Feature::MultipleRenderTarget
            | Feature::ImageType3D
            | Feature::ImageTypeArray => true,
            #[cfg(target_os = "macos")]
            Feature::TextureCompressionDXT => true,
            #[cfg(target_os = "ios")]
            Feature::TextureCompressionPVRTC | Feature::TextureCompressionETC2 => true,
            _ => false,
        }
    }

    pub fn reset_state_cache(&mut self) {
        unimplemented!();
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

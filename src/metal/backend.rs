// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Feature;

#[derive(Default)]
pub struct Backend {}

impl Backend {
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

    pub fn reset_state_cache(&mut self) {}
}

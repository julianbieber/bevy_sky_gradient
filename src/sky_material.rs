//! The material the camera-following sky is drawn with: its shader inputs and the render
//! passes it takes part in.

use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

use crate::bind_groups::{StarsBindGroup, SunBindGroup};

/// Draws its mesh on the far plane, so the sky shows only where no geometry was drawn.
///
/// The view direction is the mesh's local vertex position, so the mesh must be centred on
/// its own origin and kept at the camera's translation; `SkyboxMagnetTag` does that for the
/// default sky. The material casts no shadows and writes no depth prepass.
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FullSkyMaterial {
    #[uniform(0)]
    pub sun: crate::bind_groups::SunBindGroup,
    #[uniform(1)]
    pub stars: crate::bind_groups::StarsBindGroup,
    /// auto set. 0 = NO night, 1 = FULL night
    /// a full night cycle will go from 0 -> 1 -> 0
    #[uniform(2)]
    pub night_time_distance: f32,
    /// when in the night time to show the stars
    /// x: when to start showing star
    /// y: when the brightness of star is MAXED out
    /// example: (0.0, 0.1).
    /// 0.0: start showing sky immediately when sunset begins
    /// 0.1: sky brightness is maxed out 10% into the night
    #[uniform(3)]
    pub night_visibility_range: Vec2,

    #[uniform(4)]
    pub feature_stars_enabled: i32,
    #[uniform(5)]
    pub feature_sun_enabled: i32,
    #[uniform(6)]
    pub feature_aurora_enabled: i32,

    #[texture(10, dimension = "3d")]
    #[sampler(11)]
    pub noise3_image: Handle<Image>,
    #[texture(12, dimension = "3d")]
    #[sampler(13)]
    pub voronoi3_image: Handle<Image>,

    #[texture(14, dimension = "2d")]
    #[sampler(15)]
    pub aurora_image: Handle<Image>,
    #[texture(16, dimension = "2d")]
    #[sampler(17)]
    pub gradient_image: Handle<Image>,
}

impl Material for FullSkyMaterial {
    fn vertex_shader() -> ShaderRef {
        crate::assets::SKY_SHADER_HANDLE.into()
    }
    fn fragment_shader() -> ShaderRef {
        crate::assets::SKY_SHADER_HANDLE.into()
    }

    fn enable_prepass() -> bool {
        false
    }
    fn enable_shadows() -> bool {
        false
    }
}

impl Default for FullSkyMaterial {
    fn default() -> Self {
        FullSkyMaterial {
            gradient_image: Handle::default(),
            sun: SunBindGroup {
                sun_dir: Vec3::new(0.0, 0.1, -1.0),
                sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
                sun_strength: 1.5,
                sun_sharpness: 164.0,
            },
            night_time_distance: 0.0,
            night_visibility_range: vec2(0.0, 0.1),
            stars: StarsBindGroup::default(),
            noise3_image: Handle::default(),
            voronoi3_image: Handle::default(),
            aurora_image: Handle::default(),
            feature_stars_enabled: 1,
            feature_sun_enabled: 1,
            feature_aurora_enabled: 1,
        }
    }
}

use bevy::{prelude::*, render::render_resource::ShaderType};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct GradientBindGroup {
    /// cosine palette parameter: base color offset
    pub a: Vec3,
    /// cosine palette parameter: amplitude of color variation
    pub b: Vec3,
    /// cosine palette parameter: frequency of color oscillation
    pub c: Vec3,
    /// cosine palette parameter: phase shift
    pub d: Vec3,
}

impl Default for GradientBindGroup {
    fn default() -> Self {
        // Nice sky blue palette using cosine function: a + b*cos(2*pi*(c*t+d))
        GradientBindGroup {
            a: Vec3::new(0.5, 0.6, 0.8),
            b: Vec3::new(0.1, 0.1, 0.2),
            c: Vec3::new(1.0, 1.0, 1.0),
            d: Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

impl GradientBindGroup {
    pub fn with_a(mut self, a: Vec3) -> Self {
        self.a = a;
        self
    }
    pub fn with_b(mut self, b: Vec3) -> Self {
        self.b = b;
        self
    }
    pub fn with_c(mut self, c: Vec3) -> Self {
        self.c = c;
        self
    }
    pub fn with_d(mut self, d: Vec3) -> Self {
        self.d = d;
        self
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct StarsBindGroup {
    /// how fast to rotate sky per sec in radians
    pub sky_rotation_speed: f32,
    pub sample_scale: f32,
    pub star_threshold: f32,
    pub star_threshold_blink: f32,
    pub blink_speed: f32,
    pub mask_scale: f32,
    pub mask_threshold: f32,
    pub blink_variance_scale: f32,
}

impl Default for StarsBindGroup {
    fn default() -> Self {
        Self {
            sky_rotation_speed: 0.01,
            sample_scale: 9.0,
            mask_scale: 1.0,
            blink_variance_scale: 0.03,
            mask_threshold: 0.4,
            star_threshold: 0.9,
            star_threshold_blink: 0.01,
            blink_speed: 10.0,
        }
    }
}

impl StarsBindGroup {
    pub fn with_sky_rotation_speed(mut self, speed: f32) -> Self {
        self.sky_rotation_speed = speed;
        self
    }
    pub fn with_sample_scale(mut self, scale: f32) -> Self {
        self.sample_scale = scale;
        self
    }
    pub fn with_star_threshold(mut self, threshold: f32) -> Self {
        self.star_threshold = threshold;
        self
    }
    pub fn with_star_threshold_blink(mut self, threshold: f32) -> Self {
        self.star_threshold_blink = threshold;
        self
    }
    pub fn with_blink_speed(mut self, speed: f32) -> Self {
        self.blink_speed = speed;
        self
    }
    pub fn with_mask_scale(mut self, scale: f32) -> Self {
        self.mask_scale = scale;
        self
    }
    pub fn with_mask_threshold(mut self, threshold: f32) -> Self {
        self.mask_threshold = threshold;
        self
    }
    pub fn with_blink_variance_scale(mut self, scale: f32) -> Self {
        self.blink_variance_scale = scale;
        self
    }
}

#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct SunBindGroup {
    pub sun_dir: Vec3,
    pub sun_color: Vec4,
    pub sun_strength: f32,
    pub sun_sharpness: f32,
}

impl Default for SunBindGroup {
    fn default() -> Self {
        Self {
            sun_dir: Vec3::new(0.0, 0.1, -1.0),
            sun_color: Vec4::new(1.0, 1.0, 0.5, 1.0),
            sun_strength: 1.5,
            sun_sharpness: 164.0,
        }
    }
}

impl SunBindGroup {
    pub fn with_sun_dir(mut self, dir: Vec3) -> Self {
        self.sun_dir = dir;
        self
    }
    pub fn with_sun_color(mut self, color: Vec4) -> Self {
        self.sun_color = color;
        self
    }
    pub fn with_sun_strength(mut self, strength: f32) -> Self {
        self.sun_strength = strength;
        self
    }
    pub fn with_sun_sharpness(mut self, sharpness: f32) -> Self {
        self.sun_sharpness = sharpness;
        self
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Reflect, ShaderType)]
pub struct AuroraBindGroup {
    pub color_bottom: LinearRgba,
    pub color_top: LinearRgba,
    pub alpha: f32,
    pub density: f32,
    pub sharpness: f32,
    pub num_samples: i32,
    pub start_height: f32,
    pub end_height: f32,
    pub flow_scale: f32,
    pub flow_strength: f32,
    pub flow_speed: f32,
    pub flow_x_speed: f32,
    pub wiggle_scale: f32,
    pub wiggle_strength: f32,
    pub wiggle_speed: f32,
    pub undersparkle_color_primary: LinearRgba,
    pub undersparkle_color_secondary: LinearRgba,
    pub undersparkle_scale: f32,
    pub undersparkle_speed: f32,
    pub undersparkle_threshold: f32,
    pub undersparkle_height: f32,
    pub opacity_per_sample: f32,
}

impl Default for AuroraBindGroup {
    fn default() -> Self {
        Self {
            color_bottom: LinearRgba::new(0.0, 1.0, 0.2, 1.0),
            alpha: 0.7,
            density: 0.05,
            sharpness: 1.56,
            num_samples: 60,
            start_height: 3.1,
            end_height: 4.8,
            flow_scale: 0.002,
            flow_strength: 4.3,
            flow_speed: 0.005,
            flow_x_speed: -0.6,
            wiggle_scale: 0.03,
            wiggle_strength: 1.05,
            wiggle_speed: 0.1,
            color_top: LinearRgba::new(0.0, 1.0, 0.8, 1.0),
            undersparkle_color_primary: LinearRgba::new(0.0, 2.3, 0.0, 1.0),
            undersparkle_color_secondary: LinearRgba::new(6.3, 0.2, 4.0, 1.0),
            undersparkle_scale: 0.004,
            undersparkle_speed: 0.02,
            undersparkle_threshold: 0.3,
            undersparkle_height: 0.3,
            opacity_per_sample: 0.18,
        }
    }
}

impl AuroraBindGroup {
    pub fn with_color_bottom(mut self, color: LinearRgba) -> Self {
        self.color_bottom = color;
        self
    }
    pub fn with_color_top(mut self, color: LinearRgba) -> Self {
        self.color_top = color;
        self
    }
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }
    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density;
        self
    }
    pub fn with_sharpness(mut self, sharpness: f32) -> Self {
        self.sharpness = sharpness;
        self
    }
    pub fn with_num_samples(mut self, samples: i32) -> Self {
        self.num_samples = samples;
        self
    }
    pub fn with_start_height(mut self, height: f32) -> Self {
        self.start_height = height;
        self
    }
    pub fn with_end_height(mut self, height: f32) -> Self {
        self.end_height = height;
        self
    }
    pub fn with_flow_scale(mut self, scale: f32) -> Self {
        self.flow_scale = scale;
        self
    }
    pub fn with_flow_strength(mut self, strength: f32) -> Self {
        self.flow_strength = strength;
        self
    }
    pub fn with_flow_speed(mut self, speed: f32) -> Self {
        self.flow_speed = speed;
        self
    }
    pub fn with_flow_x_speed(mut self, speed: f32) -> Self {
        self.flow_x_speed = speed;
        self
    }
    pub fn with_wiggle_scale(mut self, scale: f32) -> Self {
        self.wiggle_scale = scale;
        self
    }
    pub fn with_wiggle_strength(mut self, strength: f32) -> Self {
        self.wiggle_strength = strength;
        self
    }
    pub fn with_wiggle_speed(mut self, speed: f32) -> Self {
        self.wiggle_speed = speed;
        self
    }
    pub fn with_undersparkle_color_primary(mut self, color: LinearRgba) -> Self {
        self.undersparkle_color_primary = color;
        self
    }
    pub fn with_undersparkle_color_secondary(mut self, color: LinearRgba) -> Self {
        self.undersparkle_color_secondary = color;
        self
    }
    pub fn with_undersparkle_scale(mut self, scale: f32) -> Self {
        self.undersparkle_scale = scale;
        self
    }
    pub fn with_undersparkle_speed(mut self, speed: f32) -> Self {
        self.undersparkle_speed = speed;
        self
    }
    pub fn with_undersparkle_threshold(mut self, threshold: f32) -> Self {
        self.undersparkle_threshold = threshold;
        self
    }
    pub fn with_undersparkle_height(mut self, height: f32) -> Self {
        self.undersparkle_height = height;
        self
    }
    pub fn with_opacity_per_sample(mut self, opacity: f32) -> Self {
        self.opacity_per_sample = opacity;
        self
    }
}

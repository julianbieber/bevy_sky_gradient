use bevy::{asset::uuid_handle, prelude::*};

pub const SKY_SHADER_PATH: &str = "shaders/full_sky.wesl";
pub const SKY_SHADER_HANDLE: Handle<Shader> = uuid_handle!("0aed3aa7-55d3-43be-9e04-5637b0e9ceef");
pub const GRADIENT_SHADER_PATH: &str = "shaders/gradient.wesl";
pub const GRADIENT_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("0aed3aa1-15d3-42be-9e04-5637b0e9cefc");
pub const AURORA_SHADER_PATH: &str = "shaders/aurora.wesl";
pub const AURORA_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("0aed3aa1-15d3-42be-9e03-2137b0eecbfc");
pub const FULL_AURORA_SHADER_PATH: &str = "shaders/full_aurora.wesl";
pub const FULL_AURORA_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("0aed3aa1-15d3-42be-9e03-2731b4eecbfb");
pub const SUN_SHADER_PATH: &str = "shaders/sun.wesl";
pub const SUN_SHADER_HANDLE: Handle<Shader> = uuid_handle!("0aed3aa1-15d3-42be-9e03-2137b0e2c3fb");
pub const STARS_SHADER_PATH: &str = "shaders/stars.wesl";
pub const STARS_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("1a3d3ae1-15d3-42be-9e03-2137b0e2c3fb");
pub const NOISE_SHADER_PATH: &str = "shaders/noise.wesl";
pub const NOISE_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("1a3d3ae1-15d3-42be-9e03-2131b0e5c2fe");
pub const FULL_GRADIENT_SHADER_PATH: &str = "shaders/full_gradient.wesl";
pub const FULL_GRADIENT_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("1a3d3ae1-15d3-42be-9e03-2131b0e3c1ef");

pub struct SkyAssetsPlugin;

impl Plugin for SkyAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_shaders);
    }
}

pub fn initialize_shaders(mut shaders: ResMut<Assets<Shader>>) {
    let _result = shaders.insert(
        &SKY_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/full_sky.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", SKY_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/full_sky.wesl",
        ),
    );
    let _result = shaders.insert(
        &GRADIENT_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/gradient.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", GRADIENT_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/gradient.wesl",
        ),
    );
    let _result = shaders.insert(
        &FULL_GRADIENT_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/full_gradient.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", FULL_GRADIENT_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/full_gradient.wesl",
        ),
    );
    let _result = shaders.insert(
        &AURORA_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/aurora.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", AURORA_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/aurora.wesl",
        ),
    );
    let _result = shaders.insert(
        &FULL_AURORA_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/full_aurora.wesl").into()).unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", FULL_AURORA_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/full_aurora.wesl",
        ),
    );
    let _result = shaders.insert(
        &STARS_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/stars.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", STARS_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/stars.wesl",
        ),
    );
    let _result = shaders.insert(
        &SUN_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/sun.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", SUN_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/sun.wesl",
        ),
    );
    let _result = shaders.insert(
        &NOISE_SHADER_HANDLE,
        Shader::from_wesl(
            String::from_utf8(include_bytes!("../assets/shaders/noise.wesl").into())
                .unwrap_or_else(|_| panic!("'{}' shader is not valid UTF-8", NOISE_SHADER_PATH)),
            "embedded://bevy_sky_gradient/shaders/noise.wesl",
        ),
    );
}

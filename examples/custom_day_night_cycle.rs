use bevy::{color::palettes::css::WHITE, light::light_consts::lux::AMBIENT_DAYLIGHT, prelude::*};
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_sky_gradient::{
    gradient::SkyPaletteBuilder,
    noise::NoiseHandles,
    plugin::{AuroraTextureHandle, GradientTextureHandle},
    prelude::*,
    sky_material::FullSkyMaterial,
};

// This example show how you can customize any aspect of the sky
// here we manually spawn: skybox, and our sun light
// we also configure the cycle timings (long night)
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NoCameraPlayerPlugin)
        // SKY
        .add_plugins(
            SkyPlugin::builder_all_features()
                .set_spawn_default_skybox(false)
                .set_cycle(SkyCyclePlugin {
                    sky_time_settings: SkyTimeSettings {
                        day_time_sec: 3.0,
                        night_time_sec: 3.0,
                        sunrise_time_sec: 0.2,
                        sunset_time_sec: 0.2,
                    },
                    sky_time: SkyTime::default(),
                })
                .set_sun_driver(SunDriverPlugin {
                    spawn_default_sun_light: false,
                    sun_settings: SunSettings {
                        illuminance: 10000.0,
                        sun_color: vec4(1.0, 1.0, 0.0, 1.0),
                        sun_light_color: Color::WHITE,
                        sun_strength: default(),
                        sun_sharpness: default(),
                    },
                })
                .set_gradient_driver(GradientDriverPlugin {
                    sky_palette_builder: CUSTOM_SKY_PALETTE_BUILDER,
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sky_materials: ResMut<Assets<FullSkyMaterial>>,
    noise_handles: Res<NoiseHandles>,
    aurora_handle: Res<AuroraTextureHandle>,
    gradient_handle: Res<GradientTextureHandle>,
) {
    // MANUAL SKYBOX CREATION, using a cuboid mesh instead of Sphere, because we can.
    let mut mesh = Cuboid::from_length(1.0).mesh().build();
    bevy_sky_gradient::utils::flip_mesh_normals(&mut mesh);
    commands.spawn((
        Mesh3d(meshes.add(mesh)),
        // if you manually create the sky mesh...
        // you still need valid texture handles, so we can fetch that.
        MeshMaterial3d(sky_materials.add(FullSkyMaterial {
            noise3_image: noise_handles.noise3.clone(),
            voronoi3_image: noise_handles.voronoi3.clone(),
            aurora_image: aurora_handle.render_target.clone(),
            gradient_image: gradient_handle.render_target.clone(),
            ..default()
        })),
    ));

    // MANUAL SUN LIGHT SOURCE creation
    commands.spawn((
        SunDriverTag,
        DirectionalLight {
            color: WHITE.into(),
            illuminance: AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform::default(),
    ));

    // spawn a flat circular base.
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(3.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // camera
    commands.spawn((
        // tell SkyPlugin we want the skybox centered on this camera
        SkyboxMagnetTag,
        Camera3d::default(),
        Transform::from_xyz(-0.4, 0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
    ));
}

// Custom sky palette with red/orange hues
pub const CUSTOM_SKY_PALETTE_BUILDER: SkyPaletteBuilder = SkyPaletteBuilder {
    day: bevy_sky_gradient::gradient::SkyPalette {
        a: Vec3::new(0.3, 0.4, 0.8),
        b: Vec3::new(0.2, 0.1, 0.1),
        c: Vec3::new(1.0, 1.0, 1.0),
        d: Vec3::new(0.0, 0.0, 0.0),
    },
    night: bevy_sky_gradient::gradient::SkyPalette {
        a: Vec3::new(0.0, 0.0, 0.1),
        b: Vec3::new(0.05, 0.0, 0.05),
        c: Vec3::new(1.0, 1.0, 1.0),
        d: Vec3::new(0.0, 0.0, 0.0),
    },
};

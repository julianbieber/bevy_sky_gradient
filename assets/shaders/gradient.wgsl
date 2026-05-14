// Cosine-based color palette system
// palette(t) = a + b * cos(2*pi * (c * t + d))

struct PaletteParams {
    a: vec3<f32>,
    b: vec3<f32>,
    c: vec3<f32>,
    d: vec3<f32>,
};

fn palette(t: f32, params: PaletteParams) -> vec3<f32> {
    return params.a + params.b * cos(6.283185 * (params.c * t + params.d));
}

// Gradient function that samples the palette based on view direction
// Converts view direction to a t value (0.0 to 1.0) based on vertical position
fn gradient(view_dir: vec3<f32>, params: PaletteParams) -> vec4<f32> {
    let t = clamp(view_dir.y * 0.5 + 0.5, 0.0, 1.0);
    let col = palette(t, params);
    return vec4<f32>(col, 1.0);
}

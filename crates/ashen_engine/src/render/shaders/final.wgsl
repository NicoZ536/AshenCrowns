// Ausgabe: pixelgenaue Hochskalierung mit Subpixel-Kamera, Bloom, Tonemapping,
// Farbkorrektur, Vignette, Filmkorn, Gamma.

@group(1) @binding(0) var t_scene: texture_2d<f32>;
@group(1) @binding(1) var t_bloom: texture_2d<f32>;
@group(1) @binding(2) var s_point: sampler;
@group(1) @binding(3) var s_linear: sampler;

// Tonemapping: linear bis 0.6 (gemalte Pixel-Art-Farben bleiben erhalten), weiche Schulter darüber.
fn tonemap(x: vec3<f32>) -> vec3<f32> {
    let knee = 0.6;
    let over = max(x - vec3<f32>(knee), vec3<f32>(0.0));
    let soft = vec3<f32>(knee) + (1.0 - knee) * (vec3<f32>(1.0) - exp(-over / (1.0 - knee)));
    return select(soft, x, x <= vec3<f32>(knee));
}

fn to_srgb(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

@fragment
fn fs_main(in: FsQuad) -> @location(0) vec4<f32> {
    let scale = g.output.w;
    // Ausgabepixel -> interne Pixel inkl. Subpixel-Versatz (1 Pixel Rand).
    let ip = in.pos.xy / scale + g.camera.zw;
    let suv = ip * g.screen.zw;
    var col = textureSampleLevel(t_scene, s_point, suv, 0.0).rgb;
    if (g.bloom.w > 0.5) {
        col = col + textureSampleLevel(t_bloom, s_linear, suv, 0.0).rgb * g.bloom.x;
    }
    col = col * g.grade.x;
    col = tonemap(max(col, vec3<f32>(0.0)));
    // Kontrast und Sättigung um Mittelgrau.
    col = (col - 0.5) * g.grade.y + 0.5;
    let l = dot(col, vec3<f32>(0.299, 0.587, 0.114));
    col = mix(vec3<f32>(l), col, g.grade.z);
    // Split-Toning: Schatten zur Tönung, Lichter warm.
    col = col + g.grade_tint.rgb * (1.0 - l) * 0.12;
    col = clamp(col, vec3<f32>(0.0), vec3<f32>(1.0));
    // Vignette.
    let q = in.uv - 0.5;
    let vig = 1.0 - dot(q, q) * g.grade.w * 1.6;
    col = col * clamp(vig, 0.0, 1.0);
    // Filmkorn (statisch je Frame-Pixel, sehr dezent).
    if (g.grade_tint.w > 0.0) {
        let n = hash12(in.pos.xy + vec2<f32>(g.time.z * 7.13, g.time.z * 3.7)) - 0.5;
        col = col + vec3<f32>(n * g.grade_tint.w);
    }
    col = clamp(col, vec3<f32>(0.0), vec3<f32>(1.0));
    if (g.output.z > 0.5) {
        // sRGB-Ziel: Hardware kodiert. Wir liefern linear.
        return vec4<f32>(col, 1.0);
    }
    return vec4<f32>(to_srgb(col), 1.0);
}

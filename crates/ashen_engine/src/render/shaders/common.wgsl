// Gemeinsame Uniforms aller Passes. Muss exakt zu `render::globals::Globals` passen.
struct Globals {
    view_proj: mat4x4<f32>,
    screen: vec4<f32>,      // interne Breite, Höhe, 1/Breite, 1/Höhe
    time: vec4<f32>,        // Zeit (s), dt, Frame, Pixelskalierung
    camera: vec4<f32>,      // Weltpixel der linken oberen Ecke (x, y), Subpixel-Versatz (x, y)
    ambient: vec4<f32>,     // Umgebungslicht rgb, Intensität
    sun_color: vec4<f32>,   // Sonnen-/Mondfarbe rgb, Intensität
    sun_dir: vec4<f32>,     // Lichtrichtung xyz (Bildschirmraum, z zum Betrachter), Schattenstärke
    fog: vec4<f32>,         // Dichte, Bodennebel, Wind x, Wind y
    fog_color: vec4<f32>,   // rgb, Lichtstreuung
    weather: vec4<f32>,     // Regen, Asche/Schnee, Blitz, Nässe
    fow: vec4<f32>,         // Kartenbreite, Kartenhöhe, aktiv, Dunkelheit unerforscht
    grade: vec4<f32>,       // Belichtung, Kontrast, Sättigung, Vignette
    grade_tint: vec4<f32>,  // Tönung rgb, Filmkorn
    bloom: vec4<f32>,       // Intensität, Schwelle, Knie, aktiv
    output: vec4<f32>,      // Ausgabe-Breite, -Höhe, sRGB-Ziel (1/0), Pixelskalierung
    tile: vec4<f32>,        // halbe Kachelbreite, halbe Kachelhöhe, Normalmapping aktiv, Qualität
};

@group(0) @binding(0) var<uniform> g: Globals;

fn hash12(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 = p3 + dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise2(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = hash12(i);
    let b = hash12(i + vec2<f32>(1.0, 0.0));
    let c = hash12(i + vec2<f32>(0.0, 1.0));
    let d = hash12(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

fn fbm2(p_in: vec2<f32>) -> f32 {
    var p = p_in;
    var s = 0.0;
    var a = 0.5;
    for (var i = 0; i < 4; i = i + 1) {
        s = s + noise2(p) * a;
        p = p * 2.03 + vec2<f32>(17.1, 9.7);
        a = a * 0.5;
    }
    return s;
}

fn bayer4(p: vec2<f32>) -> f32 {
    let x = u32(p.x) % 4u;
    let y = u32(p.y) % 4u;
    let m = array<f32, 16>(0.0, 8.0, 2.0, 10.0, 12.0, 4.0, 14.0, 6.0, 3.0, 11.0, 1.0, 9.0, 15.0, 7.0, 13.0, 5.0);
    return m[y * 4u + x] / 16.0;
}

// Weltpixel -> Kachelkoordinate (Umkehrung der isometrischen Projektion).
fn world_to_tile(w: vec2<f32>) -> vec2<f32> {
    let a = w.x / g.tile.x;
    let b = w.y / g.tile.y;
    return vec2<f32>((a + b) * 0.5, (b - a) * 0.5);
}

struct FsQuad {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Bildschirmfüllendes Dreieck ohne Vertexpuffer.
@vertex
fn vs_fullscreen(@builtin(vertex_index) i: u32) -> FsQuad {
    var out: FsQuad;
    let x = f32((i << 1u) & 2u);
    let y = f32(i & 2u);
    out.pos = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    out.uv = vec2<f32>(x, y);
    return out;
}

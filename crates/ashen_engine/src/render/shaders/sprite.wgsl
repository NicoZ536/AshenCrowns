// G-Buffer-Pass für Sprites, Wasser, Schatten und Hitzeflimmern.
// Ziel 0: Albedo (sRGB) + Hitzeverzerrung in Alpha.
// Ziel 1: Normale xy, Emission, Flags (<0.5 beleuchtet mit Schattenanteil, 0.5–0.75 unbeleuchtet).

@group(1) @binding(0) var t_albedo: texture_2d<f32>;
@group(1) @binding(1) var t_normal: texture_2d<f32>;
@group(1) @binding(2) var s_point: sampler;

struct Instance {
    @location(0) pos: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) uv: vec4<f32>,
    @location(3) color: vec4<f32>,
    @location(4) p0: vec4<f32>, // Material, Emission (Zusatz), Spiegelung (0/1), reserviert
    @location(5) p1: vec4<f32>, // Scherung, Stauchung, Wassertiefe, reserviert
};

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) p0: vec4<f32>,
    @location(3) @interpolate(flat) p1: vec4<f32>,
    @location(4) world: vec2<f32>,
    @location(5) local: vec2<f32>,
};

const MAT_LIT: f32 = 0.0;
const MAT_WATER: f32 = 1.0;
const MAT_UNLIT: f32 = 2.0;
const MAT_EMISSIVE: f32 = 3.0;
const MAT_SHADOW: f32 = 4.0;
const MAT_HEAT: f32 = 5.0;
const MAT_SOLID: f32 = 6.0; // einfarbig (UI-Balken), Textur wird ignoriert

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VsOut {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0)
    );
    let c = corners[vi];
    var local = c * inst.size;
    // Scherung: die Oberkante wird relativ zur Unterkante verschoben (Wind, Schattenwurf).
    local.x = local.x + (1.0 - c.y) * inst.p1.x * inst.size.y;
    // Stauchung: Schatten werden flach auf den Boden gelegt.
    local.y = inst.size.y - (inst.size.y - local.y) * (1.0 - inst.p1.y);
    let world = inst.pos + local;
    var out: VsOut;
    out.clip = g.view_proj * vec4<f32>(world, 0.0, 1.0);
    var u = c.x;
    if (inst.p0.z > 0.5) {
        u = 1.0 - u;
    }
    out.uv = vec2<f32>(mix(inst.uv.x, inst.uv.z, u), mix(inst.uv.y, inst.uv.w, c.y));
    out.color = inst.color;
    out.p0 = inst.p0;
    out.p1 = inst.p1;
    out.world = world;
    out.local = c;
    return out;
}

struct GOut {
    @location(0) albedo: vec4<f32>,
    @location(1) normal: vec4<f32>,
};

fn dither_discard(alpha: f32, frag: vec2<f32>) {
    if (alpha < 0.02 || alpha < bayer4(frag)) {
        discard;
    }
}

fn texel(uv: vec2<f32>, t: texture_2d<f32>) -> vec4<f32> {
    return textureSampleLevel(t, s_point, uv, 0.0);
}

@fragment
fn fs_gbuffer(in: VsOut) -> GOut {
    let mat = in.p0.x;
    var out: GOut;
    if (mat == MAT_SOLID) {
        dither_discard(in.color.a, in.clip.xy);
        out.albedo = vec4<f32>(in.color.rgb, 0.0);
        out.normal = vec4<f32>(0.5, 0.5, in.p0.y, 0.6);
        return out;
    }
    let tex = texel(in.uv, t_albedo);
    let nrm = texel(in.uv, t_normal);
    let alpha = tex.a * in.color.a;
    dither_discard(alpha, in.clip.xy);
    var n = nrm.xy * 2.0 - 1.0;
    if (in.p0.z > 0.5) {
        n.x = -n.x;
    }
    var albedo = tex.rgb * in.color.rgb;
    var emissive = nrm.b * (1.0 + in.p0.y);
    var flags = 0.0;

    if (mat == MAT_WATER) {
        // Animiertes Wasser: Tiefenfarbe, Wellen, Schaum am Ufer, Glitzern.
        let t = g.time.x;
        let w = in.world;
        let depth = clamp(in.p1.z, 0.0, 1.0);
        let shallow = vec3<f32>(0.10, 0.25, 0.28);
        let deep = vec3<f32>(0.025, 0.07, 0.10);
        var col = mix(shallow, deep, depth);
        let wave1 = sin(w.x * 0.21 + w.y * 0.43 + t * 1.6);
        let wave2 = sin(w.x * -0.13 + w.y * 0.31 - t * 1.1 + 1.7);
        let ripple = noise2(vec2<f32>(w.x * 0.09 + t * 0.35, w.y * 0.18 - t * 0.2));
        let h = wave1 * 0.35 + wave2 * 0.35 + ripple * 0.6;
        // Quantisierte Lichtbänder für Pixel-Art-Optik.
        let band = floor((h * 0.5 + 0.5) * 4.0) / 4.0;
        col = col * (0.85 + band * 0.3);
        let foam_n = noise2(vec2<f32>(w.x * 0.25 + t * 0.6, w.y * 0.5));
        let foam = step(0.62 + depth * 0.9, foam_n + (1.0 - depth) * 0.35);
        col = mix(col, vec3<f32>(0.55, 0.68, 0.66), foam * 0.6);
        let spark = step(0.985, hash12(floor(w / 2.0) + floor(t * 3.0)));
        albedo = col * tex.rgb * 2.2;
        emissive = spark * 0.6 * (1.0 - depth * 0.5);
        n = vec2<f32>(cos(w.x * 0.21 + t) * 0.25, sin(w.y * 0.43 + t * 1.3) * 0.25);
    } else if (mat == MAT_UNLIT) {
        flags = 0.6;
    } else if (mat == MAT_EMISSIVE) {
        emissive = max(emissive, in.p0.y);
    }
    out.albedo = vec4<f32>(albedo, 0.0);
    out.normal = vec4<f32>(n * 0.5 + 0.5, clamp(emissive, 0.0, 1.0), flags);
    return out;
}

// Schatten: schreibt nur den Schattenanteil in den Alphakanal von Ziel 1.
@fragment
fn fs_shadow(in: VsOut) -> GOut {
    let tex = texel(in.uv, t_albedo);
    let a = tex.a * in.color.a;
    if (a < 0.03) {
        discard;
    }
    // Weiche Kante: Stärke nimmt zur Oberkante des geworfenen Schattens ab.
    let fade = mix(0.55, 1.0, in.local.y);
    var out: GOut;
    out.albedo = vec4<f32>(0.0);
    out.normal = vec4<f32>(0.0, 0.0, 0.0, clamp(a * fade * g.sun_dir.w, 0.0, 1.0) * 0.49);
    return out;
}

// Hitzeflimmern: schreibt nur die Verzerrungsstärke in den Alphakanal von Ziel 0.
@fragment
fn fs_heat(in: VsOut) -> GOut {
    let tex = texel(in.uv, t_albedo);
    let a = tex.a * in.color.a;
    if (a < 0.03) {
        discard;
    }
    var out: GOut;
    out.albedo = vec4<f32>(0.0, 0.0, 0.0, a * in.p0.y);
    out.normal = vec4<f32>(0.0);
    return out;
}

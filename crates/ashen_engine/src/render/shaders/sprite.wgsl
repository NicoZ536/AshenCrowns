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
        // Ruhiges Pixel-Art-Wasser: Tiefenbänder aus einem Distanzfeld zum Ufer (an den Kachelecken
        // gespeichert, hier bilinear gelesen und per Bayer-Raster abgestuft), dünne wandernde
        // Wellenlinien, ein weicher Brandungssaum und seltene Glanzpunkte. Furt: heller Sandgrund.
        let t = g.time.x;
        let w = in.world;
        let ford = in.p1.z < -0.5;
        let pk = u32(in.p1.w + 0.5);
        let c00 = f32(pk & 15u);
        let c10 = f32((pk >> 4u) & 15u);
        let c01 = f32((pk >> 8u) & 15u);
        let c11 = f32((pk >> 12u) & 15u);
        let tf = fract(world_to_tile(w));
        var depth = clamp(mix(mix(c00, c10, tf.x), mix(c01, c11, tf.x), tf.y) / 15.0, 0.0, 1.0);
        if (ford) {
            depth = 0.3;
        }
        let bay = bayer4(in.clip.xy);
        let lv = clamp(floor(depth * 4.0 + bay), 0.0, 4.0);
        var col = vec3<f32>(0.20, 0.42, 0.43);
        if (lv > 3.5) {
            col = vec3<f32>(0.05, 0.13, 0.21);
        } else if (lv > 2.5) {
            col = vec3<f32>(0.08, 0.19, 0.27);
        } else if (lv > 1.5) {
            col = vec3<f32>(0.11, 0.26, 0.32);
        } else if (lv > 0.5) {
            col = vec3<f32>(0.15, 0.34, 0.38);
        }
        // Wellenlinien: kurze, langsam driftende Striche, leicht geschwungen.
        let warp = sin(w.x * 0.07 + t * 0.5) * 1.6 + sin(w.x * 0.19 - t * 0.8 + w.y * 0.1) * 0.6;
        let row = (w.y + warp) / 6.0;
        let ri = floor(row);
        let streak = noise2(vec2<f32>(w.x * 0.045 + t * 0.12 * (1.0 + fract(ri * 0.37)) + ri * 7.3, ri * 3.1));
        let on_line = step(fract(row), 0.17) * step(0.58, streak);
        let trough = step(0.5, fract(row)) * step(fract(row), 0.67) * step(0.62, streak);
        let calm = 1.0 - depth * 0.55;
        col = col + vec3<f32>(0.20, 0.27, 0.25) * on_line * calm;
        col = col * (1.0 - 0.10 * trough);
        // Brandungssaum: weißer Schaum nahe am Ufer, atmet mit der Welle.
        let phase = noise2(w * 0.04) * 5.0;
        let front = 0.06 + 0.07 * (0.5 + 0.5 * sin(t * 0.9 + phase));
        let breakup = noise2(vec2<f32>(w.x * 0.18 + t * 0.2, w.y * 0.36));
        let foam = step(depth, front) * step(0.34, breakup) * (1.0 - f32(ford));
        col = mix(col, vec3<f32>(0.62, 0.74, 0.72), foam * 0.75);
        if (ford) {
            // Furt: Sandgrund scheint durch das flache Wasser.
            let sand = tex.rgb * in.color.rgb * 1.0;
            let shallow = vec3<f32>(0.10, 0.36, 0.40) + vec3<f32>(0.2, 0.24, 0.2) * on_line;
            col = mix(sand, shallow, 0.46 + 0.2 * on_line);
        } else {
            col = col * in.color.rgb;
        }
        let spark = step(0.9965, hash12(floor(w * 0.5) + floor(t * 2.0 + hash12(floor(w * 0.5)) * 3.0)));
        albedo = col;
        emissive = spark * 0.45 * (1.0 - depth * 0.6);
        n = vec2<f32>(sin(w.x * 0.12 + t * 0.9) * 0.1, sin(w.y * 0.2 + t * 0.7) * 0.1);
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

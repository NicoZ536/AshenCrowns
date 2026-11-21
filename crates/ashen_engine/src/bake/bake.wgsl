// 3D→Pixel-Bake: rastert Low-Poly-Modelle ohne Kantenglättung in Sprite-Zellen.
// Ausgabe 0: Albedo (sRGB-kodiert, Alpha = Abdeckung) mit leichtem Cel-Shading und prozeduralen Materialmustern.
// Ausgabe 1: Normale (Bildschirmraum, x rechts, y unten, z zum Betrachter) xy, Leuchtkraft, Abdeckung.

struct VIn {
    @location(0) px: vec2<f32>,
    @location(1) depth: f32,
    @location(2) normal: vec3<f32>,
    @location(3) color: vec4<f32>,
    @location(4) obj: vec3<f32>,
    @location(5) mat: u32,
    @location(6) cell: vec2<f32>,
};

struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) obj: vec3<f32>,
    @location(3) @interpolate(flat) mat: u32,
};

@vertex
fn vs_main(v: VIn) -> VOut {
    var o: VOut;
    o.clip = vec4<f32>(v.px.x / v.cell.x * 2.0 - 1.0, 1.0 - v.px.y / v.cell.y * 2.0, v.depth, 1.0);
    o.normal = v.normal;
    o.color = v.color;
    o.obj = v.obj;
    o.mat = v.mat;
    return o;
}

fn hash13(p: vec3<f32>) -> f32 {
    var q = fract(p * 0.1031);
    q = q + dot(q, q.zyx + 31.32);
    return fract((q.x + q.y) * q.z);
}

fn vnoise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let a = mix(mix(hash13(i), hash13(i + vec3<f32>(1.0, 0.0, 0.0)), u.x), mix(hash13(i + vec3<f32>(0.0, 1.0, 0.0)), hash13(i + vec3<f32>(1.0, 1.0, 0.0)), u.x), u.y);
    let b = mix(mix(hash13(i + vec3<f32>(0.0, 0.0, 1.0)), hash13(i + vec3<f32>(1.0, 0.0, 1.0)), u.x), mix(hash13(i + vec3<f32>(0.0, 1.0, 1.0)), hash13(i + vec3<f32>(1.0, 1.0, 1.0)), u.x), u.y);
    return mix(a, b, u.z);
}

fn to_srgb(c: vec3<f32>) -> vec3<f32> {
    let lo = c * 12.92;
    let hi = 1.055 * pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(hi, lo, c <= vec3<f32>(0.0031308));
}

struct FOut {
    @location(0) albedo: vec4<f32>,
    @location(1) normal: vec4<f32>,
};

@fragment
fn fs_main(in: VOut) -> FOut {
    let n = normalize(in.normal);
    let o = in.obj;
    var c = in.color.rgb;
    var glow = in.color.a;
    var shade_bias = 0.0;
    switch in.mat {
        case 1u: { // Steinziegel
            let row = floor(o.y / 0.16);
            let along = o.x + o.z + row * 0.17;
            let brick = floor(along / 0.32);
            let mortar = step(fract(o.y / 0.16), 0.14) + step(fract(along / 0.32), 0.09);
            c = c * (0.82 + hash13(vec3<f32>(brick, row, 3.0)) * 0.32) * (1.0 - min(mortar, 1.0) * 0.38);
        }
        case 2u: { // Holzbretter
            let plank = floor((o.x + o.z) / 0.11);
            let grain = vnoise(vec3<f32>(plank * 3.0, o.y * 9.0, 1.0));
            let seam = step(fract((o.x + o.z) / 0.11), 0.12);
            c = c * (0.78 + hash13(vec3<f32>(plank, 2.0, 7.0)) * 0.26 + grain * 0.12) * (1.0 - seam * 0.35);
        }
        case 3u: { // Dachziegel
            let row = floor(o.y / 0.1);
            let col = floor((o.x - o.z) / 0.13 + row * 0.5);
            let edge = step(fract(o.y / 0.1), 0.22);
            c = c * (0.8 + hash13(vec3<f32>(col, row, 5.0)) * 0.3) * (1.0 - edge * 0.3);
        }
        case 4u: { // Stroh
            let s = vnoise(vec3<f32>((o.x + o.z) * 22.0, o.y * 4.0, 2.0));
            c = c * (0.7 + s * 0.5);
        }
        case 5u: { // Stoff
            c = c * (0.88 + vnoise(o * 30.0) * 0.18);
        }
        case 6u, 17u: { // Metall / Gold: Glanzband
            let spec = pow(max(dot(n, normalize(vec3<f32>(-0.3, -0.6, 0.75))), 0.0), 8.0);
            c = c * (0.8 + vnoise(o * 18.0) * 0.12) + vec3<f32>(spec * 0.45);
        }
        case 8u: { // Laub
            let clump = vnoise(o * 7.0);
            let fine = hash13(floor(o * 24.0));
            c = c * (0.62 + clump * 0.55 + fine * 0.12);
            shade_bias = clump * 0.15 - 0.05;
        }
        case 9u: { // Rinde
            c = c * (0.7 + vnoise(vec3<f32>((o.x + o.z) * 26.0, o.y * 3.0, 4.0)) * 0.45);
        }
        case 10u: { // Knochen
            c = c * (0.85 + vnoise(o * 20.0) * 0.2);
        }
        case 11u: { // Leuchten
            glow = max(glow, 0.9);
        }
        case 12u: { // Fels
            let crack = step(abs(vnoise(o * 6.0) - 0.5), 0.03);
            c = c * (0.75 + vnoise(o * 9.0) * 0.4) * (1.0 - crack * 0.4);
        }
        case 13u: { // Putz
            c = c * (0.86 + vnoise(o * 14.0) * 0.2);
        }
        case 14u: { // Haar
            c = c * (0.75 + vnoise(vec3<f32>(o.x * 40.0, o.y * 4.0, o.z * 40.0)) * 0.4);
        }
        case 15u: { // Feldfrüchte
            let rows = step(fract((o.x - o.z) * 4.0), 0.45);
            c = mix(c * 0.55, c * (0.95 + vnoise(o * 25.0) * 0.3), rows);
        }
        case 16u: { // Leder
            c = c * (0.82 + vnoise(o * 22.0) * 0.22);
        }
        default: {}
    }
    // Cel-Shading in drei Stufen (das Spiel beleuchtet zusätzlich dynamisch über die Normalen).
    let l = normalize(vec3<f32>(-0.55, -0.7, 0.55));
    let ndl = dot(n, l) + shade_bias;
    var band = 0.7;
    if (ndl > 0.55) {
        band = 1.06;
    } else if (ndl > 0.12) {
        band = 0.88;
    }
    // Leichte Bodenverdunkelung (Kontaktschatten).
    let ao = clamp(0.78 + o.y * 0.6, 0.78, 1.0);
    let lit = c * band * select(ao, 1.0, in.mat == 11u);
    var out: FOut;
    out.albedo = vec4<f32>(to_srgb(lit), 1.0);
    out.normal = vec4<f32>(n.x * 0.5 + 0.5, n.y * 0.5 + 0.5, clamp(glow, 0.0, 1.0), 1.0);
    return out;
}

// Punktlichter: additive Akkumulation in einen HDR-Lichtpuffer (isometrisch gestaucht).

@group(1) @binding(0) var t_gnormal: texture_2d<f32>;

struct LightIn {
    @location(0) pos_radius: vec4<f32>, // Weltpixel x, y, Radius, Höhe
    @location(1) color: vec4<f32>,      // rgb, Intensität
    @location(2) params: vec4<f32>,     // Flackern, Phase, Wassertiefe-unabhängig, reserviert
};

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec2<f32>,
    @location(1) @interpolate(flat) pos_radius: vec4<f32>,
    @location(2) @interpolate(flat) color: vec4<f32>,
    @location(3) @interpolate(flat) params: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, l: LightIn) -> VsOut {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0)
    );
    let c = corners[vi];
    // Lichtkegel als Ellipse (Boden ist isometrisch auf die Hälfte gestaucht).
    let world = l.pos_radius.xy + c * vec2<f32>(l.pos_radius.z, l.pos_radius.z * 0.6);
    var out: VsOut;
    out.clip = g.view_proj * vec4<f32>(world, 0.0, 1.0);
    out.world = world;
    out.pos_radius = l.pos_radius;
    out.color = l.color;
    out.params = l.params;
    return out;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let px = vec2<i32>(in.clip.xy);
    let gn = textureLoad(t_gnormal, px, 0);
    var d = in.world - in.pos_radius.xy;
    d.y = d.y * 2.0; // Bodenabstand in isometrischer Tiefe
    let dist = length(d);
    let r = in.pos_radius.z;
    if (dist > r) {
        discard;
    }
    var att = 1.0 - dist / r;
    att = att * att;
    // Normalen-Beleuchtung (Pseudo-3D der Pixel-Art).
    var n = vec3<f32>(gn.xy * 2.0 - 1.0, 0.0);
    n.z = sqrt(max(1.0 - dot(n.xy, n.xy), 0.0));
    let to_light = normalize(vec3<f32>(-d.x, -d.y * 0.5, in.pos_radius.w));
    var diffuse = max(dot(n, to_light), 0.0);
    if (g.tile.z < 0.5) {
        diffuse = 1.0;
    }
    let wrap = 0.35 + 0.65 * diffuse;
    let flick = 1.0 - in.params.x * (0.5 + 0.5 * sin(g.time.x * 13.0 + in.params.y) * sin(g.time.x * 7.3 + in.params.y * 2.1));
    // Gequantelte Lichtstufen erhalten die Pixel-Art-Ästhetik.
    let steps = 8.0;
    let q = floor(att * wrap * steps + 0.5) / steps;
    return vec4<f32>(in.color.rgb * in.color.a * q * flick, 1.0);
}

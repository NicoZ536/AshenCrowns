// Bloom: Helligkeitsfilter, Downsample-Kette (13-Tap) und additive Upsample-Kette (Tent-Filter).

struct BloomPass {
    texel: vec4<f32>, // 1/Breite, 1/Höhe der Quelle, Schwelle, Knie
};

@group(1) @binding(0) var t_src: texture_2d<f32>;
@group(1) @binding(1) var s_linear: sampler;
@group(1) @binding(2) var<uniform> bp: BloomPass;

fn sample13(uv: vec2<f32>) -> vec3<f32> {
    let t = bp.texel.xy;
    let a = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(-2.0, -2.0), 0.0).rgb;
    let b = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(0.0, -2.0), 0.0).rgb;
    let c = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(2.0, -2.0), 0.0).rgb;
    let d = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(-2.0, 0.0), 0.0).rgb;
    let e = textureSampleLevel(t_src, s_linear, uv, 0.0).rgb;
    let f = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(2.0, 0.0), 0.0).rgb;
    let gg = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(-2.0, 2.0), 0.0).rgb;
    let h = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(0.0, 2.0), 0.0).rgb;
    let i = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(2.0, 2.0), 0.0).rgb;
    let j = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(-1.0, -1.0), 0.0).rgb;
    let k = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(1.0, -1.0), 0.0).rgb;
    let l = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(-1.0, 1.0), 0.0).rgb;
    let m = textureSampleLevel(t_src, s_linear, uv + t * vec2<f32>(1.0, 1.0), 0.0).rgb;
    var o = e * 0.125;
    o = o + (a + c + gg + i) * 0.03125;
    o = o + (b + d + f + h) * 0.0625;
    o = o + (j + k + l + m) * 0.125;
    return o;
}

@fragment
fn fs_prefilter(in: FsQuad) -> @location(0) vec4<f32> {
    let c = sample13(in.uv);
    let br = max(c.r, max(c.g, c.b));
    let knee = bp.texel.w;
    let thr = bp.texel.z;
    var soft = clamp(br - thr + knee, 0.0, 2.0 * knee);
    soft = soft * soft / (4.0 * knee + 1e-4);
    let contrib = max(soft, br - thr) / max(br, 1e-4);
    return vec4<f32>(c * contrib, 1.0);
}

@fragment
fn fs_down(in: FsQuad) -> @location(0) vec4<f32> {
    return vec4<f32>(sample13(in.uv), 1.0);
}

@fragment
fn fs_up(in: FsQuad) -> @location(0) vec4<f32> {
    let t = bp.texel.xy;
    var s = textureSampleLevel(t_src, s_linear, in.uv, 0.0).rgb * 4.0;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(-t.x, 0.0), 0.0).rgb * 2.0;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(t.x, 0.0), 0.0).rgb * 2.0;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(0.0, -t.y), 0.0).rgb * 2.0;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(0.0, t.y), 0.0).rgb * 2.0;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(-t.x, -t.y), 0.0).rgb;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(t.x, -t.y), 0.0).rgb;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(-t.x, t.y), 0.0).rgb;
    s = s + textureSampleLevel(t_src, s_linear, in.uv + vec2<f32>(t.x, t.y), 0.0).rgb;
    return vec4<f32>(s / 16.0, 1.0);
}

// Komposition: Albedo × (Umgebung + Sonne/Mond + Punktlichter) + Emission,
// Kriegsnebel, atmosphärischer Nebel, Wetter. Ergebnis: HDR-Szene in interner Auflösung.

@group(1) @binding(0) var t_albedo: texture_2d<f32>;
@group(1) @binding(1) var t_normal: texture_2d<f32>;
@group(1) @binding(2) var t_light: texture_2d<f32>;
@group(1) @binding(3) var t_fow: texture_2d<f32>;
@group(1) @binding(4) var s_point: sampler;
@group(1) @binding(5) var s_linear: sampler;

fn luma(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.299, 0.587, 0.114));
}

@fragment
fn fs_main(in: FsQuad) -> @location(0) vec4<f32> {
    let px = in.pos.xy;
    let world = g.camera.xy + px;
    var uv = in.uv;

    // Hitzeflimmern verschiebt die Abtastung.
    let heat = textureSampleLevel(t_albedo, s_point, uv, 0.0).a;
    if (heat > 0.01) {
        let wob = vec2<f32>(sin(world.y * 0.9 + g.time.x * 9.0), cos(world.x * 0.7 + g.time.x * 7.0));
        uv = uv + wob * heat * 1.5 * g.screen.zw;
    }
    let alb = textureSampleLevel(t_albedo, s_point, uv, 0.0).rgb;
    let nb = textureSampleLevel(t_normal, s_point, uv, 0.0);
    let lights = textureSampleLevel(t_light, s_point, uv, 0.0).rgb;
    let flags = nb.a;

    var col: vec3<f32>;
    if (flags >= 0.5) {
        // Unbeleuchtete Overlays (Lebensbalken, Auswahlkreise).
        return vec4<f32>(alb, 1.0);
    }
    let shadow = clamp(flags / 0.49, 0.0, 1.0);
    var n = vec3<f32>(nb.xy * 2.0 - 1.0, 0.0);
    n.z = sqrt(max(1.0 - dot(n.xy, n.xy), 0.0));
    if (g.tile.z < 0.5) {
        n = vec3<f32>(0.0, 0.0, 1.0);
    }
    let sun = normalize(g.sun_dir.xyz);
    let ndl = max(dot(n, sun), 0.0);
    let sun_term = g.sun_color.rgb * g.sun_color.a * (0.45 + 0.55 * ndl) * (1.0 - shadow * 0.85);
    let amb = g.ambient.rgb * g.ambient.a * (1.0 - shadow * 0.35);
    let lightning = g.weather.z;
    col = alb * (amb + sun_term + lights + vec3<f32>(lightning * 0.9));
    col = col + alb * nb.b * 3.2;

    // Nässe: dunkleres, leicht glänzendes Material bei Regen.
    col = col * (1.0 - g.weather.w * 0.18);

    // Kriegsnebel (in Kachelraum, linear gefiltert für weiche Ränder).
    if (g.fow.z > 0.5) {
        let tile = world_to_tile(world);
        let fuv = tile / g.fow.xy;
        var vis = textureSampleLevel(t_fow, s_linear, fuv, 0.0).r;
        if (fuv.x < 0.0 || fuv.y < 0.0 || fuv.x > 1.0 || fuv.y > 1.0) {
            vis = 0.0;
        }
        // 0 = unerforscht, 0.5 = erforscht, 1 = sichtbar
        let explored = smoothstep(0.15, 0.5, vis);
        let visible = smoothstep(0.55, 0.95, vis);
        let grey = vec3<f32>(luma(col)) * vec3<f32>(0.75, 0.8, 0.95);
        let remembered = mix(grey, col, 0.35) * 0.6;
        col = mix(remembered, col, visible);
        let unexplored = vec3<f32>(0.012, 0.012, 0.02) + fbm2(world * 0.02 + g.time.x * 0.03) * 0.02;
        col = mix(unexplored * g.fow.w, col, explored);
    }

    // Atmosphärischer Nebel: treibende Bänke im Weltraum, von Lichtern angestrahlt.
    if (g.fog.x > 0.001) {
        let drift = vec2<f32>(g.fog.z, g.fog.w) * g.time.x;
        let f1 = fbm2((world + drift * 12.0) * 0.012);
        let f2 = fbm2((world * vec2<f32>(1.0, 2.0) - drift * 7.0) * 0.02 + vec2<f32>(5.2, 1.3));
        var density = smoothstep(0.25, 0.85, f1 * 0.65 + f2 * 0.5) * g.fog.x * 0.75;
        density = density + g.fog.y * 0.18;
        density = clamp(density, 0.0, 0.72);
        let lit_fog = g.fog_color.rgb * (g.ambient.a * 0.75 + g.sun_color.a * 0.35) + lights * g.fog_color.a;
        let q = floor(density * 16.0 + bayer4(px) * 0.6) / 16.0;
        col = mix(col, lit_fog, q);
    }

    // Regen: diagonale Schlieren im Bildschirmraum.
    if (g.weather.x > 0.01) {
        let speed = 160.0;
        let p = px + vec2<f32>(g.time.x * speed * 0.35, g.time.x * speed);
        let q2 = vec2<f32>(p.x - p.y * 0.35, p.y);
        let cell = floor(q2 / vec2<f32>(5.0, 13.0));
        let r = hash12(cell);
        let local = fract(q2 / vec2<f32>(5.0, 13.0));
        let streak = step(1.0 - 0.09 * g.weather.x, r) * step(local.y, 0.45) * step(abs(local.x - 0.5), 0.1);
        col = mix(col, vec3<f32>(0.55, 0.62, 0.7) * (g.ambient.a + g.sun_color.a * 0.5 + 0.2), streak * 0.35);
    }
    // Asche-/Schneefall: treibende Flocken.
    if (g.weather.y > 0.01) {
        let p = px + vec2<f32>(sin(g.time.x * 0.7) * 20.0 + g.time.x * 9.0, g.time.x * 18.0);
        let cell = floor(p / 7.0);
        let r = hash12(cell);
        let inner = fract(p / 7.0);
        let flake = step(1.0 - 0.05 * g.weather.y, r) * step(length(inner - 0.5), 0.08);
        col = mix(col, vec3<f32>(0.72, 0.7, 0.68) * (g.ambient.a * 1.5 + g.sun_color.a * 0.7 + 0.15), flake * 0.8);
    }
    return vec4<f32>(col, 1.0);
}

#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(1)]] uv: vec2<f32>;
};

struct Input {
    time: f32;
    seed: f32;
};

[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> input: Input;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    var out: VertexOutput;
    out.uv = vertex.uv;
    out.clip_position = view.view_proj * world_position;
    return out;
}


fn random2(p: vec2<f32>) -> vec2<f32> {
    let p = vec2<f32>(dot(p, vec2<f32>(12.9898, 78.233)), dot(p, vec2<f32>(26.65125, 83.054543)));
    return fract(sin(p) * 43758.5453);
}


fn sm_vr(st: vec2<f32>, time: f32) -> f32 {
    let i_st = floor(st);
    let f_st = fract(st);

    var c: f32 = 0.0;
    for (var j = -1; j <= 1; j = j + 1 ) {
        for (var i = -1; i <= 1; i = i + 1 ) {
            let neighbor = vec2<f32>(f32(i), f32(j));
            var point = random2(i_st + neighbor);
            point = 0.5 + 0.5 * sin(time + 6.2831 * point);
            let diff = neighbor + point - f_st;
            let dist = length(diff) + (sin(time * length(random2(i_st + neighbor))) * 0.25 + 0.25);
            c = c + exp(-16.0 * dist);
        }
    }
    return -(1.0 / 16.0) * log(c);
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let coords = in.uv.xy * 20.0;

    var c = 0.0;
    for (var i = 3.0; i >= 0.0; i = i - 1.0)   {
        var vr = sm_vr(coords * pow(2.0, i), input.time/2.0 + input.seed);
        vr = smoothStep(0.5, 1.5 , vr);
        c = mix(c, vr, 1.0 - smoothStep(0.4, 0.5, vr));
        c = (c + 0.325) * (1.0 - i * 0.1);

    }
    let color = vec3<f32>(0.3, 0.01, 0.01) * (c);

    return vec4<f32>(color, 1.0);

}
 
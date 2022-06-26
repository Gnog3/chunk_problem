#import "bevy_pbr::mesh_view_bindings"
#import "bevy_pbr::mesh_bindings"
#import "bevy_pbr::mesh_functions"

struct Vertex {
    [[location(0)]] data1: vec4<u32>; // 2 bytes each
    [[location(1)]] data2: vec4<u32>; // 1 byte each
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] color: vec3<f32>;
    [[location(4)]] kind: u32;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: Vertex;
    var pos: vec3<f32> = (vec3<f32>(vertex.data1.xyz) - 32768.0) / 128.0;
    var normal: vec3<f32> = (vec3<f32>(vertex.data2.xyz) - 128.0) / 127.0;
    
    var model = mesh.model;
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(pos, 1.0));
    out.world_normal = mesh_normal_local_to_world(normal);
    out.clip_position = mesh_position_world_to_clip(out.world_position);

    var data_and_kind = (vertex.data1.w << u32(8)) | vertex.data2.w;
    var kind = data & 0b11;
    if kind == 0 {
        var data = data_and_kind >> u32(2);
        f32 r = f32(data >> u32(15)) / 127.0;
        f32 g = f32((data >> u32(7)) & 0xFF) / 255.0;
        f32 b = f32(data & 0x7F) / 127.0;
        out.color = vec3<f32>(r, g, b);
    } else if kind == 1 {
        var data = data_and_kind >> u32(3);
    } else if kind == 2 {
        var data = data_and_kind >> u32(2);
    } else if kind == 3 {
        var data = data_and_kind >> u32(2);
    }

    out.kind = kind;
    return out;
}
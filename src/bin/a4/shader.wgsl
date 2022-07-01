struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] color: vec3<f32>;
};


// vertex shader
[[stage(vertex)]]
fn vs_main(model: VertexInput) -> [[builtin(position)]] vec4<f32> {    
    
    return vec4<f32>(model.position[0], model.position[1], 0.0, 1.0);
}

// fragment shader

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}
struct Interp {
    float4 pos: SV_Position;
    float3 col;
};

float4x4 perspective_reversed_infinite_z_vk(
    float vertical_fov,
    float aspect_ratio,
    float z_near,
)  {
    let t = tan(vertical_fov / 2.0);

    let sy = 1.0 / t;

    let sx = sy / aspect_ratio;

    // Construct a matrix from a list of rows
    return float4x4(
        float4(sx, 0.0, 0.0, 0.0),
        float4(0.0, -sy, 0.0, 0.0),
        float4(0.0, 0.0, 0.0, z_near),
        float4(0.0, 0.0, -1.0, 0.0),
    );
}

float4x4 look_to_rh(float3 eye, float3 dir, float3 up) {
    let f = dir;
    let s = normalize(cross(f, up));
    let u = cross(s, f);
    
    return float4x4(
        float4(s, -dot(eye, s)),
        float4(u, -dot(eye, u)),
        float4(-f, dot(eye, f)),
        float4(0.0, 0.0, 0.0, 1.0)  
    );
}

float4x4 look_at_rh(float3 eye, float3 center, float3 up) {
    return look_to_rh(eye, normalize(center - eye), up);
}

[shader("vertex")]
Interp vertex(uint vertex_id : SV_VertexID) {
    Interp interp;
    interp.col = float3(0);
    interp.col[vertex_id] = 1.0;
    interp.pos = float4(0.0, 0.5, 0.0, 1.0);
    if (vertex_id == 0) {
        interp.pos.y = -0.5;
    } else if (vertex_id == 1) {
        interp.pos.x = 0.5;
    } else {
        interp.pos.x = -0.5;
    }
    
    let proj = perspective_reversed_infinite_z_vk(1.0297443, 1.0, 0.001);
    let view = look_at_rh(float3(0.5, 0.0, -1.0), float3(0.0), float3(0.0, -1.0, 0.0));
    
    interp.pos = mul(mul(proj, view), interp.pos);
    return interp;
}

[shader("pixel")]
float4 fragment(Interp in) {
    return float4(in.col, 1.0);
}

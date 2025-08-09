struct Uniforms {
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    time: f32,
    fractal_power: f32,
    fractal_iterations: u32,
    fractal_type: u32,
    camera_pos: vec3<f32>,
    _padding2: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    
    var vertex: vec2<f32>;
    switch vertex_index {
        case 0u: { vertex = vec2<f32>(-1.0, -1.0); }
        case 1u: { vertex = vec2<f32>( 1.0, -1.0); }
        case 2u: { vertex = vec2<f32>(-1.0,  1.0); }
        case 3u: { vertex = vec2<f32>( 1.0, -1.0); }
        case 4u: { vertex = vec2<f32>( 1.0,  1.0); }
        default: { vertex = vec2<f32>(-1.0,  1.0); }
    }
    
    out.clip_position = vec4<f32>(vertex, 0.0, 1.0);
    out.world_position = vertex;
    
    return out;
}

fn mandelbulb_distance(pos: vec3<f32>) -> f32 {
    var z = pos;
    var dr = 1.0;
    var r = 0.0;
    let power = uniforms.fractal_power;
    
    for (var i: u32 = 0u; i < uniforms.fractal_iterations; i++) {
        r = length(z);
        if (r > 4.0) { break; }
        
        let theta = acos(z.z / r);
        let phi = atan2(z.y, z.x);
        dr = pow(r, power - 1.0) * power * dr + 1.0;
        
        let zr = pow(r, power);
        let theta_new = theta * power;
        let phi_new = phi * power;
        
        z = zr * vec3<f32>(
            sin(theta_new) * cos(phi_new),
            sin(theta_new) * sin(phi_new),
            cos(theta_new)
        ) + pos;
    }
    
    return 0.5 * log(r) * r / dr;
}

fn julia_distance(pos: vec3<f32>) -> f32 {
    var z = pos;
    let c = vec3<f32>(
        0.4 * cos(uniforms.time * 0.3),
        0.4 * sin(uniforms.time * 0.2),
        0.4 * cos(uniforms.time * 0.1)
    );
    
    var dr = 1.0;
    var r = 0.0;
    let power = uniforms.fractal_power;
    
    for (var i: u32 = 0u; i < uniforms.fractal_iterations; i++) {
        r = length(z);
        if (r > 4.0) { break; }
        
        let theta = acos(z.z / r);
        let phi = atan2(z.y, z.x);
        dr = pow(r, power - 1.0) * power * dr;
        
        let zr = pow(r, power);
        let theta_new = theta * power;
        let phi_new = phi * power;
        
        z = zr * vec3<f32>(
            sin(theta_new) * cos(phi_new),
            sin(theta_new) * sin(phi_new),
            cos(theta_new)
        ) + c;
    }
    
    return 0.5 * log(r) * r / dr;
}

fn menger_sponge_distance(pos: vec3<f32>) -> f32 {
    var p = pos;
    var d = abs(p.x) - 1.0;
    d = max(d, abs(p.y) - 1.0);
    d = max(d, abs(p.z) - 1.0);
    
    var scale = 1.0;
    
    for (var i: u32 = 0u; i < 5u; i++) {
        p = abs(p);
        if (p.x < p.y) { p = p.yxz; }
        if (p.x < p.z) { p = p.zyx; }
        if (p.y < p.z) { p = p.xzy; }
        
        p = p * 3.0 - vec3<f32>(2.0, 2.0, 0.0);
        
        if (p.z < -1.0) {
            p.z += 2.0;
        }
        
        d = min(d, (max(p.x, p.y) - 1.0) / (scale * 3.0));
        scale *= 3.0;
    }
    
    return d;
}

fn kleinian_distance(pos: vec3<f32>) -> f32 {
    var p = pos;
    var scale = 1.0;
    let iterations = min(uniforms.fractal_iterations, 20u);
    
    for (var i: u32 = 0u; i < iterations; i++) {
        p = abs(p);
        
        if (p.x < p.y) { p = p.yxz; }
        if (p.x < p.z) { p = p.zyx; }
        if (p.y < p.z) { p = p.xzy; }
        
        let fold_factor = 2.0 + sin(uniforms.time * 0.2) * 0.5;
        p = p * fold_factor - vec3<f32>(fold_factor - 1.0, fold_factor - 1.0, fold_factor - 1.0);
        
        if (length(p) < 0.5) {
            p = p * 4.0;
            scale *= 4.0;
        } else if (length(p) < 1.0) {
            p = p * 2.0;
            scale *= 2.0;
        }
        
        p = p * 1.3 - vec3<f32>(0.5, 0.3, 0.8);
        scale *= 1.3;
    }
    
    return (length(p) - 0.1) / scale;
}

fn apollonian_distance(pos: vec3<f32>) -> f32 {
    var p = pos;
    var scale = 1.0;
    let power = uniforms.fractal_power * 0.5;
    
    for (var i: u32 = 0u; i < min(uniforms.fractal_iterations, 12u); i++) {
        p = abs(p);
        
        let dot_p = dot(p, p);
        if (dot_p < 1.0) {
            p = p / dot_p - vec3<f32>(1.0, 1.0, 1.0);
            scale = scale / dot_p;
        } else if (dot_p < 2.0) {
            p = p - vec3<f32>(1.0, 1.0, 1.0);
        }
        
        p = p * power;
        scale = scale * power;
    }
    
    return length(p) / abs(scale);
}

fn mandelbox_distance(pos: vec3<f32>) -> f32 {
    var p = pos;
    var dr = 1.0;
    let scale = -2.5 + sin(uniforms.time * 0.1) * 0.5;
    
    for (var i: u32 = 0u; i < min(uniforms.fractal_iterations, 20u); i++) {
        // Box fold
        p = clamp(p, vec3<f32>(-1.0, -1.0, -1.0), vec3<f32>(1.0, 1.0, 1.0)) * 2.0 - p;
        
        // Sphere fold
        let r2 = dot(p, p);
        if (r2 < 0.25) {
            p = p * 4.0;
            dr = dr * 4.0;
        } else if (r2 < 1.0) {
            p = p / r2;
            dr = dr / r2;
        }
        
        p = scale * p + pos;
        dr = dr * abs(scale) + 1.0;
        
        if (length(p) > 4.0) {
            break;
        }
    }
    
    return length(p) / abs(dr);
}

fn scene_distance(pos: vec3<f32>) -> f32 {
    var fractal_choice: i32;
    if (uniforms.fractal_type == 99u) {
        fractal_choice = i32(uniforms.time * 0.08) % 6;  // Auto-cycle mode
    } else {
        fractal_choice = i32(uniforms.fractal_type) % 6;  // Manual mode
    }
    
    if (fractal_choice == 0) {
        return mandelbulb_distance(pos);
    } else if (fractal_choice == 1) {
        return julia_distance(pos);
    } else if (fractal_choice == 2) {
        return menger_sponge_distance(pos);
    } else if (fractal_choice == 3) {
        return kleinian_distance(pos);
    } else if (fractal_choice == 4) {
        return apollonian_distance(pos);
    } else {
        return mandelbox_distance(pos);
    }
}

fn calculate_normal(pos: vec3<f32>) -> vec3<f32> {
    let epsilon = 0.001;
    let gradient = vec3<f32>(
        scene_distance(pos + vec3<f32>(epsilon, 0.0, 0.0)) - scene_distance(pos - vec3<f32>(epsilon, 0.0, 0.0)),
        scene_distance(pos + vec3<f32>(0.0, epsilon, 0.0)) - scene_distance(pos - vec3<f32>(0.0, epsilon, 0.0)),
        scene_distance(pos + vec3<f32>(0.0, 0.0, epsilon)) - scene_distance(pos - vec3<f32>(0.0, 0.0, epsilon))
    );
    return normalize(gradient);
}

fn ray_march(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec2<f32> {
    var depth = 0.0;
    let max_steps = 256;
    let max_distance = 100.0;
    let surface_threshold = 0.001;
    
    for (var i: i32 = 0; i < max_steps; i++) {
        let pos = ray_origin + ray_direction * depth;
        let distance = scene_distance(pos);
        
        if (distance < surface_threshold) {
            return vec2<f32>(depth, f32(i));
        }
        
        depth += distance;
        
        if (depth > max_distance) {
            break;
        }
    }
    
    return vec2<f32>(-1.0, f32(max_steps));
}

fn get_color_palette(t: f32, steps: f32) -> vec3<f32> {
    let normalized_steps = steps / 256.0;
    
    let color1 = vec3<f32>(0.0, 0.1, 0.3);
    let color2 = vec3<f32>(0.8, 0.2, 0.5);
    let color3 = vec3<f32>(1.0, 0.8, 0.2);
    let color4 = vec3<f32>(0.2, 0.8, 0.9);
    
    let time_offset = uniforms.time * 0.5;
    let palette_time = fract(t + time_offset);
    
    var color: vec3<f32>;
    if (palette_time < 0.33) {
        color = mix(color1, color2, palette_time * 3.0);
    } else if (palette_time < 0.66) {
        color = mix(color2, color3, (palette_time - 0.33) * 3.0);
    } else {
        color = mix(color3, color4, (palette_time - 0.66) * 3.0);
    }
    
    return mix(color * 0.3, color, 1.0 - normalized_steps);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.world_position;
    
    let inv_view = transpose(uniforms.view_matrix);
    let inv_proj = transpose(uniforms.projection_matrix);
    
    let ray_direction_view = normalize(vec3<f32>(
        uv.x / uniforms.projection_matrix[0][0],
        uv.y / uniforms.projection_matrix[1][1],
        -1.0
    ));
    
    let ray_direction = normalize((inv_view * vec4<f32>(ray_direction_view, 0.0)).xyz);
    let ray_origin = uniforms.camera_pos;
    
    let march_result = ray_march(ray_origin, ray_direction);
    let depth = march_result.x;
    let steps = march_result.y;
    
    if (depth < 0.0) {
        let bg_color = vec3<f32>(0.05, 0.05, 0.1) * (1.0 + 0.5 * sin(uniforms.time * 0.3));
        return vec4<f32>(bg_color, 1.0);
    }
    
    let hit_point = ray_origin + ray_direction * depth;
    let normal = calculate_normal(hit_point);
    
    let light_dir1 = normalize(vec3<f32>(1.0, 1.0, -1.0));
    let light_dir2 = normalize(vec3<f32>(-1.0, 0.5, 1.0));
    
    let diffuse1 = max(dot(normal, light_dir1), 0.0);
    let diffuse2 = max(dot(normal, light_dir2), 0.0) * 0.5;
    
    let view_dir = normalize(ray_origin - hit_point);
    let reflect_dir1 = reflect(-light_dir1, normal);
    let reflect_dir2 = reflect(-light_dir2, normal);
    
    let spec1 = pow(max(dot(view_dir, reflect_dir1), 0.0), 32.0);
    let spec2 = pow(max(dot(view_dir, reflect_dir2), 0.0), 16.0) * 0.3;
    
    let ambient = 0.3;
    let lighting = ambient + diffuse1 + diffuse2 + spec1 + spec2;
    
    let distance_from_origin = length(hit_point);
    let base_color = get_color_palette(distance_from_origin * 0.3, steps);
    
    let ao = 1.0 - (steps / 256.0) * 0.8;
    let fog = exp(-depth * 0.01);
    
    let final_color = base_color * lighting * ao * fog;
    
    return vec4<f32>(final_color, 1.0);
}
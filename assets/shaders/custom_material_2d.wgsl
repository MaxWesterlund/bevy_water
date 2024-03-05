#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> light_color: vec4<f32>;
@group(2) @binding(1) var<uniform> medium_light_color: vec4<f32>;
@group(2) @binding(2) var<uniform> medium_dark_color: vec4<f32>;
@group(2) @binding(3) var<uniform> dark_color: vec4<f32>;
@group(2) @binding(4) var<uniform> time: f32;


@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let resolution = 64.0;

    let uv = mesh.uv;

    let ratio = 1.0 / resolution;
    let coord = vec2<f32>(
        ratio * floor(uv.x / ratio),
        ratio * floor(uv.y / ratio)
    );

    let wave_amount = 30;
    let wave_speed = 0.07;

    var amplitude = 1.0;
    var frequency = 10.0;

    var total_amplitude = 0.0;
    
    var val = 0.0;
    var last_wave_derivative = 0.0;

    for (var i = 0; i < wave_amount; i++) {
        let angle = 3.5765 * f32(i);
        let direction = vec2<f32>(cos(angle), sin(angle));

        let content = (coord.x * direction.x + coord.y * direction.y) * frequency + time * wave_speed * frequency;

        val += amplitude * sin(content);

        last_wave_derivative = cos(content);

        total_amplitude += amplitude;

        amplitude *= 0.82;
        frequency *= 1.18;
    }

    val /= total_amplitude * 2.0;
    val += 0.5;

    if val < 0.3 {
        return dark_color;
    }
    else if val < 0.5 {
        return medium_dark_color;
    }
    else if val < 0.7 {
        return medium_light_color;
    }
    else {
        return light_color;
    }
}

struct VertexOutput {
  @builtin(position) position: vec4f,
}

struct Constants {
  screen_size: vec2f,
  position: vec2f,
  radius: f32,
}

var<push_constant> constants: Constants;

const POSITIONS: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(-1.0, 1.0),
    vec2f(1.0, 1.0),
    vec2f(-1.0, -1.0),
    vec2f(-1.0, -1.0),
    vec2f(1.0, 1.0),
    vec2f(1.0, -1.0),
);


@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;

    var positions = POSITIONS;
    let position = positions[vertex_index] * constants.radius;

    let translated_position = position + constants.position + (constants.screen_size / 2);
    let scaled_position = translated_position / constants.screen_size;
    let normalized_position = scaled_position * 2.0 - 1.0;

    out.position = vec4f(normalized_position.x, -normalized_position.y, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let frag_pos = in.position.xy - (constants.screen_size / 2);

    let dist = distance(constants.position, frag_pos);
    if dist > constants.radius {
      discard;
    }

    return vec4f(0.2, 0.2, 1.0, 1.0);
}
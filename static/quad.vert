#version 300 es
precision lowp float;

// define the vertices of a quad, so we can index it with gl_VertexID
const vec2 quadVertices[6] = vec2[](
    vec2(-1.0, -1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0, -1.0),
    vec2(-1.0,  1.0),
    vec2( 1.0,  1.0)
);

void main() {
    // get the vertex position from the quadVertices array
    vec2 vertex = quadVertices[gl_VertexID];
    // pass the vertex position to the fragment shader
    gl_Position = vec4(vertex, 0.0, 1.0);
}
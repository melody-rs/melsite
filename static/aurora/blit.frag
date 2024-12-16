#version 300 es
precision lowp float;

uniform sampler2D source;
uniform vec2 resolution;
out vec4 fragColor;

void main() {
  fragColor = texture(source, gl_FragCoord.xy / resolution);
}
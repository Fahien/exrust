attribute vec3 in_position;
attribute vec4 in_color;
attribute vec2 in_uv;

varying vec4 color;
varying vec2 uv;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 proj;

void main() {
    color = in_color;
    uv = in_uv;
    gl_Position = proj * view * transform * vec4(in_position, 1.0);
}

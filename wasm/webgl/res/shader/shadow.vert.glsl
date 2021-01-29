attribute vec3 in_position;

uniform mat4 transform;
uniform mat4 light_view_proj;

void main() {
    gl_Position = light_view_proj * transform * vec4(in_position, 1.0);
}

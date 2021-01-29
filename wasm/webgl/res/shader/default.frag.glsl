precision mediump float;

varying vec3 position;
varying vec4 color;
varying vec3 normal;
varying vec2 uv;
varying vec4 position_from_light;

uniform vec4 select_color;
uniform sampler2D tex_sampler;
uniform sampler2D shadow_map;
uniform vec3 light_color;
uniform vec3 light_position;

void main() {
    vec3 shadow_coords = (position_from_light.xyz / position_from_light.w) / 2.0 + 0.5;
    vec4 rgba_depth = texture2D(shadow_map, shadow_coords.xy);
    float depth = rgba_depth.r;
    float visibility = (shadow_coords.z > depth + 0.005 ) ? 0.5 : 1.0;

    vec3 light_direction = light_position - position;
    float n_dot_l = max(
        dot(
            normalize(light_direction),
            normalize(normal)
        ),
        0.0
    );
    vec3 diffuse = light_color * vec3(color) * n_dot_l;
    vec3 ambient = light_color * vec3(color) * 0.1;
    gl_FragColor = select_color + vec4((diffuse + ambient) * visibility, color.a) * texture2D(tex_sampler, uv);
    gl_FragColor = vec4(visibility * diffuse, 1.0);
}

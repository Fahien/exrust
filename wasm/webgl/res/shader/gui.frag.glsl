precision mediump float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D tex_sampler;

void main() {
    gl_FragColor = color * texture2D(tex_sampler, uv);
}

#version 460

layout(location = 0) in vec4 f_color;
layout(location = 1) in vec3 v_coord;

layout(location = 0) out vec4 out_color;

void main() {
    out_color = 0.1 * f_color / gl_FragCoord.z;
}

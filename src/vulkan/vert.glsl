#version 460

layout(location = 0) in vec4 color;
layout(location = 1) in vec3 position;
layout(location = 0) out vec4 f_color;
layout(location = 1) out vec3 v_coord;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;
} uniforms;

void main() {
    mat4 v_transform = uniforms.proj * uniforms.view * uniforms.model;
    gl_Position = v_transform * vec4(position, 1.0);
    // gl_Position = vec4(position, 1.0);

    f_color = color;
    v_coord = position;
}

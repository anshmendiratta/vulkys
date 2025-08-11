#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;

layout(location = 0) out vec4 f_color;
layout(location = 1) out vec3 v_coord;
layout(location = 2) out vec3 v_normal;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;
} uniforms;

void main() {
    mat4 model_view = uniforms.view * uniforms.model;
    mat4 v_transform = uniforms.proj * uniforms.view * uniforms.model;

    gl_Position = v_transform * vec4(position, 1.0);
    // gl_Position.z = -gl_Position.z;
    // gl_Position.y = -gl_Position.y;
    // gl_Position = vec4(position, 1.0);

    f_color = color;
    v_coord = position;
    v_normal = transpose(inverse(mat3(model_view))) * normal;
}

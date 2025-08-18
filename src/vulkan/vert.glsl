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
    mat3 normal_matrix = mat3(transpose(inverse(uniforms.model)));

    gl_Position = uniforms.proj * uniforms.view * uniforms.model * vec4(position, 1.0);
    gl_Position.xy *= -1; // Correction because of the Math/OpenGL convention used.

    f_color = color;
    v_coord = position;
    v_normal = normal_matrix * normal;
}

#version 460

layout(location = 0) in vec3 position;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;
} uniforms;

void main() {
    mat4 transform = uniforms.proj * uniforms.view * uniforms.model;
    gl_Position = transform * vec4(position, 1.0);    
}

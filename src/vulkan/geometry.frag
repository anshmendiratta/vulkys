#version 460

layout(location = 0) in vec4 f_color;
layout(location = 1) in vec3 v_coord;
layout(location = 2) in vec3 v_normal;

layout(location = 0) out vec4 out_color;

const vec3 LIGHT = vec3(
    10.0,
   -10.0,
    10.0
);

const float INTENSITY = 2.;

void main() {
    vec3 vert_to_light = LIGHT - v_coord;
    float brightness = 0.5 + 0.5 * (dot(normalize(v_normal), normalize(vert_to_light)));
    vec3 dark = vec3(0.2, 0.2, 0.2);
    vec3 light = vec3(1., 1., 1.);
    vec3 out_color_grayscale = mix(dark, light, pow(brightness, 1 / INTENSITY));
    float average_intensity = (out_color_grayscale.r + out_color_grayscale.g + out_color_grayscale.b) / 3; 
    
    out_color = average_intensity * f_color;
}

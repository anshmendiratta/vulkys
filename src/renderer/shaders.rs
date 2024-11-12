pub mod compute_shaders {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            // layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            // layout(binding = 0, set = 0) buffer P {
            //     vec2 pos[];
            // } positions;

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 0, set = 0) buffer V {
                vec2 vel[];
            } velocities;

            void main() {
                uint x = gl_GlobalInvocationID.x;
                float gravity = -1.0;
                float dt = 10.0;
                // positions.pos[x] += velocities.vel[x] * dt;
                velocities.vel[x] += vec2(gravity * dt);
            }
        ",
    }
}

pub mod vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;
            layout(location = 1) out vec2 pos;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                gl_PointSize = 10.0;

                pos = gl_Position.xy;
            }
        ",
    }
}
pub mod fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) out vec4 f_color;
            layout(location = 1) in vec2 pos;

            void main() {
                vec3 pixel_color = vec3(0.5, 0.2, 1.0);
                f_color = vec4(pixel_color, 1.0);
            }
        ",
    }
}

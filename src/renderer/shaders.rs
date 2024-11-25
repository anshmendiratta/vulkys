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

            layout(location = 0) in vec2 position_in;
            layout(location = 1) out vec2 position_out;

            void main() {
                gl_Position = vec4(position_in, 0.0, 1.0);
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
            layout(location = 2) in vec3 color;
            
            void main() {
                f_color = vec4(color, 1.0);
            }
        ",
    }
}

pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 410 core

            layout(location = 0) in vec4 color;
            layout(location = 1) in vec3 position;
            layout(location = 0) out vec4 f_color;
            layout(location = 1) out vec3 v_coord;
          
            // layout(push_constant) uniform Matrices {
            //     mat4 projection;
            //     mat4 view;
            //     mat4 model;
            // };

            void main() {
                // mat4 v_transform = projection * view * model;
                // gl_Position = v_transform * vec4(position, 1.0);
                gl_Position = vec4(position, 1.0);

                f_color = color;
                v_coord = position;
            }
        ",
    }
}

pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 410 core
            
            layout(location = 0) in vec4 f_color;
            layout(location = 1) in vec3 v_coord;

            layout(location = 0) out vec4 out_color;

            void main() {
                out_color = f_color;
            }
        ",
    }
}

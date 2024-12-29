pub mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec4 color;
            layout(location = 1) in vec2 position_in;
            // layout(push_constant) uniform PolygonConstants {
            //     float radius;
            // };

            layout(location = 0) out vec4 color_out;
            layout(location = 1) out vec2 position_out;
            // layout(location = 2) out float radius_out;
           
            void main() {
                color_out = color;
                // radius_out = radius;
                position_out = position_in;
                gl_Position = vec4(position_in, 0.0, 1.0);
            }
        ",
    }
}
pub mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec4 color;
            layout(location = 1) in vec2 pos;
            layout(location = 0) out vec4 f_color;

            // layout(set = 0, binding = 0) uniform sampler s;
            // layout(set = 0, binding = 1) uniform texture2D tex;
            
            void main() {
                // vec2 texture_coords = gl_FragCoord.xy;
                // f_color = texture(sampler2D(tex, s), texture_coord);
                f_color = color;
            }
        ",
    }
}

pub mod update_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        src: r"
            #version 460

            layout(push_constant) uniform ComputeConstants {
                float gravity;
                float dt;
                uint num_objects;
            };
            
            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 0, set = 0) buffer P {
                vec2 p[];
            } positions;

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 1, set = 0) buffer V {
                vec2 v[];
            } velocities;

            layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;
            layout(binding = 2, set = 0) buffer R {
                // Had to pass in [radius, 0.0] to satisfy my `get_compute_command_buffer` function
                vec2 r[];
            } radii;

            bool do_objects_collide(uint ref_object_id, uint other_object_id) {
                if (ref_object_id == other_object_id) {
                    return false;
                }

                vec2 vector_between_coms = positions.p[ref_object_id] - positions.p[other_object_id]; 
                float distance_between_coms = length(vector_between_coms);
                float radius_ref = radii.r[ref_object_id][0];
                float radius_other = radii.r[other_object_id][0];

                if (distance_between_coms <= (radius_ref + radius_other)) {
                    return true;
                }
               
                return false;
            }

            void resolve_object_collision(uint object_one_id, uint object_two_id) {
                vec2 object_one_position = positions.p[object_one_id];
                vec2 object_one_velocity = velocities.v[object_one_id];
                float object_one_radius = radii.r[object_one_id][0];
                vec2 object_two_position = positions.p[object_two_id];
                vec2 object_two_velocity = velocities.v[object_two_id];
                float object_two_radius = radii.r[object_two_id][0];

                vec2 com_distance_vector = object_one_position - object_two_position;
                float overlapping_distance = length(com_distance_vector) - object_one_radius - object_two_radius;
                float inclination_angle_of_vector = atan(com_distance_vector.y, com_distance_vector.x);
                vec2 distance_to_move_either =  vec2(overlapping_distance * cos(inclination_angle_of_vector), overlapping_distance * sin(inclination_angle_of_vector));

                vec2 updated_velocity_one = object_one_velocity - (dot(object_one_velocity - object_two_velocity, object_one_position - object_two_position))
                / (pow(length(object_two_position - object_one_position), 2)) * (object_one_position - object_two_position);
                vec2 updated_velocity_two = object_two_velocity - (dot(object_two_velocity - object_one_velocity, object_two_position - object_one_position))
                / (pow(length(object_two_position - object_one_position), 2)) * (object_two_position - object_one_position);


                positions.p[object_one_id] += -1. * distance_to_move_either;
                positions.p[object_two_id] += 1. * distance_to_move_either;
                velocities.v[object_one_id] = updated_velocity_one;
                velocities.v[object_two_id] = updated_velocity_two;
            }

            void check_and_resolve_world_collision(uint object_id) {
                vec2 object_position = positions.p[object_id];
                float object_radius = radii.r[object_id][0];
                bool crossed_lateral = false;
                bool crossed_vertical = false;

                if ((abs(object_position[0]) + abs(object_radius)) > 1.) {
                    crossed_lateral = true;
                }
                if ((abs(object_position[1]) + abs(object_radius)) > 1.) {
                    crossed_vertical = true;
                }

                if (crossed_lateral) {
                    velocities.v[object_id].x *= -1.;
                }
                if (crossed_vertical) {
                    velocities.v[object_id].y *= -1.;
                }
            }

            void main() {
                uint x = gl_GlobalInvocationID.x;

                // Check and resolve object-world collisions.
                check_and_resolve_world_collision(x);

                // Check and resolve object-object collisions.
                for (uint other_idx = 0; other_idx < num_objects; other_idx++) {
                    bool collides = do_objects_collide(x, other_idx);
                    if (collides) {
                        resolve_object_collision(x, other_idx);
                    }
                }

                // Update state as usual. First-order Euler, or something.
                vec2 position_change = vec2(velocities.v[x] * dt);
                vec2 velocity_change = vec2(0, gravity * dt);
                positions.p[x] += position_change;
                velocities.v[x] += velocity_change;
            }
            ",
    }
}

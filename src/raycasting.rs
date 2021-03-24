use gl::types::*;

use crate::chunk::{Chunk, CHUNK_BLOCKS, CHUNK_X_SIZE, CHUNK_Y_SIZE, CHUNK_Z_SIZE, COBBLESTONE};
use crate::constants::*;

pub fn raycast(aspect_ratio: f32, fov: f32, camera_pos: &glm::Vec3, camera_ray: &glm::Vec3, up: &glm::Vec3, chunk: &Chunk) -> Vec<[GLfloat; 3]> {
    let mut offsets: Vec<[GLfloat; 3]> = Vec::with_capacity(CHUNK_BLOCKS);

    let mut block_used: [bool; CHUNK_BLOCKS] = [false; CHUNK_BLOCKS];

    let far_height = 2. * ((1.1 * fov) / 2.).tan() * FAR_DISTANCE;
    let far_width = aspect_ratio * far_height;

    let right = glm::cross(&camera_ray, &up).normalize();
    let camera_up = glm::cross(&right, &camera_ray).normalize();
    let fc = camera_pos + camera_ray * FAR_DISTANCE;
    let fbl = fc - (camera_up * far_height / 2.) - (right * far_width / 2.);

    let max_u = 320;
    let max_v = 180;
    for v in 0..max_v {
        for u in 0..max_u {
            let du = u as GLfloat / max_u as GLfloat;
            let dv = v as GLfloat / max_v as GLfloat;
            // initialization step

            // ray starting position
            let ray_start = camera_pos;
            // ray direction
            let ray_direction =
                (fbl + camera_up * far_height * dv + right * far_width * du) - ray_start;
            let ray_direction = ray_direction.normalize();
            // voxel on which the ray origin is found
            let mut ray_voxel = glm::floor(&camera_pos);
            // how much to increment as we cross voxel boundaries
            let step = glm::sign(&ray_direction);
            // the value of t at which the ray crosses the fist vertical voxel boundary
            let mut t_max = ((ray_voxel + step) - ray_start)
                .component_div(&ray_direction.add_scalar(0.0000001));
            // how far along the ray we must move for each component of such movement to equal the width of a voxel
            let t_delta = (glm::vec3(1., 1., 1.)
                .component_div(&ray_direction.add_scalar(0.0000001)))
            .component_mul(&step);

            // traversal step
            for _ in 0..(FAR_DISTANCE * (3.0f32).sqrt() + 1.) as u32 {
                if t_max.x < t_max.y {
                    if t_max.x < t_max.z {
                        ray_voxel.x += step.x;
                        t_max.x += t_delta.x;
                    } else {
                        ray_voxel.z += step.z;
                        t_max.z += t_delta.z;
                    }
                } else {
                    if t_max.y < t_max.z {
                        ray_voxel.y += step.y;
                        t_max.y += t_delta.y;
                    } else {
                        ray_voxel.z += step.z;
                        t_max.z += t_delta.z;
                    }
                }

                if ray_voxel.x >= (CHUNK_X_SIZE as f32) {
                    break;
                }
                if ray_voxel.y >= (CHUNK_Y_SIZE as f32) {
                    break;
                }
                if ray_voxel.z >= (CHUNK_Z_SIZE as f32) {
                    break;
                }
                if ray_voxel.x < 0. {
                    break;
                }
                if ray_voxel.y < 0. {
                    break;
                }
                if ray_voxel.z < 0. {
                    break;
                }

                let x_ = ray_voxel.x as GLuint;
                let y_ = ray_voxel.y as GLuint;
                let z_ = ray_voxel.z as GLuint;
                let x = ray_voxel.x;
                let y = ray_voxel.y;
                let z = ray_voxel.z;

                if chunk.get(x_, y_, z_) == COBBLESTONE
                    && !block_used
                        [(z_ * CHUNK_Y_SIZE * CHUNK_X_SIZE + y_ * CHUNK_X_SIZE + x_) as usize]
                {
                    block_used
                        [(z_ * CHUNK_Y_SIZE * CHUNK_X_SIZE + y_ * CHUNK_X_SIZE + x_) as usize] =
                        true;
                    offsets.push([x, y, z]);
                    break;
                }
            }
        }
    }

    return offsets;
}

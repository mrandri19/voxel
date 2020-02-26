    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, input_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLuint;
    //     println!("after input_ssbo");
    //     dbg!(std::slice::from_raw_parts(ptr, 4));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }
    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, output_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLfloat;
    //     println!("after output_ssbo");
    //     dbg!(std::slice::from_raw_parts(ptr, 16));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }

// for z in 0..chunk.z_blocks() {
// for y in 0..chunk.y_blocks() {
// for x in 0..chunk.x_blocks() {
// // Only render cobblestone blocks
// if chunk.get(x, y, z) == COBBLESTONE {
// // Render only if not surrounded by other blocks

// let mut exists_block_on_top = true;
// if z == (chunk.z_blocks() - 1) {
// exists_block_on_top = false;
// } else if chunk.get(x, y, z + 1) == AIR {
// exists_block_on_top = false;
// }

// let mut exists_block_on_bottom = true;
// if z == 0 {
// exists_block_on_bottom = false;
// } else if chunk.get(x, y, z - 1) == AIR {
// exists_block_on_bottom = false;
// }

// let mut exists_block_on_front = true;
// if x == (chunk.x_blocks() - 1) {
// exists_block_on_front = false;
// } else if chunk.get(x + 1, y, z) == AIR {
// exists_block_on_front = false;
// }

// let mut exists_block_on_back = true;
// if x == 0 {
// exists_block_on_back = false;
// } else if chunk.get(x - 1, y, z) == AIR {
// exists_block_on_back = false;
// }

// let mut exists_block_on_right = true;
// if y == (chunk.y_blocks() - 1) {
// exists_block_on_right = false;
// } else if chunk.get(x, y + 1, z) == AIR {
// exists_block_on_right = false;
// }

// let mut exists_block_on_left = true;
// if y == 0 {
// exists_block_on_left = false;
// } else if chunk.get(x, y - 1, z) == AIR {
// exists_block_on_left = false;
// }

// if !exists_block_on_top
// || !exists_block_on_bottom
// || !exists_block_on_front
// || !exists_block_on_back
// || !exists_block_on_right
// || !exists_block_on_left
// {
// cubes_offsets.push([x as f32, y as f32, z as f32]);
// }
// }
// }
// }
// }

    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, input_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLuint;
    //     println!("before input_ssbo");
    //     dbg!(std::slice::from_raw_parts(ptr, 4));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }
    // unsafe {
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, output_ssbo);
    //     let ptr = gl::MapBuffer(gl::SHADER_STORAGE_BUFFER, gl::READ_ONLY) as *const GLfloat;
    //     println!("before output_ssbo");
    //     dbg!(std::slice::from_raw_parts(ptr, 16));

    //     gl::UnmapBuffer(gl::SHADER_STORAGE_BUFFER);
    //     gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
    // }

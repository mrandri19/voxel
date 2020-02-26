use gl::types::*;
use noise::{Fbm, MultiFractal, NoiseFn};

pub const AIR: GLuint = 0;
pub const COBBLESTONE: GLuint = 1;

pub struct Chunk {
    pub blocks: Vec<GLuint>,
}

pub const CHUNK_X_SIZE: GLuint = 128;
pub const CHUNK_Y_SIZE: GLuint = 128;
pub const CHUNK_Z_SIZE: GLuint = 128;
pub const CHUNK_BLOCKS: usize = (CHUNK_X_SIZE * CHUNK_Y_SIZE * CHUNK_Z_SIZE) as usize;
pub const CHUNK_AREA: usize = (CHUNK_X_SIZE * CHUNK_Y_SIZE) as usize;

const BASE_HEIGHT: GLuint = 10;

impl Chunk {
    // TODO(andrea): make this much much cooler.
    // See: Perlin noise, Simplex noise, Value noise, Gradient noise, fractional Brownian Motion
    // Support multiple chunks, and make terrain generation consistent
    pub fn new() -> Self {
        let mut blocks: Vec<GLuint> = vec![AIR; CHUNK_BLOCKS];
        let mut height: Vec<GLuint> = vec![BASE_HEIGHT; CHUNK_AREA];

        let fbm = Fbm::new().set_frequency(5.);

        // Create a half-air, half-cobblestone chunk
        for y in 0..CHUNK_Y_SIZE {
            for x in 0..CHUNK_X_SIZE {
                height[(y * CHUNK_X_SIZE + x) as usize] += (10.
                    * fbm.get([
                        (x as f64 / CHUNK_X_SIZE as f64),
                        (y as f64 / CHUNK_Y_SIZE as f64),
                    ])) as GLuint;
            }
        }

        for y in 0..CHUNK_Y_SIZE {
            for x in 0..CHUNK_X_SIZE {
                for z in 0..height[(y * CHUNK_X_SIZE + x) as usize] {
                    blocks[(z * CHUNK_Y_SIZE * CHUNK_X_SIZE + y * CHUNK_X_SIZE + x) as usize] =
                        COBBLESTONE;
                }
            }
        }

        Self { blocks }
    }

    #[inline(always)]
    pub fn get(&self, x: GLuint, y: GLuint, z: GLuint) -> GLuint {
        self.blocks[(z * CHUNK_Y_SIZE * CHUNK_X_SIZE + y * CHUNK_X_SIZE + x) as usize]
    }
}

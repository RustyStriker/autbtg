use std::{collections::HashMap, error::Error};

use crate::{entity::*, ivec2::IVec2, ui::EColor};

pub struct Game {
    pub entities: EntityMap,
    pub world: World,
}
impl Game {
    pub fn new() -> Game {
        Game {
            entities: HashMap::new(),
            world: World { chunks: HashMap::with_capacity(32) },
        }
    }
    pub fn game_loop(&mut self) -> Result<(), Box<dyn Error>> {
        // Loop over all entities to check who's turn is next
        // play the current turn
        // - Check who he can see and know about
        // - Check what he can see and do(tho less important)
        // - Let him act on his turn and choose what to do
        // - Update world based on all of it :)
        // advance everyone
        // repeat
        let next = self.entities.iter()
            .min_by_key(|(_,e)| e.timer())
            .map(|(id, _)| id);
        let next = next.expect("Couldn't find the next turn wtf?");
        let advance_time = self.entities.get(next).unwrap().timer();
        // advance the time
        self.entities.iter_mut()
            .for_each(|(_, e)| e.advance_timer(advance_time));
        // find who it can see :)
        // let visible = self.entities.iter_mut()
        //     .filter
        

        Ok(())
    }
}

/// # World
/// The world itself(no living creatures here)
/// 
/// N - the size of world to be calculated and kept in memory
pub struct World {
    pub chunks: HashMap<IVec2, Chunk>,
}
impl World {
    pub fn new() -> Self {
        Self { chunks: HashMap::with_capacity(64) }
    }
    pub fn con_chunk(&self, block: IVec2) -> Option<&Chunk> {
        let x = block.x() / 16;
        let y = block.y() / 16;

        self.chunks.get(&IVec2::new(x,y))
    }
    pub fn con_chunk_key(&self, block: IVec2) -> IVec2 {
        let x = block.x() / 16;
        let y = block.y() / 16;

        IVec2::new(x,y)
    }
}
pub struct Chunk {
    blocks: [[Block;Chunk::SIZE];Chunk::SIZE],
}
impl Chunk {
    pub const SIZE: usize = 16;

    pub fn new(block: Block) -> Self {
        Self { blocks: [[block; Chunk::SIZE]; Chunk::SIZE] }
    }

    pub fn get(&self, x: usize, y: usize) -> Block {
        self.blocks[x][y]
    }
    pub fn set(&mut self,new: Block, x: usize, y: usize) {
        self.blocks[x][y] = new;
    }
}

#[derive(Copy, Clone, Debug,)]
pub enum Block {
    // Passable
    Dirt,
    // Unpassble
    Wall,
}
impl Block {
    /// Turns to pass a block
    /// 
    /// 0 if unpassable
    pub fn passable(&self) -> i32 {
        match *self {
            Block::Dirt => 1,
            Block::Wall => 0,
        }
    }
    /// How visible it is beyond a given block [0,1]
    /// 
    /// 0 - blocks, 1 - doesnt affect
    pub fn visibility(&self) -> f32 {
        match *self {
            Block::Dirt => 1.0,
            Block::Wall => 0.0,
        }
    }
    pub fn color(&self) -> EColor {
        match self {
            Block::Dirt => EColor::Black,
            Block::Wall => EColor::White,
        }
    }
}
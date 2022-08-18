use std::collections::HashSet;

use crate::block::Block;

pub struct Tetris {
    width: i32,
    height: i32,
    cur_block: Block,
    fixed_block: Vec<Block>,
    is_lost: bool,
}

impl Tetris {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            // cur_block: &Block::new_rand() + Pos(width / 2, 0),
            cur_block: todo!(),
            fixed_block: vec![],
            is_lost: false,
        }
    }

    pub fn is_line_full(&self, y: i32) -> bool {
        self.fixed_block
            .iter()
            .flat_map(|b| b.iter_positions())
            .filter(|pos| pos.1 == y)
            .collect::<HashSet<_>>()
            .len() as i32
            == self.width
    }

    fn remove_line(&mut self, y: i32) {}
}

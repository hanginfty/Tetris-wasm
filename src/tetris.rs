use crate::block::{Block, Pos};
use std::collections::HashSet;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub struct Tetris {
    width: i32,
    height: i32,
    curr_block: Block,
    fixed_block: Vec<Block>,
    is_lost: bool,
}

impl Tetris {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as i32,
            height: height as i32,
            curr_block: &Block::new_rand() + Pos(width as i32 / 2, 0),
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

    pub fn iter_positions(&self) -> impl Iterator<Item = Pos> {
        let width = self.width;
        let height = self.height;

        (0..height).flat_map(move |y| (0..width).map(move |x| Pos(x, y)))
    }

    pub fn get(&self, pos: Pos) -> Option<&'static str> {
        if self.curr_block.has_position(pos) {
            Some(self.curr_block.typ())
        } else {
            self.fixed_block
                .iter()
                .find(|block| block.has_position(pos))
                .map(|block| block.typ())
        }
    }

    pub fn is_out_of_bounds(&self, block: &Block) -> bool {
        !block
            .iter_positions()
            .all(|pos| 0 <= pos.0 && pos.0 < self.width && 0 <= pos.1 && pos.1 < self.height)
    }

    pub fn is_colliding(&self, block: &Block) -> bool {
        self.fixed_block
            .iter()
            .any(|fixed_block| fixed_block.collides_with(block))
    }

    fn remove_line(&mut self, y: i32) {
        for block in self.fixed_block.iter_mut() {
            block.remove_line(y);
        }
    }

    fn remove_full_lines(&mut self) {
        for y in 0..self.height {
            if self.is_line_full(y) {
                self.remove_line(y);
            }
        }
    }

    pub fn tick(&mut self) {
        if self.is_lost {
            return;
        }

        let translated_curr_block = &self.curr_block + Pos(0, 1);

        if self.is_out_of_bounds(&translated_curr_block)
            || self.is_colliding(&translated_curr_block)
        {
            // Make current block fixed

            let new_fixed_block = mem::replace(
                &mut self.curr_block,
                &Block::new_rand() + Pos((self.width - 1) / 2, 0),
            );

            self.fixed_block.push(new_fixed_block);
            self.remove_full_lines();

            if self.is_colliding(&self.curr_block) {
                self.is_lost = true;
            }
        } else {
            self.curr_block = translated_curr_block;
        }
    }

    pub fn shift(&mut self, direction: Direction) {
        if self.is_lost {
            return;
        }

        let translated_curr_block = &self.curr_block
            + match direction {
                Direction::Left => Pos(-1, 0),
                Direction::Right => Pos(1, 0),
            };

        if !self.is_out_of_bounds(&translated_curr_block)
            && !self.is_colliding(&translated_curr_block)
        {
            self.curr_block = translated_curr_block;
        }
    }

    pub fn rotate(&mut self) {
        if self.is_lost {
            return;
        }

        let rotated_curr_block = self.curr_block.rotated();

        if !self.is_out_of_bounds(&rotated_curr_block) && !self.is_colliding(&rotated_curr_block) {
            self.curr_block = rotated_curr_block;
        }
    }
}

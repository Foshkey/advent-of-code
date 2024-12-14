use std::cmp::Ordering;

pub struct Drive {
    blocks: Vec<Block>,
}

impl Drive {
    pub fn compress(&mut self) {
        loop {
            // Take right-most block
            let Some(block) = self.blocks.pop() else {
                break;
            };

            // If it doesn't have an id then just continue
            let Some(id) = block.id else {
                continue;
            };

            // Take the first empty block
            let Some((index, empty_block)) = self.take_empty_block() else {
                // If no empty block then put the id block back and break out.
                self.blocks.push(block);
                break;
            };

            match block.len.cmp(&empty_block.len) {
                Ordering::Less => {
                    // Then we want to insert the entire id block followed by remaining empty
                    let remaining = empty_block.len - block.len;
                    let new_empty = Block {
                        id: None,
                        len: remaining,
                    };
                    self.blocks.insert(index, new_empty);
                    self.blocks.insert(index, block);
                }
                Ordering::Greater => {
                    // Then we want to split up the id block.
                    let replacement_block = Block {
                        id: Some(id),
                        len: empty_block.len,
                    };
                    let new_block = Block {
                        id: Some(id),
                        len: block.len - empty_block.len,
                    };
                    self.blocks.insert(index, replacement_block);
                    self.blocks.push(new_block);
                }
                Ordering::Equal => {
                    // If equal, then just replace.
                    self.blocks.insert(index, block);
                }
            }
        }
    }

    pub fn get_checksum(&self) -> u128 {
        let mut position = 0;
        let mut sum = 0;

        for block in self.blocks.iter() {
            for _ in 0..block.len {
                if let Some(id) = block.id {
                    sum += id as u128 * position;
                }
                position += 1
            }
        }

        sum
    }

    fn take_empty_block(&mut self) -> Option<(usize, Block)> {
        let (index, _) = self
            .blocks
            .iter()
            .enumerate()
            .find(|(_, block)| block.id.is_none())?;

        let block = self.blocks.remove(index);

        Some((index, block))
    }
}

impl From<&str> for Drive {
    fn from(s: &str) -> Self {
        let mut drive = Drive { blocks: Vec::new() };
        let mut id = 0;
        let mut is_file = true;

        for c in s.chars() {
            let n = c.to_string().parse().unwrap();
            let block_id = if is_file {
                let block_id = id;
                id += 1;
                Some(block_id)
            } else {
                None
            };

            drive.blocks.push(Block {
                id: block_id,
                len: n,
            });

            is_file = !is_file;
        }

        drive
    }
}

struct Block {
    id: Option<usize>,
    len: usize,
}

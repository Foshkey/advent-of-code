use std::cmp::Ordering;

pub struct Drive {
    blocks: Vec<Block>,
    next_id: usize,
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
            let Some((index, empty_block)) = self.take_empty_block(1, self.blocks.len()) else {
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

    pub fn compress_whole(&mut self) {
        let mut id = self.next_id;

        while id > 0 {
            id -= 1;

            let Some((mut block_index, block)) = self.find_block(id) else {
                break;
            };

            let Some((empty_index, empty_block)) = self.take_empty_block(block.len, block_index)
            else {
                continue;
            };

            // Subtract 1 from the block index because we just took an empty block
            block_index -= 1;

            // Remove the block and replace it with empty space
            let block = self.blocks.remove(block_index);
            self.blocks.insert(
                block_index,
                Block {
                    id: None,
                    len: block.len,
                },
            );

            // Insert new empty block
            let new_empty_block = Block {
                id: None,
                len: empty_block.len - block.len,
            };
            self.blocks.insert(empty_index, new_empty_block);

            // And finally insert the block
            self.blocks.insert(empty_index, block);
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

    fn take_empty_block(&mut self, min_len: usize, max_index: usize) -> Option<(usize, Block)> {
        let (index, _) = self.blocks.iter().enumerate().find(|(index, block)| {
            block.id.is_none() && block.len >= min_len && *index < max_index
        })?;

        let block = self.blocks.remove(index);

        Some((index, block))
    }

    fn find_block(&self, id: usize) -> Option<(usize, &Block)> {
        for (index, block) in self.blocks.iter().enumerate().rev() {
            if block.id == Some(id) {
                return Some((index, block));
            }
        }

        None
    }
}

impl From<&str> for Drive {
    fn from(s: &str) -> Self {
        let mut blocks = Vec::new();
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

            blocks.push(Block {
                id: block_id,
                len: n,
            });

            is_file = !is_file;
        }

        Drive {
            blocks,
            next_id: id,
        }
    }
}

struct Block {
    id: Option<usize>,
    len: usize,
}

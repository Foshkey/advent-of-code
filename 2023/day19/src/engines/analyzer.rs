use anyhow::Result;

use crate::models::sequence::Sequence;

pub fn count_distinct_combinations(sequence: &Sequence) -> Result<u64> {
    Ok(sequence.workflows.len() as u64)
}

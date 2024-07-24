use anyhow::{anyhow, bail, Result};

use crate::models::{
    part::Part,
    sequence::{Sequence, Workflow},
    step::{Step, StepResult},
};

pub fn run(sequence: &Sequence) -> Result<u64> {
    Ok(sequence
        .parts
        .iter()
        .filter_map(|part| {
            let result = run_part(part, sequence);
            match result {
                Ok(StepResult::Accept) => Some(Ok(part.total_rating() as u64)),
                Ok(StepResult::Reject) => None,
                Ok(StepResult::Continue(_)) => Some(Err(anyhow!(
                    "Invalid result for part {:?}: {:?}",
                    part,
                    result
                ))),
                Err(error) => Some(Err(error)),
            }
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum())
}

fn run_part(part: &Part, sequence: &Sequence) -> Result<StepResult> {
    let mut result = StepResult::Continue("in".to_string());

    while let StepResult::Continue(next) = result {
        let workflow = sequence
            .workflows
            .get(&next)
            .ok_or(anyhow!("Could not find workflow: {}", next))?;
        result = run_part_through_workflow(part, workflow)?;
    }

    Ok(result)
}

fn run_part_through_workflow(part: &Part, workflow: &Workflow) -> Result<StepResult> {
    for step in workflow {
        match step {
            Step::Compare(variable, operator, value, result) => {
                let part_value = part.ratings.get(variable).unwrap_or(&0);
                if &part_value.cmp(value) == operator {
                    return Ok(result.clone());
                }
            }
            Step::Final(result) => return Ok(result.clone()),
        }
    }

    bail!(
        "No result found with part {:?} and workflow {:?}",
        part,
        workflow
    )
}

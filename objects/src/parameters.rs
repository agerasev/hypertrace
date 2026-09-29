//! Packed child payloads shared by material combinators.
//!
//! A component's prefix is followed by one relative start offset per child and
//! a final end offset. Equal adjacent offsets represent zero-word materials.
use crate::shader::{Geometry, Result, SourceModule};
use std::convert::TryFrom;

fn header_length(prefix: usize, children: usize) -> Result<usize> {
    prefix
        .checked_add(children)
        .and_then(|n| n.checked_add(1))
        .ok_or_else(|| anyhow::anyhow!("component parameter header overflows"))
}

/// Known total size when every child is fixed-size; all layouts use offsets.
pub(crate) fn schema_size<G: Geometry>(
    prefix: usize,
    children: &[SourceModule<G>],
) -> Result<Option<u32>> {
    let mut minimum = u32::try_from(header_length(prefix, children.len())?)?;
    let mut fixed = true;
    for child in children {
        match child.parameter_words {
            Some(length) => {
                minimum = minimum
                    .checked_add(length)
                    .ok_or_else(|| anyhow::anyhow!("component parameters overflow"))?;
            }
            None => fixed = false,
        }
    }
    Ok(if fixed { Some(minimum) } else { None })
}

pub(crate) fn pack(mut prefix: Vec<u32>, children: &[Vec<u32>]) -> Result<Vec<u32>> {
    let table = prefix.len();
    let header = header_length(table, children.len())?;
    let total = children.iter().try_fold(header, |total, child| {
        total
            .checked_add(child.len())
            .ok_or_else(|| anyhow::anyhow!("component parameters overflow"))
    })?;
    // Validate the complete address range before allocating or writing offsets.
    let end = u32::try_from(total)?;
    prefix.resize(header, 0);
    for (index, child) in children.iter().enumerate() {
        prefix[table + index] = u32::try_from(prefix.len())?;
        prefix.extend(child);
    }
    prefix[table + children.len()] = end;
    Ok(prefix)
}

/// Validate the complete table before returning any child payload slices.
pub(crate) fn slices(words: &[u32], prefix: usize, children: usize) -> Result<Vec<&[u32]>> {
    let header = header_length(prefix, children)?;
    anyhow::ensure!(words.len() >= header, "child offset table is truncated");
    let end = u32::try_from(words.len())?;
    let offsets = &words[prefix..header];
    anyhow::ensure!(
        offsets[0] as usize == header,
        "child payload must start after its offset table"
    );
    anyhow::ensure!(
        offsets[children] == end,
        "child payload end does not match its parameter length"
    );
    let mut result = Vec::with_capacity(children);
    for pair in offsets.windows(2) {
        let (start, end) = (pair[0] as usize, pair[1] as usize);
        anyhow::ensure!(
            start >= header && start <= end && end <= words.len(),
            "invalid child payload range"
        );
        result.push(&words[start..end]);
    }
    Ok(result)
}

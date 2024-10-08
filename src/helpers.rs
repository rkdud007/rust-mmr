use anyhow::{anyhow, Result};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug)]
pub enum TreeMetadataKeys {
    LeafCount,
    ElementCount,
    RootHash,
}

/// Append Result
#[derive(Debug, PartialEq, Eq)]
pub struct AppendResult {
    pub leaves_count: usize,
    pub elements_count: usize,
    /// The index of the appended element.
    pub element_index: usize,
    pub root_hash: String,
}

pub fn find_peaks(mut elements_count: usize) -> Vec<usize> {
    let mut mountain_elements_count = (1 << bit_length(elements_count)) - 1;
    let mut mountain_index_shift = 0;
    let mut peaks: Vec<usize> = Vec::new();

    while mountain_elements_count > 0 {
        if mountain_elements_count <= elements_count {
            mountain_index_shift += mountain_elements_count;
            peaks.push(mountain_index_shift);
            elements_count -= mountain_elements_count;
        }
        mountain_elements_count >>= 1;
    }

    if elements_count > 0 {
        return Vec::new();
    }

    peaks
}

pub fn leaf_count_to_append_no_merges(leaf_count: usize) -> usize {
    count_trailing_ones(leaf_count)
}
fn count_trailing_ones(mut num: usize) -> usize {
    let mut count = 0;
    while num != 0 && num & 1 == 1 {
        num >>= 1;
        count += 1;
    }
    count
}

pub fn find_siblings(element_index: usize, elements_count: usize) -> Result<Vec<usize>> {
    let mut leaf_index = element_index_to_leaf_index(element_index)?;
    let mut height = 0;
    let mut siblings = Vec::new();
    let mut current_element_index = element_index;

    while current_element_index <= elements_count {
        let siblings_offset = (2 << height) - 1;
        if leaf_index % 2 == 1 {
            // right child
            siblings.push(current_element_index - siblings_offset);
            current_element_index += 1;
        } else {
            // left child
            siblings.push(current_element_index + siblings_offset);
            current_element_index += siblings_offset + 1;
        }
        leaf_index /= 2;
        height += 1;
    }

    siblings.pop();
    Ok(siblings)
}

pub fn element_index_to_leaf_index(element_index: usize) -> Result<usize> {
    if element_index == 0 {
        return Err(anyhow!("Invalid element index"));
    }
    match elements_count_to_leaf_count(element_index - 1) {
        Ok(val) => Ok(val),
        Err(_) => Err(anyhow!("Invalid element index")),
    }
}

pub fn elements_count_to_leaf_count(elements_count: usize) -> Result<usize> {
    let mut leaf_count = 0;
    let mut mountain_leaf_count = 1 << bit_length(elements_count);
    let mut current_elements_count = elements_count;

    while mountain_leaf_count > 0 {
        let mountain_elements_count = 2 * mountain_leaf_count - 1;
        if mountain_elements_count <= current_elements_count {
            leaf_count += mountain_leaf_count;
            current_elements_count -= mountain_elements_count;
        }
        mountain_leaf_count >>= 1;
    }

    if current_elements_count > 0 {
        return Err(anyhow!("Invalid elements count"));
    }

    Ok(leaf_count)
}

fn bit_length(num: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - num.leading_zeros() as usize
}

pub fn array_deduplicate<T: Eq + Hash>(array: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = array.into_iter().collect();
    set.into_iter().collect::<Vec<T>>()
}

pub fn get_peak_info(mut elements_count: usize, mut element_index: usize) -> (usize, usize) {
    let mut mountain_height = bit_length(elements_count);
    let mut mountain_elements_count = (1 << mountain_height) - 1;
    let mut mountain_index = 0;

    loop {
        if mountain_elements_count <= elements_count {
            if element_index <= mountain_elements_count {
                return (mountain_index, mountain_height - 1);
            }
            elements_count -= mountain_elements_count;
            element_index -= mountain_elements_count;
            mountain_index += 1;
        }
        mountain_elements_count >>= 1;
        mountain_height -= 1;
    }
}

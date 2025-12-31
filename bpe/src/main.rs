use std::collections::HashMap;
use std::fs::read;
use std::vec::Vec;

fn main() -> std::io::Result<()> {
    let data = read("src/input.txt")?;
    let mut ids: Vec<u32> = data.into_iter().map(|b| b as u32).collect();
    let mut merges: HashMap<u32, (u32, u32)> = HashMap::new();

    // Create a counter for new IDs
    let mut next_id = 256;
    let num_merges = 10; // Let's try 10 merges to start

    for iteration in 0..num_merges {
        // we MUST clear/re-create the map every time because the pairs changed!
        let mut counts = HashMap::new();
        for pair in ids.windows(2) {
            *counts.entry((pair[0], pair[1])).or_insert(0) += 1;
        }

        // Find the best pair for THIS round
        let (most_common_pair, _) = match counts.iter().max_by_key(|&(_, count)| count) {
            Some(res) => res,
            None => break, // Stop if no pairs left
        };

        // Perform the merge
        let mut new_ids = Vec::with_capacity(ids.len());
        let mut i = 0;
        while i < ids.len() {
            if i < ids.len() - 1 && ids[i] == most_common_pair.0 && ids[i + 1] == most_common_pair.1
            {
                new_ids.push(next_id); // Use our counter!
                merges.insert(next_id, *most_common_pair);
                i += 2;
            } else {
                new_ids.push(ids[i]);
                i += 1;
            }
        }

        println!(
            "Iter {}: Merged {:?} into ID {}. Len: {}. Merges: {:#?}",
            iteration,
            most_common_pair,
            next_id,
            new_ids.len(),
            merges
        );

        // Update for next round
        ids = new_ids;
        next_id += 1;
    }

    Ok(())
}

use std::collections::HashMap;
use std::fs::read;
use std::vec::Vec;

pub struct Tokenizer {
    pub merges: HashMap<(u32, u32), u32>,
    pub vocab: HashMap<u32, Vec<u8>>,
}

impl Tokenizer {
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        for i in 0..256 {
            vocab.insert(i as u32, vec![i as u8]);
        }
        Self {
            merges: HashMap::new(),
            vocab,
        }
    }
}

fn main() -> std::io::Result<()> {
    let data = read("src/input.txt")?;
    let mut ids: Vec<u32> = data.into_iter().map(|b| b as u32).collect();
    let mut merges: HashMap<(u32, u32), u32> = HashMap::new();
    let mut vocab: HashMap<u32, Vec<u8>> = HashMap::new();

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
                merges.insert(*most_common_pair, next_id);
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

    for i in 0..256 {
        vocab.insert(i as u32, vec![i as u8]);
    }
    // build vocab
    let mut sorted_merges: Vec<_> = merges.iter().collect();
    sorted_merges.sort_by_key(|(key, _)| key.0);

    for ((p1, p2), new_id) in sorted_merges {
        let mut combined = vocab.get(p1).unwrap().clone();
        combined.extend(vocab.get(p2).unwrap());
        vocab.insert(*new_id, combined);
    }

    // print the result
    println!("--- TOP LEARNED TOKENS ---");
    for i in 256..next_id {
        if let Some(bytes) = vocab.get(&i) {
            let s = String::from_utf8_lossy(bytes);
            println!("ID {}: {:?}", i, s);
        }
    }

    // encode the text
    let encoded = encode("thin", &merges);
    println!("Encoded: {:?}", encoded);
    println!("Decoded: {:?}", decode(&encoded, &vocab));
    println!("Original: {:?}", "thin");

    Ok(())
}

fn encode(text: &str, merges: &HashMap<(u32, u32), u32>) -> Vec<u32> {
    // convert the input text into a vector of u32
    let mut ids: Vec<u32> = text.as_bytes().iter().map(|&b| b as u32).collect();

    println!("Original: {:?}", text);

    // order the rules by their id with respect to new id and asecnding order
    let mut rules: Vec<_> = merges.iter().collect();
    rules.sort_by_key(|&(_, &id)| id);

    for (pair, new_id) in rules {
        let mut compressed = Vec::new();
        let mut i = 0;

        while i < ids.len() {
            if i < ids.len() - 1 && ids[i] == pair.0 && ids[i + 1] == pair.1 {
                compressed.push(*new_id);
                i += 2;
            } else {
                compressed.push(ids[i]);
                i += 1;
            }
        }
        ids = compressed;
    }
    ids
}

pub fn decode(ids: &[u32], vocab: &HashMap<u32, Vec<u8>>) -> String {
    let mut bytes = Vec::new();
    for id in ids {
        if let Some(token_bytes) = vocab.get(id) {
            bytes.extend(token_bytes);
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}

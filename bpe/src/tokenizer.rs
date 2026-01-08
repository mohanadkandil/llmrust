use std::collections::HashMap;
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

    pub fn train_from_bytes(&mut self, data: &[u8], num_merges: usize) {
        // Start ids as the raw bytes (0..255) promoted to u32
        let mut ids: Vec<u32> = data.iter().map(|&b| b as u32).collect();

        let mut next_id: u32 = 256;

        for _ in 0..num_merges {
            // Count neighbor pairs
            let mut counts: HashMap<(u32, u32), u32> = HashMap::new();
            for pair in ids.windows(2) {
                *counts.entry((pair[0], pair[1])).or_insert(0) += 1;
            }

            // Find most frequent pair
            let Some((&best_pair, _best_count)) = counts.iter().max_by_key(|&(_, c)| c) else {
                break;
            };

            // Record merge rule (pair -> new token id)
            let new_id = next_id;
            self.merges.insert(best_pair, new_id);

            // Replace occurrences in ids
            let mut new_ids = Vec::with_capacity(ids.len());
            let mut i = 0;
            while i < ids.len() {
                if i + 1 < ids.len() && ids[i] == best_pair.0 && ids[i + 1] == best_pair.1 {
                    new_ids.push(new_id);
                    i += 2;
                } else {
                    new_ids.push(ids[i]);
                    i += 1;
                }
            }

            ids = new_ids;
            next_id += 1;
        }

        self.rebuild_vocab();
    }

    fn rebuild_vocab(&mut self) {
        // reset base vocab 0..255
        self.vocab.clear();
        for i in 0..256u32 {
            self.vocab.insert(i, vec![i as u8]);
        }

        // apply merges in the order learned (ascending new_id)
        let mut rules: Vec<((u32, u32), u32)> =
            self.merges.iter().map(|(p, id)| (*p, *id)).collect();
        rules.sort_by_key(|&(_, id)| id);

        for ((p1, p2), new_id) in rules {
            let mut combined = self.vocab[&p1].clone();
            combined.extend(self.vocab[&p2].iter().copied());
            self.vocab.insert(new_id, combined);
        }
    }

    pub fn encode(&self, text: &str) -> Vec<u32> {
        let mut ids: Vec<u32> = text.as_bytes().iter().map(|&b| b as u32).collect();

        // apply merges in the order learned
        let mut rules: Vec<((u32, u32), u32)> =
            self.merges.iter().map(|(p, id)| (*p, *id)).collect();
        rules.sort_by_key(|&(_, id)| id);

        for (pair, new_id) in rules {
            let mut out = Vec::with_capacity(ids.len());
            let mut i = 0;
            while i < ids.len() {
                if i + 1 < ids.len() && ids[i] == pair.0 && ids[i + 1] == pair.1 {
                    out.push(new_id);
                    i += 2;
                } else {
                    out.push(ids[i]);
                    i += 1;
                }
            }
            ids = out;
        }

        ids
    }

    pub fn decode(&self, ids: &[u32]) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        for &id in ids {
            if let Some(token_bytes) = self.vocab.get(&id) {
                bytes.extend(token_bytes.iter().copied());
            }
        }
        String::from_utf8_lossy(&bytes).to_string()
    }
}

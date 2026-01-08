use rand::random;

pub struct EmbeddingModel {
    pub vocab_size: usize,
    pub dim: usize,
    pub table: Vec<f32>,
}

impl EmbeddingModel {
    pub fn new(vocab_size: usize, dim: usize) -> Self {
        // create vector with enough space
        let mut table = Vec::with_capacity(vocab_size * dim);

        // fill in with random nums
        for _ in 0..vocab_size * dim {
            let random_num = (random::<f32>() * 2.0) - 1.0;
            table.push(random_num);
        }

        Self {
            vocab_size,
            dim,
            table,
        }
    }

    // Look up the vector for a given id
    pub fn get_vector(&self, id: u32) -> &[f32] {
        let start = (id as usize) * self.dim;
        let end = start + self.dim;
        &self.table[start..end]
    }

    pub fn similarity(&self, vec1: &[f32], vec2: &[f32]) -> f32 {
        let dot: f32 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();
        let mag1: f32 = vec1.iter().map(|x| x * x).sum();
        let mag2: f32 = vec2.iter().map(|x| x * x).sum();
        dot / (mag1 * mag2).sqrt()
    }

    pub fn find_closest(&self, target_vec: &[f32], target_id: u32) -> Vec<(u32, f32)> {
        let mut best_id = f32::NEG_INFINITY;
        let mut best_sim = 0.0;
        let mut top5_ids: Vec<(u32, f32)> = Vec::with_capacity(5);

        for id in 0..self.vocab_size {
            // loop throught all ids and find the id with the highest similarity
            let current_id = id as u32;

            // skip the current id
            if target_id == current_id {
                continue;
            }

            let sim = self.similarity(target_vec, self.get_vector(current_id));
            if sim > best_sim {
                best_sim = sim;
                top5_ids.push((current_id, sim));
                if top5_ids.len() < 5 {
                    continue;
                }
            }
            top5_ids.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        }
        top5_ids
    }
}

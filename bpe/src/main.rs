mod embedding;
mod tokenizer;

use embedding::EmbeddingModel;
use std::fs::read;
use tokenizer::Tokenizer;

fn main() -> std::io::Result<()> {
    let data = read("src/input.txt")?;

    let mut tok = Tokenizer::new();
    tok.train_from_bytes(&data, 500);

    let text = "romans";
    let encoded = tok.encode(text);

    // Increase dimensions for better "resolution"
    let dim = 64;
    let embedding = EmbeddingModel::new(tok.vocab.len(), dim);

    // Get the vector for the first token of "hello"
    let target_id = encoded[0];
    let target_vec = embedding.get_vector(target_id);

    // Find the top 5 closest
    let closest_results = embedding.find_closest(target_vec, target_id);

    println!(
        "Searching for words similar to: '{}'",
        tok.decode(&[encoded[0]])
    );
    println!("-------------------------------------------");

    for (id, score) in closest_results {
        let word = tok.decode(&[id]);

        println!("Match: {:<10} | Score: {:.4}", word, score);
    }

    Ok(())
}

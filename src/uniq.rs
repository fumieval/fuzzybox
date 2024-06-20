use std::collections::HashMap;

use crate::union_find;
use tokio::io::AsyncBufReadExt;

pub async fn uniq(api_key: &str, threshold: f32, dot_path: Option<std::path::PathBuf>) {
    // read lines from stdin in a streaming fashion
    let stdin = tokio::io::stdin();
    let mut lines = tokio::io::BufReader::new(stdin).lines();

    let client = openai_rust::Client::new(api_key);

    let mut buffer = Vec::new();
    while let Ok(line) = lines.next_line().await {
        match line {
            Some(text) => buffer.push(text),
            None => break,
        };
    }

    let all_words = buffer.clone();

    buffer.sort();
    buffer.dedup();

    let embeddings = compute(&client, &buffer).await;

    let mut union_find = union_find::UnionFind::new();
    let mut counters = HashMap::new();

    for i in 0..buffer.len() {
        for j in i + 1..buffer.len() {
            let similarity = dot_product(&embeddings[i], &embeddings[j]);
            if similarity > threshold {
                union_find.union(buffer[i].clone(), buffer[j].clone());
            }
        }
    }

    if let Some(dot_path) = dot_path {
        let dot = union_find.to_dot();
        std::fs::write(dot_path, dot).unwrap();
    }

    for item in all_words {
        let root = union_find.find(item.clone());
        let count = counters.entry(root.clone()).or_insert(0);
        *count += 1;
    }

    // print the results, descending order of count
    let mut results: Vec<_> = counters.into_iter().collect();
    results.sort_by_key(|(_, count)| -count);
    for (item, count) in results {
        println!("{} {}", item, count);
    }
}

// calculate the dot product of two vectors
fn dot_product(vec1: &[f32], vec2: &[f32]) -> f32 {
    assert_eq!(vec1.len(), vec2.len(), "Vector lengths must be equal");

    let mut result = 0.0;
    for i in 0..vec1.len() {
        result += vec1[i] * vec2[i];
    }

    result
}

async fn compute(client: &openai_rust::Client, texts: &[String]) -> Vec<Vec<f32>> {
    let args = openai_rust::embeddings::EmbeddingsArguments::new(
        "text-embedding-3-large",
        texts.iter().map(|s| s.to_owned()).collect(),
    );
    client
        .create_embeddings(args)
        .await
        .unwrap()
        .data
        .into_iter()
        .map(|data| data.embedding)
        .collect()
}

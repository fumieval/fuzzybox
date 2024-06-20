use std::fmt::Display;
use std::{collections::HashMap, hash::Hash};

pub struct UnionFind<K> {
    parent: HashMap<K, K>,
    rank: HashMap<K, u32>,
    weight: HashMap<K, f32>,
}

impl<K: PartialEq + Eq + Hash + Clone + Display> UnionFind<K> {
    pub fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
            weight: HashMap::new(),
        }
    }

    pub fn find(&mut self, k: K) -> K {
        if !self.parent.contains_key(&k) {
            self.parent.insert(k.clone(), k.clone());
            self.rank.insert(k.clone(), 0);
            self.weight.insert(k.clone(), 1.0);
        }

        let parent: K = self.parent[&k].clone();
        if parent != k {
            let new_parent = self.find(parent.clone()).clone();
            self.parent.insert(k.clone(), new_parent);
            self.weight
                .insert(k.clone(), self.weight[&k] + self.weight[&parent]);
        }

        parent
    }

    pub fn weight(&mut self, k: K) -> f32 {
        self.find(k.clone());
        self.weight[&k]
    }

    pub fn union(&mut self, x: K, y: K) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        if self.rank[&x_root] < self.rank[&y_root] {
            self.parent.insert(x_root.clone(), y_root.clone());
        } else if self.rank[&x_root] > self.rank[&y_root] {
            self.parent.insert(y_root.clone(), x_root.clone());
        } else {
            self.parent.insert(y_root.clone(), x_root.clone());
            self.rank.insert(x_root.clone(), self.rank[&x_root] + 1);
        }
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        for (k, v) in &self.parent {
            if k == v {
                continue;
            }
            dot.push_str(&format!("\"{}\" -> \"{}\"\n", k, v));
        }
        dot.push_str("}\n");
        dot
    }
}

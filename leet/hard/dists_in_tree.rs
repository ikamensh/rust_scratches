// https://leetcode.com/problems/sum-of-distances-in-tree/submissions/

use std::collections::{HashMap, HashSet, VecDeque};
use std::cell::RefCell;

struct Graph {
    adj: HashMap<i32, Vec<i32>>,
    _count_via: RefCell<HashMap<(i32, i32), i32>>,
    _dist_via: RefCell<HashMap<(i32, i32), i32>>,
}

impl Graph {
    pub fn add_edge(&mut self, v: i32, u: i32) {
        self.adj.entry(v).or_insert(vec![]).push(u);
        self.adj.entry(u).or_insert(vec![]).push(v);
    }

    fn count_via(&self, source: i32, neighb: i32) -> i32 {
        let key = (source, neighb);
        if self._count_via.borrow().contains_key(&key) {
            return self._count_via.borrow()[&key];
        }
        let mut result = 1;
        for next in &self.adj[&neighb] {
            if *next == source { continue; }
            result += self.count_via(neighb, *next);
        }

        self._count_via.borrow_mut().insert(key, result);
        result
    }

    pub fn dist_via(&self, source: i32, neighb: i32) -> i32 {
        let key = (source, neighb);
        if self._dist_via.borrow().contains_key(&key) {
            return self._dist_via.borrow()[&key];
        }
        let mut result = 0;
        for next in &self.adj[&neighb] {
            if *next == source { continue; }
            result += self.dist_via(neighb, *next);
        }
        result += self.count_via(source, neighb);

        self._dist_via.borrow_mut().insert(key, result);
        result
    }
}


struct Solution {}

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }
        let mut graph = Graph {
            adj: HashMap::<i32, Vec<i32>>::new(),
            _count_via: RefCell::new(HashMap::new()),
            _dist_via: RefCell::new(HashMap::new())
        };

        for edge in edges {
            let mut it = edge.into_iter();
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            graph.add_edge(first, second);
        }

        let mut result = vec![];

        for i in 0..n {
            let mut dist = 0;
            for neighb in &graph.adj[&i]{
                dist += graph.dist_via(i, *neighb);
            }
            result.push(dist);
        }

        result
    }
}

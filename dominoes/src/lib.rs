#![allow(dead_code)]

use std::{
    collections::{HashMap, HashSet},
    ops::Rem,
};

struct Domino(u8, u8);

impl Domino {
    fn new(a: u8, b: u8) -> Self { Domino(a, b) }
}

pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    let mut edges = vec![];
    let mut pairs = HashSet::new();
    let mut nodes = HashMap::<u8, Vec<u8>>::new();

    for &(r, l) in input {
        (*nodes.entry(r).or_default()).push(l);
        (*nodes.entry(l).or_default()).push(r);
    }

    println!("NODES: {nodes:?}");

    for (&node, connections) in nodes.iter() {
        let count = connections.len();
        if count.eq(&2) {
            pairs.insert(node);
        } else if count.rem(&2) != 0 {
            edges.push(node);
        }
    }

    println!("EDGES: {edges:?}");
    if edges.len() != 2 {
        return None;
    }

    let mut chained = vec![];
    let root = edges.first().unwrap();
    let next_options = nodes.get_mut(&root).unwrap();

    let r = root;
    let domis =
        next_options
            .iter()
            .fold(Vec::<(u8, u8)>::new(), |mut acc, curr| {
                acc.push((*r, *curr));
                acc
            });

    for next in domis {
        if let Some(result) =
            _chain(Some(next), next_options, &mut chained, input.len() - 1)
        {
            println!("RESULT: {result:#?}");
            println!("CHAINED: {chained:?}");
        }
    }

    Some(chained)
}

fn _chain(
    edge: Option<(u8, u8)>,
    next: &mut Vec<u8>,
    chain: &mut Vec<(u8, u8)>,
    pieces: usize,
) -> Option<(u8, u8)> {
    println!("EDGE: {edge:?}");
    println!("NEXT: {next:?}");
    println!("CHAIN: {chain:?}");

    if edge.is_none() || pieces.eq(&0) {
        return edge;
    }

    let edge_tail = edge.unwrap().1;

    if next.is_empty() {
        return None;
    }

    let mut idx = next.len() - 1;
    let mut prev = idx;
    while idx != 0 {
        if prev != idx {
            next.push(next.get(prev).unwrap().clone());
            prev = idx;
        }

        let curr = next.remove(idx);
        idx -= 1;

        println!("EDGE: {edge:?}");
        println!("NEXT: {next:?}");
        println!("CHAIN: {chain:?}");
        if let Some(result) =
            _chain(Some((edge_tail, curr)), next, chain, pieces - 1)
        {
            chain.push(edge.unwrap());

            return Some(result);
        }
    }

    None
}

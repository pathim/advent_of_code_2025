use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use crate::{AocInput, AocResult};

type Node = (i64, i64, i64);

fn dist2(&n1: &Node, n2: &Node) -> i64 {
    let dx = n1.0 - n2.0;
    let dy = n1.1 - n2.1;
    let dz = n1.2 - n2.2;
    dx * dx + dy * dy + dz * dz
}

fn make_node(s: &str) -> Node {
    let v = s.split(',').map(|a| a.parse().unwrap()).collect::<Vec<_>>();
    (v[0], v[1], v[2])
}

pub fn f(input: AocInput) -> AocResult {
    let nodes = input
        .lines()
        .flatten()
        .map(|l| make_node(&l))
        .collect::<Vec<_>>();
    let mut node_networks = (0..nodes.len()).enumerate().collect::<HashMap<_, _>>();
    let mut network_nodes = (0..nodes.len())
        .map(|i| (i, vec![i]))
        .collect::<HashMap<_, _>>();
    let mut dists = BinaryHeap::new();
    for i in 0..nodes.len() - 1 {
        for j in i + 1..nodes.len() {
            dists.push(Reverse((dist2(&nodes[i], &nodes[j]), (i, j))));
        }
    }
    let num_nodes = nodes.len();
    let mut res1 = 0;
    let mut res2 = 0;
    for i in 0.. {
        let Reverse((_d, idx)) = dists.pop().unwrap();
        let new_net = node_networks[&idx.0];
        let old_net = node_networks[&idx.1];
        if new_net != old_net {
            let [old_nodes, new_nodes] = network_nodes.get_disjoint_mut([&old_net, &new_net]);
            let old_nodes = old_nodes.unwrap();
            let new_nodes = new_nodes.unwrap();

            for n in old_nodes.drain(..) {
                new_nodes.push(n);
                *node_networks.get_mut(&n).unwrap() = new_net;
            }
            if new_nodes.len() == num_nodes {
                res2 = nodes[idx.0].0 * nodes[idx.1].0;
                break;
            }
        }
        if i == 999 {
            let mut sorted = network_nodes.iter().map(|n| n.1.len()).collect::<Vec<_>>();
            sorted.sort_unstable();
            let l = sorted.len();
            res1 = sorted[l - 1] * sorted[l - 2] * sorted[l - 3];
        }
    }

    (res1, res2).into()
}

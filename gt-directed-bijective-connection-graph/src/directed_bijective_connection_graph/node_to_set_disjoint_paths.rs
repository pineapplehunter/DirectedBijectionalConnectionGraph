use crate::{DirectedBijectiveConnectionGraph, SinglePath};
use gt_graph::Node;
use gt_graph_path::GraphPath;

pub trait NodeToSetDisjointPaths {
    fn node_to_set_disjoint_paths(&self, s: Node, d: &[Node]) -> Vec<GraphPath>;
}

impl<F> NodeToSetDisjointPaths for F
where
    F: DirectedBijectiveConnectionGraph + SinglePath,
{
    fn node_to_set_disjoint_paths(&self, s: Node, d: &[Node]) -> Vec<GraphPath> {
        assert!(d.len() <= self.dimension() as usize);
        assert_ne!(d.len(), 0);

        if d.len() == 1 {
            if d[0] == s {
                let mut tmp = GraphPath::new(self);
                tmp.push(s);
                return vec![tmp];
            } else {
                let mut tmp = GraphPath::new(self);
                tmp.push(s);
                tmp.push(s ^ 1);
                return vec![tmp];
            }
        }

        let mut paths;

        let dim = d.len() as u64;
        let mask = 1 << (dim - 1);

        let all_on_same_side_as_src = d
            .iter()
            .map(|node| *node & mask)
            .all(|node_masked| node_masked == s & mask);
        if all_on_same_side_as_src {
            paths = self.node_to_set_disjoint_paths(s, &d[..dim as usize - 1]);
            paths.push(GraphPath::new(self));

            debug_assert_eq!(paths.len(), dim as usize);

            let mut working_index = dim as usize - 1;

            for (index, path) in paths.iter().enumerate() {
                if let Some(pos) = path.iter().position(|&node| node == d[working_index]) {
                    paths[index].truncate(pos);
                    paths.swap(index, dim as usize - 1);
                    working_index = index;
                    break;
                }
            }

            let phi_s = self.phi(dim, s);
            let psi_d = self.psi(dim, d[working_index]);

            let last_path = &mut paths[working_index];
            let tmp_path = self.single_path(phi_s, psi_d);
            last_path.extend(tmp_path.iter());
            last_path.push(d[working_index]);
        } else {
            let mut same_ds = d
                .iter()
                .filter(|&node| node & mask == s & mask)
                .copied()
                .collect::<Vec<u64>>();

            let mut new_d = Vec::with_capacity(dim as usize);
            let mut tmp_paths = Vec::with_capacity(self.dimension() as usize);

            for &n in d {
                if n & mask == s & mask {
                    new_d.push(n);
                    tmp_paths.push(GraphPath::new(self));
                } else {
                    let dd = self.psi(dim, n);
                    if !same_ds.contains(&&dd) {
                        same_ds.push(dd);

                        new_d.push(dd);
                        tmp_paths.push({
                            let mut path = GraphPath::new(self);
                            path.push(n);
                            path
                        });
                    } else {
                        for i in (1..dim).rev() {
                            let dd = self.psi(i, n);
                            let ddd = self.psi(dim, dd);

                            if !same_ds.contains(&&ddd) {
                                new_d.push(ddd);
                                tmp_paths.push({
                                    let mut path = GraphPath::new(self);
                                    path.push(dd);
                                    path.push(n);
                                    path
                                });
                                break;
                            }
                        }
                    }
                }
            }

            debug_assert_eq!(new_d.len(), dim as usize);
            debug_assert_eq!(tmp_paths.len(), dim as usize);

            //dbg!(&new_d);
            //dbg!(&tmp_paths);

            let mut working_index = d.iter().position(|&node| node & mask != s & mask).unwrap();
            let dn = d[working_index];

            let mut path = GraphPath::new(self);
            let phi_s = self.phi(dim, s);
            path.push(s);
            let tmp_path = self.single_path(phi_s, dn);
            path.extend(tmp_path.iter());

            //dbg!(&path);

            'exit: for (index, node) in path.iter().enumerate() {
                for (path_index, other_path) in tmp_paths.iter().enumerate() {
                    if let Some(pos) = other_path.iter().position(|other| node == other) {
                        path.truncate(index);
                        path.extend(other_path.iter().skip(pos));
                        working_index = path_index;
                        break 'exit;
                    }
                }
            }

            tmp_paths[working_index] = GraphPath::new(self);

            new_d.swap(working_index, dim as usize - 1);
            let mut partial_paths = self.node_to_set_disjoint_paths(s, &new_d[..dim as usize - 1]);
            partial_paths.push(path);
            partial_paths.swap(working_index, dim as usize - 1);

            //dbg!(&partial_paths);

            partial_paths
                .iter_mut()
                .zip(tmp_paths.iter())
                .for_each(|(partial_path, path)| partial_path.extend(path.iter()));
            paths = partial_paths;
        }

        debug_assert_eq!(paths.len(), dim as usize);
        paths
    }
}

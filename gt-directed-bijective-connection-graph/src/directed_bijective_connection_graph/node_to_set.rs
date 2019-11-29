use crate::{DirectedBijectiveConnectionGraph, Lemma2};
use gt_graph::{Graph, InterChangeUsize};
use gt_graph_path::GraphPath;
use std::ops::{BitAnd, BitXor};

pub trait NodeToSet<G>
where
    G: Graph,
{
    #[allow(non_snake_case)]
    fn N2S(&self, s: G::Node, d: &[G::Node]) -> Vec<GraphPath<G>>;
    fn node_to_set(&self, s: G::Node, d: &[G::Node]) -> Vec<GraphPath<G>>;
}

impl<G, N, D> NodeToSet<G> for G
where
    N: Copy + PartialEq + BitAnd<Output = N> + Eq + InterChangeUsize + BitXor<Output = N>,
    D: Copy + InterChangeUsize,
    G: DirectedBijectiveConnectionGraph + Lemma2<G> + Graph<Node = N, Dims = D>,
{
    #[allow(non_snake_case)]
    #[inline(always)]
    fn N2S(&self, s: G::Node, d: &[G::Node]) -> Vec<GraphPath<G>> {
        self.node_to_set(s, d)
    }

    fn node_to_set(&self, s: G::Node, d: &[G::Node]) -> Vec<GraphPath<G>> {
        assert!(d.len() <= self.dimension().to_usize());
        assert_ne!(d.len(), 0);

        if d.len() == 1 {
            if d[0] == s {
                let mut tmp = GraphPath::new(self);
                tmp.push_back(s);
                return vec![tmp];
            } else {
                let mut tmp = GraphPath::new(self);
                tmp.push_back(s);
                tmp.push_back(s ^ InterChangeUsize::from_usize(1));
                return vec![tmp];
            }
        }

        let mut paths;

        let dim: D = InterChangeUsize::from_usize(d.len());
        let mask: N = InterChangeUsize::from_usize(1 << (dim.to_usize() - 1));

        let all_on_same_side_as_src = d
            .iter()
            .map(|node| *node & mask)
            .all(|node_masked| node_masked == s & mask);
        if all_on_same_side_as_src {
            paths = self.node_to_set(s, &d[..dim.to_usize() - 1]);
            paths.push(GraphPath::new(self));

            debug_assert_eq!(paths.len(), dim.to_usize());

            let mut working_index = dim.to_usize() - 1;

            for (index, path) in paths.iter().enumerate() {
                if let Some(pos) = path
                    .inner_path()
                    .iter()
                    .position(|&node| node == d[working_index])
                {
                    paths[index].inner_path_mut().truncate(pos);
                    paths.swap(index, dim.to_usize() - 1);
                    working_index = index;
                    break;
                }
            }

            let phi_s = self.phi(dim, s);
            let psi_d = self.psi(dim, d[working_index]);

            let last_path = &mut paths[working_index];
            last_path.push_back(s);
            self.R_helper(dim, phi_s, psi_d, last_path);
            last_path.push_back(d[working_index]);
        } else {
            let mut same_ds = d
                .iter()
                .filter(|&&node| node & mask == s & mask)
                .copied()
                .collect::<Vec<N>>();

            let mut new_d = Vec::with_capacity(dim.to_usize());
            let mut tmp_paths = Vec::with_capacity(self.dimension().to_usize());

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
                            path.push_back(n);
                            path
                        });
                    } else {
                        for i in (1..dim.to_usize()).rev() {
                            let dd = self.psi(InterChangeUsize::from_usize(i), n);
                            let ddd = self.psi(dim, dd);

                            if !same_ds.contains(&&ddd) {
                                new_d.push(ddd);
                                tmp_paths.push({
                                    let mut path = GraphPath::new(self);
                                    path.push_back(dd);
                                    path.push_back(n);
                                    path
                                });
                                break;
                            }
                        }
                    }
                }
            }

            debug_assert_eq!(new_d.len(), dim.to_usize());
            debug_assert_eq!(tmp_paths.len(), dim.to_usize());

            //dbg!(&new_d);
            //dbg!(&tmp_paths);

            let mut working_index = d
                .iter()
                .position(|&node| node & mask != s & mask.into())
                .unwrap();
            let dn = d[working_index];

            let mut path = GraphPath::new(self);
            let phi_s = self.phi(dim, s);
            path.push_back(s);
            path.push_back(phi_s);
            self.R_helper(dim, phi_s, dn, &mut path);

            //dbg!(&path);

            'exit: for (index, node) in path.inner_path().iter().enumerate() {
                for (path_index, other_path) in tmp_paths.iter().enumerate() {
                    if let Some(pos) = other_path
                        .inner_path()
                        .iter()
                        .position(|other| node == other)
                    {
                        path.inner_path_mut().truncate(index);
                        path.inner_path_mut()
                            .extend(other_path.inner_path().iter().skip(pos));
                        working_index = path_index;
                        break 'exit;
                    }
                }
            }

            tmp_paths[working_index] = GraphPath::new(self);

            new_d.swap(working_index, dim.to_usize() - 1);
            let mut partial_paths = self.node_to_set(s, &new_d[..dim.to_usize() - 1]);
            partial_paths.push(path);
            partial_paths.swap(working_index, dim.to_usize() - 1);

            //dbg!(&partial_paths);

            partial_paths
                .iter_mut()
                .zip(tmp_paths.iter())
                .for_each(|(partial_path, path)| {
                    partial_path
                        .inner_path_mut()
                        .extend(path.inner_path().iter())
                });
            paths = partial_paths;
        }

        debug_assert_eq!(paths.len(), dim.to_usize());
        paths
    }
}

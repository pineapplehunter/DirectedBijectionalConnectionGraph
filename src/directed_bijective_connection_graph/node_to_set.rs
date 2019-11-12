use crate::directed_bijective_connection_graph::lemma2::Lemma2;
use crate::node_path::NodePath;
use crate::{DirectedBijectiveConnectionGraphFunctions, Node};

pub trait NodeToSet {
    #[allow(non_snake_case)]
    fn N2S(&self, s: Node, d: &[Node]) -> Vec<NodePath>;
    fn node_to_set(&self, s: Node, d: &[Node]) -> Vec<NodePath>;
}

impl<F> NodeToSet for F
where
    F: DirectedBijectiveConnectionGraphFunctions + Lemma2,
{
    #[allow(non_snake_case)]
    #[inline(always)]
    fn N2S(&self, s: Node, d: &[Node]) -> Vec<NodePath> {
        self.node_to_set(s, d)
    }

    fn node_to_set(&self, s: Node, d: &[Node]) -> Vec<NodePath> {
        assert!(d.len() <= self.dimension() as usize);
        assert_ne!(d.len(), 0);

        if d.len() == 1 {
            if d[0] == s {
                let mut tmp = NodePath::new(self);
                tmp.push_back(s);
                return vec![tmp];
            } else {
                let mut tmp = NodePath::new(self);
                tmp.push_back(s);
                tmp.push_back(s ^ 1);
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
            paths = self.node_to_set(s, &d[..dim as usize - 1]);
            paths.push(NodePath::new(self));

            debug_assert_eq!(paths.len(), dim as usize);

            let mut working_index = dim as usize - 1;

            for (index, path) in paths.iter().enumerate() {
                if let Some(pos) = path
                    .inner_path()
                    .iter()
                    .position(|&node| node == d[working_index])
                {
                    paths[index].inner_path_mut().truncate(pos);
                    paths.swap(index, dim as usize - 1);
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
                .filter(|&node| node & mask == s & mask)
                .copied()
                .collect::<Vec<u64>>();

            let mut new_d = Vec::with_capacity(dim as usize);
            let mut tmp_paths = Vec::with_capacity(self.dimension() as usize);

            for &n in d {
                if n & mask == s & mask {
                    new_d.push(n);
                    tmp_paths.push(NodePath::new(self));
                } else {
                    let dd = self.psi(dim, n);
                    if !same_ds.contains(&&dd) {
                        same_ds.push(dd);

                        new_d.push(dd);
                        tmp_paths.push({
                            let mut path = NodePath::new(self);
                            path.push_back(n);
                            path
                        });
                    } else {
                        for i in (1..dim).rev() {
                            let dd = self.psi(i, n);
                            let ddd = self.psi(dim, dd);

                            if !same_ds.contains(&&ddd) {
                                new_d.push(ddd);
                                tmp_paths.push({
                                    let mut path = NodePath::new(self);
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

            debug_assert_eq!(new_d.len(), dim as usize);
            debug_assert_eq!(tmp_paths.len(), dim as usize);

            //dbg!(&new_d);
            //dbg!(&tmp_paths);

            let mut working_index = d.iter().position(|&node| node & mask != s & mask).unwrap();
            let dn = d[working_index];

            let mut path = NodePath::new(self);
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

            tmp_paths[working_index] = NodePath::new(self);

            new_d.swap(working_index, dim as usize - 1);
            let mut partial_paths = self.node_to_set(s, &new_d[..dim as usize - 1]);
            partial_paths.push(path);
            partial_paths.swap(working_index, dim as usize - 1);

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

        debug_assert_eq!(paths.len(), dim as usize);
        paths
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::HyperCube;
    use crate::NodeToSet;

    #[test]
    fn node_to_set() {
        let graph = HyperCube::new(4);

        let s = 0b0000;
        let d = [0b0001, 0b0011, 0b0111, 0b1111];

        let paths = graph.node_to_set(s, &d);

        assert_eq!(paths.len(), 4);

        paths.iter().zip(d.iter()).for_each(|(path, dest)| {
            assert!(path.is_valid());
            assert_eq!(path.inner_path().first().unwrap(), &0b0000);
            assert_eq!(path.inner_path().last().unwrap(), dest);
        });
    }
}

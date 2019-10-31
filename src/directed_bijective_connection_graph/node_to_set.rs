use crate::{DirectedBijectiveConnectionGraph, DirectedBijectiveConnectionGraphFunctions, Dims, Node};
use crate::node_path::NodePath;

impl<F> DirectedBijectiveConnectionGraph<F>
where F: DirectedBijectiveConnectionGraphFunctions {
    #[allow(non_snake_case)]
    #[inline(always)]
    pub fn N2S(&self, n: Dims, s: Node, d: &mut [Node]) -> Vec<NodePath> {
        self.node_to_set(n, s, d)
    }

    pub fn node_to_set(&self, n: Dims, s: Node, d: &mut [Node]) -> Vec<NodePath> {
        debug_assert_eq!(d.len(), self.dimension as usize);
        self.node_to_set_helper(n, s, d)
    }

    fn node_to_set_helper(&self, n: Dims, s: Node, d: &mut [Node]) -> Vec<NodePath> {
        unimplemented!();
        // Check Case1
        let mask = 1 << (n - 1);
        let flg = d[0] & mask == 0;
        let case1 = d.iter().map(|n| n & mask).all(|n| (n == 0) == flg);

        // Case1
        if case1 {
            // Step 1
            let mut tmp_paths = self.node_to_set_helper(n - 1, s, &mut d[..n as usize - 1]);
            tmp_paths.push(NodePath::new(self.dimension));

            // Step 2
            let dn_lays_on_path_position = tmp_paths.iter().enumerate().find_map(|(pos, path)| {
                path.inner_path()
                    .iter()
                    .position(|node| *node == d[n as usize - 1])
                    .map(|in_path_pos| (pos, in_path_pos))
            });

            // Step 3
            if let Some((vec_pos, path_pos)) = dn_lays_on_path_position {
                tmp_paths[vec_pos].inner_path_mut().truncate(path_pos);
                d.swap(vec_pos, n as usize - 1);
            }
            let d = d[n as usize - 1];
            let phi_s = F::phi(n, s);
            let psi_d = F::psi(n, d);

            // Step 4
            self.R_helper(n, phi_s, psi_d, &mut tmp_paths[n as usize - 1]);
            tmp_paths[n as usize - 1].push_back(d);

            return tmp_paths;
        }

        // Check Case2 and Case3
        // Step 1
        let s_mask = s & mask;

        let mut tmp_paths = Vec::new();
        for node in d {
            let node = *node;

            if node & mask == s_mask {
                let mut path = NodePath::new(self.dimension);
                path.push_back(node);
                tmp_paths.push(path)
            } else {
                for i in (1..n).rev() {
                    let dd = F::psi(n, node);
                    let ddd = F::psi(n, dd);
                    let has_vals = tmp_paths
                        .iter()
                        .any(|p| p.inner_path().contains(&dd) || p.inner_path().contains(&ddd));

                    if !has_vals {
                        let mut path = NodePath::new(self.dimension);
                        path.push_front(node);
                        path.push_front(dd);
                        path.push_front(ddd);

                        tmp_paths.push(path);
                    }
                }
            }
        }

        let index: usize = *d.iter().find(|&&node| node & mask != s_mask).unwrap() as _;
        let phi_s = F::phi(n, s);
        let path = self.R(phi_s, d[index]);

        'exit: for node in path.inner_path() {
            for tmp_path in tmp_paths {
                for other_node in tmp_path.inner_path() {
                    if node == other_node {
                        break 'exit;
                    }
                }
            }
        }

        unimplemented!()
    }
}
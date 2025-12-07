pub mod stackgraph {
    #[derive(Clone, Copy, Default, Debug)]
    struct Node<N> {
        weight: N,
        incoming: Option<usize>,
        outgoing: Option<usize>,
    }

    impl<N> Node<N> {
        fn new(weight: N) -> Self {
            Self {
                weight,
                incoming: None,
                outgoing: None,
            }
        }
    }

    #[derive(Clone, Copy, Default, Debug)]
    struct Edge<E> {
        weight: E,
        from: usize,
        to: usize,
        next_outgoing: Option<usize>,
        next_incoming: Option<usize>,
    }

    impl<E> Edge<E> {
        fn new(from: usize, to: usize, weight: E) -> Self {
            Self {
                weight,
                from,
                to,
                next_outgoing: None,
                next_incoming: None,
            }
        }
    }

    #[derive(Debug)]
    pub struct Graph<N, E, const NN: usize, const NE: usize> {
        nodes: [Node<N>; NN],
        edges: [Edge<E>; NE],
        node_count: usize,
        edge_count: usize,
    }

    impl<N: Default + Copy, E: Default + Copy, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
        pub fn default() -> Self {
            Self {
                nodes: [Default::default(); NN],
                edges: [Default::default(); NE],
                node_count: 0,
                edge_count: 0,
            }
        }
    }

    impl<N: Default, E: Default, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
        pub fn new() -> Self {
            Self {
                nodes: core::array::from_fn(|_| Default::default()),
                edges: core::array::from_fn(|_| Default::default()),
                node_count: 0,
                edge_count: 0,
            }
        }
    }

    impl<N, E, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
        pub fn add_node(&mut self, node: N) -> Option<usize> {
            if self.node_count == NN {
                None
            } else {
                self.nodes[self.node_count] = Node::new(node);
                self.node_count += 1;
                Some(self.node_count - 1)
            }
        }

        pub fn add_edge_undirected(&mut self, from: usize, to: usize, weight: E) -> Option<usize> {
            if self.edge_count == NE || from >= self.node_count || to >= self.node_count {
                None
            } else {
                let edge_index = self.edge_count;
                let mut edge = Edge::new(from, to, weight);

                let from_ref = &mut self.nodes[from];
                edge.next_outgoing = from_ref.outgoing;
                from_ref.outgoing = Some(edge_index);

                let to_ref = &mut self.nodes[to];
                edge.next_incoming = to_ref.incoming;
                to_ref.incoming = Some(edge_index);

                self.edges[edge_index] = edge;

                self.edge_count += 1;

                Some(edge_index)
            }
        }

        pub fn get_neighbors(&self, node: usize) -> Neighbors<'_, N, E, NN, NE> {
            if node >= self.node_count {
                Neighbors::new_empty(self)
            } else {
                Neighbors::new(node, self)
            }
        }

        pub fn get_node(&self, node_index: usize) -> &N {
            &self.nodes[node_index].weight
        }

        pub fn node_count(&self) -> usize {
            self.node_count
        }
    }

    impl<N: Eq + PartialEq, E, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
        pub fn find_node_by_weight(&self, weight: N) -> Option<usize> {
            for (i, n) in self.nodes.iter().enumerate() {
                if n.weight == weight {
                    return Some(i);
                }
            }

            None
        }
    }

    pub struct Neighbors<'a, N, E, const NN: usize, const NE: usize> {
        graph: &'a Graph<N, E, NN, NE>,
        edge: Option<usize>,
    }

    impl<'a, N, E, const NN: usize, const NE: usize> Neighbors<'a, N, E, NN, NE> {
        pub fn new(node: usize, graph: &'a Graph<N, E, NN, NE>) -> Self {
            let edge = graph.nodes[node].outgoing;
            Self { graph, edge }
        }

        fn new_empty(graph: &'a Graph<N, E, NN, NE>) -> Self {
            Self { graph, edge: None }
        }
    }

    impl<'a, N, E, const NN: usize, const NE: usize> Iterator for Neighbors<'a, N, E, NN, NE> {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            match self.edge {
                Some(edge_index) => {
                    let edge = &self.graph.edges[edge_index];
                    let next = edge.to;
                    self.edge = edge.next_outgoing;

                    Some(next)
                }
                none => none,
            }
        }
    }
}

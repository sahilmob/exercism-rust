pub mod graph {
    use graph_items::edge::*;
    use graph_items::node::*;
    use std::collections::HashMap;

    pub mod graph_items {

        pub mod edge {
            use crate::graph::graph_items::node::*;
            use std::collections::HashMap;
            #[derive(Debug, PartialEq)]
            pub struct Edge<'a> {
                pub a: Node<'a>,
                pub b: Node<'a>,
                pub attributes: HashMap<&'a str, &'a str>,
            }

            impl<'a> Edge<'a> {
                pub fn new(a: &'a str, b: &'a str) -> Self {
                    Self {
                        a: Node::new(a),
                        b: Node::new(b),
                        attributes: HashMap::new(),
                    }
                }

                pub fn attr(&self, name: &'a str) -> Option<&str> {
                    self.attributes.get(name).map(|v| &**v)
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attributes = HashMap::new();

                    attrs.iter().for_each(|(k, v)| {
                        self.attributes.insert(*k, *v);
                    });

                    self
                }
            }

            impl<'a> Clone for Edge<'a> {
                fn clone(&self) -> Self {
                    Self {
                        a: self.a.clone(),
                        b: self.b.clone(),
                        attributes: self.attributes.clone(),
                    }
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;
            #[derive(Debug, PartialEq)]
            pub struct Node<'a> {
                pub value: &'a str,
                pub attributes: HashMap<&'a str, &'a str>,
            }

            impl<'a> Node<'a> {
                pub fn new(value: &'a str) -> Self {
                    Self {
                        value,
                        attributes: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
                    self.attributes = HashMap::new();

                    attrs.iter().for_each(|(k, v)| {
                        self.attributes.insert(k, v);
                    });

                    self
                }

                pub fn expect(&self) -> &Self {
                    self
                }

                pub fn attr(&self, name: &'a str) -> Option<&str> {
                    self.attributes.get(name).map(|v| &**v)
                }
            }

            impl<'a> Clone for Node<'a> {
                fn clone(&self) -> Self {
                    Self {
                        value: self.value,
                        attributes: self.attributes.clone(),
                    }
                }
            }
        }
    }

    pub struct Graph<'a> {
        pub nodes: Vec<Node<'a>>,
        pub edges: Vec<Edge<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes<T>(mut self, nodes: &T) -> Self
        where
            T: IntoIterator<Item = Node<'a>> + Clone,
        {
            nodes.clone().into_iter().for_each(|n| {
                self.nodes.push(n.clone());
            });
            self
        }

        pub fn with_edges<T>(mut self, edges: T) -> Self
        where
            T: IntoIterator<Item = &'a Edge<'a>>,
        {
            edges.into_iter().for_each(|e| {
                self.edges.push(e.clone());
            });
            self
        }

        pub fn with_attrs<U: ToString + 'a, T>(mut self, attrs: T) -> Self
        where
            T: IntoIterator<Item = &'a (U, U)>,
        {
            attrs.into_iter().for_each(|(k, v)| {
                self.attrs.insert(k.to_string(), v.to_string());
            });
            self
        }

        pub fn node(&self, val: &'a str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.value == val)
        }
    }
}

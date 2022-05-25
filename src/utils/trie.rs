use crate::definition::Definition;
use crate::grammar::token::Token;

#[derive(Debug, Clone)]
pub struct Node {
    value: Option<Token>,
    pub data: Vec<Definition>,
    pub children: Vec<Box<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            value: None,
            data: vec![],
            children: vec![],
        }
    }

    pub fn with_value(&self, value: Token) -> Self {
        let mut res = self.clone();

        res.value = Some(value);
        res
    }

    pub fn with_data(&self, data: Definition) -> Self {
        let mut res = self.clone();

        res.data.push(data);
        res
    }

    pub fn push(&mut self, nodes: Vec<Node>) {
        if nodes.len() < 1 {
            return;
        }

        let mut head = nodes.first().unwrap().clone();
        let tail = nodes[1..].to_vec();

        for (i, child) in self.children.iter().enumerate() {
            if child.value != None && child.value.clone().unwrap() == head.clone().value.unwrap() {
                self.children[i].push(tail);
                return;
            }
        }

        head.push(tail);
        self.children.push(Box::new(head.clone()));
    }

    pub fn find_all(&self, list: Vec<Token>) -> Vec<Node> {
        if list.len() < 1 {
            return vec![];
        }

        let mut res = vec![];

        let head = list.first().unwrap().clone();

        for child in self.children.iter() {
            if child.value != None && child.value.clone().unwrap() == head.clone() {
                if list.len() > 1 {
                    let tail = list[1..].to_vec();
                    res.extend(child.find_all(tail))
                } else {
                    res.push(*child.clone())
                }
            }
        }

        res
    }

    pub fn find(&self, list: Vec<Token>) -> Option<Node> {
        if list.len() < 1 {
            return None;
        }

        let head = list.first().unwrap().clone();
        for child in self.children.iter() {
            if child.value != None && child.value.clone().unwrap() == head.clone() {
                if list.len() > 1 {
                    let tail = list[1..].to_vec();
                    match child.find(tail) {
                        None => (),
                        res => return res,
                    }
                } else {
                    return Some(*child.clone());
                }
            }
        }

        return None;
    }

    pub fn is_final(&self) -> bool {
        self.children.len() == 0
    }

    pub fn keys(&self) -> Vec<Vec<Token>> {
        if self.is_final() {
            return vec![vec![self.clone().value.unwrap()]];
        }

        // if root
        if self.value == None {
            return self
                .children
                .clone()
                .into_iter()
                .map(|child| child.keys())
                .flatten()
                .collect();
        }

        let res: Vec<Vec<Token>> = self
            .children
            .clone()
            .into_iter()
            .map(move |node| {
                let prefix = self.clone().value.unwrap();
                let keys: Vec<Vec<Token>> = node
                    .keys()
                    .into_iter()
                    .map(move |key| {
                        let mut res: Vec<Token> = vec![prefix.clone()];
                        res.extend(key);

                        res
                    })
                    .collect();

                keys
            })
            .flatten()
            .collect();

        res
    }
}

#[derive(Debug, Clone)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn push(&mut self, list: Vec<Token>, data: Definition) {
        let found = self.find(list.clone());
        let nodes: Vec<Node> = list
            .clone()
            .into_iter()
            .enumerate()
            .map(move |(i, item)| {
                let mut node = Node::new().with_value(item);
                if i == list.len() - 1 {
                    if let Some(f) = &found {
                        node = f.with_data(data.clone());
                    } else {
                        node = node.with_data(data.clone())
                    }
                }
                node
            })
            .collect();
        self.root.push(nodes);
    }

    pub fn find(&self, list: Vec<Token>) -> Option<Node> {
        self.root.find(list)
    }

    pub fn find_all(&self, list: Vec<Token>) -> Vec<Node> {
        self.root.find_all(list)
    }

    pub fn keys(&self) -> Vec<Vec<Token>> {
        self.root.keys()
    }
}

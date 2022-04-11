use super::map::Map;

#[derive(Clone)]
pub struct Node<T, Y> {
    children: Map<T, Vec<Node<T, Y>>>,
    pub data: Option<Y>,
}

impl<T, Y> Node<T, Y>
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug,
    Y: Clone,
{
    pub fn new() -> Self {
        Node {
            children: Map::new(),
            data: None,
        }
    }

    pub fn push(&mut self, list: Vec<T>, data: Option<Y>) {
        if list.len() < 1 {
            return;
        }

        let head = list.first().unwrap();
        let tail = &list[1..];

        let mut child = Node::new();
        if list.len() == 1 {
            child.data = data;
        } else {
            child.push(tail.to_vec(), data);
        }

        let found = self.children.get(head.clone());
        match found {
            None => self.children.insert(head.clone(), vec![child]),
            Some(value) => {
                let mut v = value.clone();
                v.push(child);
                self.children.upsert(head.clone(), v)
            }
        }
    }

    pub fn find(&self, list: Vec<T>) -> Option<(Vec<T>, Self)> {
        if list.len() == 1 {
            let head = list.first().unwrap();
            let res = self.children.get(head.clone());

            return match res {
                None => None,
                Some(nodes) => Some((list, nodes.first().unwrap().clone())),
            };
        }

        let head = list.first().unwrap();
        let tail = &list[1..];

        match self.children.get(head.clone()) {
            Some(nodes) => {
                for node in nodes {
                    if let Some((_, n)) = node.find(tail.to_vec()) {
                        return Some((list, n));
                    }
                }
                return None;
            }
            None => None,
        }
    }

    pub fn find_all(&self, list: Vec<T>) -> (Vec<T>, Vec<Self>) {
        let mut matches: Vec<Self> = vec![];

        let head = list.first().unwrap();
        let tail = &list[1..];

        match self.children.get(head.clone()) {
            Some(nodes) => {
                for node in nodes {
                    if tail.len() == 1 {
                        let found = node.children.get(tail.first().unwrap().clone());
                        matches.extend(match found {
                            Some(n) => n.clone(),
                            None => vec![],
                        })
                    } else {
                        let (_, found) = node.find_all(tail.to_vec());
                        matches.extend(found);
                    }
                }
            }
            None => (),
        };

        (list, matches)
    }
}

#[test]
fn should_be_able_to_push() {
    let mut tree: Node<&str, &str> = Node::new();

    tree.push(vec!["a", "b", "c"], Some("hello world"));
    tree.push(vec!["a", "b", "d"], Some("hello alien"));

    if let Some((key, node)) = tree.find(vec!["a", "b", "c"]) {
        assert_eq!(vec!["a", "b", "c"], key);
        assert_eq!(Some("hello world"), node.data);
    }

    if let Some((key, node)) = tree.find(vec!["a", "b", "d"]) {
        assert_eq!(vec!["a", "b", "d"], key);
        assert_eq!(Some("hello alien"), node.data);
    }
}

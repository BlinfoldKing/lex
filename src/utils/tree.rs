use super::map::Map;

#[derive(Clone)]
pub struct Node<T, Y> {
    children: Map<T, Vec<Node<T, Y>>>,
    pub data: Option<Y>,
}

impl<T, Y> Node<T, Y>
where
    T: Clone + std::cmp::PartialEq + std::fmt::Debug,
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

        self.children.insert(head.clone(), vec![child]);
    }

    pub fn find(&self, list: Vec<T>) -> Option<(Vec<T>, &Node<T, Y>)> {
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
}

#[test]
fn should_be_able_to_push() {
    let mut tree: Node<&str, &str> = Node::new();

    tree.push(vec!["a", "b", "c"], Some("hello world"));

    if let Some((key, node)) = tree.find(vec!["a", "b", "c"]) {
        assert_eq!(vec!["a", "b", "c"], key);
        assert_eq!(Some("hello world"), node.data);
    }
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    meta: Vec<i32>,
} 

fn make_node(children_count: i32, meta_count: i32, text_itr: &mut std::str::SplitWhitespace) -> Node {
    let mut n = Node{
        children: vec![],
        meta: vec![], 
    };

    for _i in 0..children_count {
        let children: i32 = text_itr.next().unwrap().parse().unwrap();
        let meta: i32 = text_itr.next().unwrap().parse().unwrap();

        n.children.push(make_node(children, meta, text_itr));
    }

    for _i in 0..meta_count {
        let meta: i32 = text_itr.next().unwrap().parse().unwrap();
        n.meta.push(meta);
    }

    n
}

fn sum_meta(node: &Node) -> i32 {
    let mut value: i32 = 0;

    if node.children.is_empty() {

        for m in node.meta.clone() {
            value = value + m;
        }
    }
    else {

        for m in node.meta.clone() {
            
            if m <= node.children.len() as i32 {
                value = value + sum_meta(&node.children[m as usize - 1]);
            }
        }
    }

    value
}

fn main() {
    let input = include_str!("input.data");
    
    let mut text_itr = input.split_whitespace();

    let children_count: i32 = text_itr.next().unwrap().parse().unwrap();
    let meta_count: i32 = text_itr.next().unwrap().parse().unwrap();

    let root = make_node(children_count, meta_count, &mut text_itr);

    let result = sum_meta(&root);
    
    println!("{:?}", result);
}

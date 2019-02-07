extern crate regex;

use regex::Regex;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Edges = HashMap<char, HashSet<char>>;

fn parse_edge(record: &str, edges: &mut Edges) {
    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    match re.captures(record) {
        Some(caps) => {
            let from = caps[1].chars().next().unwrap();
            let to = caps[2].chars().next().unwrap();

            let vertices = edges.entry(from).or_insert(HashSet::new());
            vertices.insert(to);
        }
        None => panic!(),
    };
}

fn depends_on(vertex_1: char, vertex_2: char, edges: &Edges) -> bool {

    let vertices = edges.get(&vertex_2);

    if vertices.is_none() {
        return false;
    }

    let vertices = vertices.unwrap();

    if vertices.contains(&vertex_1) {
        return true;
    }
    else {
        let mut res = false;

        for v in vertices {
            res = res || depends_on(vertex_1, *v, edges);
        }

        return res;
    }
}

fn process_vertex(
    vertex: char,
    edges: &Edges,
    path: &mut Vec<char>,
    queue: &mut Vec<char>,
    res: &mut Vec<char>,
) {
//    println!("Process vertex: {}", vertex);

    path.push(vertex);
    res.push(vertex);

    let vxs = edges.get(&vertex);

 //   println!("Edges of process vertex: {:?}", vxs);
 //   println!("Current queue: {:?}", queue);

    match vxs {
        Some(vertices) => {
            let mut sorted_vxs: BTreeSet<char> = vertices
                .into_iter()
                .filter(|v| {
                    let mut result = false;
                    for qv in queue.clone() {
                        result = result || depends_on(**v, qv, edges);
                    }

                    !result
                }).cloned()
                .collect();

            if sorted_vxs.is_empty() {
                return;
            }

            for v in queue.clone() {
                sorted_vxs.insert(v);
            }

            queue.clear();
            for v in sorted_vxs {
                queue.push(v)
            }

            queue.reverse();

   //         println!("Queue after adding edges: {:?}", queue);
        }
        _ => {
          //  println!("vertex without outgoung edges:{}\n", vertex);
         //   res.push(vertex);
            return;
        }
    };

    while queue.len() > 0 {
        let v = queue.pop().unwrap();

       // if !res.contains(&v) && !path.contains(&v) {
        if !res.contains(&v) {
            process_vertex(v, edges, path, queue, res);
        }
    }

    path.pop();
}

fn main() {
    let input = include_str!("test.data");

    let mut edges = Edges::new();

    for line in input.lines() {
        parse_edge(line, &mut edges);
    }

    let mut vertices_with_incoming_edges: HashSet<char> = HashSet::new();
    edges.values().for_each(|vals| {
        vertices_with_incoming_edges = vals.union(&vertices_with_incoming_edges).cloned().collect();
    });

  //  println!("{:?}", depends_on('A', 'K', &edges));

    let start_vertices: BTreeSet<char> = edges
        .keys()
        .filter(|&k| !vertices_with_incoming_edges.contains(k))
        .cloned()
        .collect();

    let mut res = Vec::with_capacity(vertices_with_incoming_edges.len() + 1);

    let mut queue: Vec<char> = start_vertices.iter().cloned().collect();
    queue.reverse();

    let mut path = Vec::new();
    process_vertex(
        queue.pop().unwrap(),
        &edges,
        &mut path,
        &mut queue,
        &mut res,
    );

    print!("{:?}\n", String::from_iter(res));
}

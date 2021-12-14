use petgraph::{graphmap::UnGraphMap, EdgeDirection::Outgoing};
use std::iter::from_fn;

const INPUT: &str = include_str!("input");

struct Cave<'a> {
    graph: UnGraphMap<&'a str, ()>,
}

impl<'a> Cave<'a> {
    fn path_count(&self) -> usize {
        let start = "start";
        let stop = "end";

        let mut visited: Vec<&str> = vec![start];
        let mut stack = vec![self.graph.neighbors_directed(start, Outgoing)];

        from_fn(move || {
            while let Some(children) = stack.last_mut() {
                if let Some(child) = children.next() {
                    if child == stop {
                        let path = visited
                            .iter()
                            .cloned()
                            .chain(Some(stop))
                            .collect::<Vec<_>>();
                        return Some(path);
                    } else if !visited.contains(&child) || child.to_uppercase() == child {
                        visited.push(child);
                        stack.push(self.graph.neighbors_directed(child, Outgoing));
                    }
                } else {
                    visited.pop();
                    stack.pop();
                }
            }
            None
        })
        .count()
    }
}

impl<'a> TryFrom<&'a str> for Cave<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let edges = value.lines().filter_map(|l| l.split_once("-"));

        let graph = UnGraphMap::<_, _>::from_edges(edges);

        Ok(Cave { graph })
    }
}

fn main() {
    let cave = Cave::try_from(INPUT).expect("parse cave");
    println!("{}", cave.path_count());
}

#[cfg(test)]
const TEST_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

#[test]
fn part_1() {
    let cave = Cave::try_from(TEST_INPUT).expect("parse cave");
    assert_eq!(226, cave.path_count());
}

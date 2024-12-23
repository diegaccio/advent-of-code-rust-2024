use advent_of_code::utils::hash::{FastMap, FastMapBuilder, FastSet, FastSetBuilder};

advent_of_code::solution!(23);

fn parse(input: &str) -> FastMap<&str, FastSet<&str>> {
    let mut graph: FastMap<&str, FastSet<&str>> = FastMap::new();

    input.lines().for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    });

    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    // Step 1: Build the adjacency list
    let graph = parse(input);

    // Step 2: Find unique triangles
    let mut unique_triangles_with_t = FastSet::new();
    for &a in graph.keys() {
        if let Some(neighbors_a) = graph.get(a) {
            for &b in neighbors_a {
                if let Some(neighbors_b) = graph.get(b) {
                    for &c in neighbors_b {
                        if c != a && neighbors_a.contains(c) && b < c {
                            // Ensure uniqueness by sorting the triangle lexicographically
                            let mut triangle = vec![a, b, c];
                            if triangle.iter().any(|&node| node.starts_with('t')) {
                                triangle.sort_unstable();
                                unique_triangles_with_t.insert(triangle);
                            }
                        }
                    }
                }
            }
        }
    }

    Some(unique_triangles_with_t.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    // Step 1: Build the adjacency list
    let graph = parse(input);

    // Step 2: Convert the graph to a vector-based representation
    let nodes: Vec<&str> = graph.keys().cloned().collect();
    let mut adj_list: FastMap<&str, FastSet<&str>> = FastMap::new();
    for &node in &nodes {
        adj_list.insert(node, graph.get(node).unwrap().clone());
    }

    // Step 3: Bron-Kerbosch Algorithm with Pivoting
    fn bron_kerbosch<'a>(
        graph: &FastMap<&str, FastSet<&'a str>>,
        r: FastSet<&'a str>,
        p: FastSet<&'a str>,
        x: FastSet<&str>,
        max_clique: &mut Vec<&'a str>,
    ) {
        if p.is_empty() && x.is_empty() {
            // Update max_clique if R is larger
            if r.len() > max_clique.len() {
                *max_clique = r.iter().cloned().collect();
            }
            return;
        }

        // Choose a pivot to reduce the search space
        let pivot = *p.union(&x).next().unwrap();
        let binding = FastSet::new();
        let neighbors = graph.get(pivot).unwrap_or(&binding);

        for &node in &p.difference(neighbors).cloned().collect::<FastSet<_>>() {
            let mut r_new = r.clone();
            r_new.insert(node);

            let p_new = p.intersection(graph.get(node).unwrap()).cloned().collect();
            let x_new = x.intersection(graph.get(node).unwrap()).cloned().collect();

            bron_kerbosch(graph, r_new, p_new, x_new, max_clique);

            // Move node from P to X
            let mut p_mut = p.clone();
            let mut x_mut = x.clone();
            p_mut.remove(node);
            x_mut.insert(node);
        }
    }

    let mut max_clique = Vec::new();
    let r = FastSet::new();
    let p = nodes.iter().cloned().collect();
    let x = FastSet::new();
    bron_kerbosch(&adj_list, r, p, x, &mut max_clique);

    // Step 4: Generate the password
    max_clique.sort();

    // Output the result
    //println!("Largest clique: {:?}", max_clique);

    Some(max_clique.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}

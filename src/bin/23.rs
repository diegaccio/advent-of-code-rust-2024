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

    // Step 2: Function to check if a subset is a clique
    fn is_clique(graph: &FastMap<&str, FastSet<&str>>, nodes: &Vec<&str>) -> bool {
        for i in 0..nodes.len() {
            for j in i + 1..nodes.len() {
                if !graph
                    .get(nodes[i])
                    .unwrap_or(&FastSet::new())
                    .contains(nodes[j])
                {
                    return false;
                }
            }
        }
        true
    }

    // Step 3: Find the largest clique using backtracking

    fn find_largest_clique<'a>(
        graph: &FastMap<&str, FastSet<&str>>,
        nodes: &Vec<&'a str>,
        current_clique: &mut Vec<&'a str>,
        max_clique: &mut Vec<&'a str>,
        index: usize,
    ) {
        // Update the largest clique if the current one is bigger
        if current_clique.len() > max_clique.len() {
            *max_clique = current_clique.clone();
        }

        // Try to expand the current clique
        for i in index..nodes.len() {
            let node = nodes[i];
            current_clique.push(node);
            if is_clique(graph, current_clique) {
                find_largest_clique(graph, nodes, current_clique, max_clique, i + 1);
            }
            current_clique.pop();
        }
    }

    let all_nodes: Vec<&str> = graph.keys().cloned().collect();
    let mut max_clique = Vec::new();
    let mut current_clique = Vec::new();
    find_largest_clique(&graph, &all_nodes, &mut current_clique, &mut max_clique, 0);

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

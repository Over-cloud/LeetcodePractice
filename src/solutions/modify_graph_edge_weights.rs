use crate::solutions::base_solution::Solution;

// Incomplete solution, failed some tests.
impl Solution {
    pub fn modified_graph_edges(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32, target: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut map = vec![vec![0; n]; n];
        for edge in &edges {
            let (i, j, dist) = (edge[0] as usize, edge[1] as usize, edge[2]);
            map[i][j] = dist;
            map[j][i] = dist;
        }

        if Self::shortest_dist(&map, source as usize, destination as usize) < target {
            return Vec::new();
        }

        let mut visited = vec![false; n];
        visited[source as usize] = true;
        let mut modified = Vec::new();
        let mut total_dist = 0;
        if Self::modified_graph_edges_aux(&map, source as usize, destination as usize, target, &mut total_dist,  &mut visited,  &mut modified) {
            let mut result = edges.clone();
            let mut first = target - total_dist + 1;
            for edge in &mut result {
                if edge[2] > 0 {
                    continue;
                }
                edge[2] = 2 * 1_000_000_000;
                for m in &modified {
                    if (edge[0] as usize == m[0] && edge[1] as usize == m[1]) || (edge[1] as usize == m[0] && edge[0] as usize == m[1]) {
                        edge[2] = first;
                        if first > 1 {
                            first = 1;
                        }
                    }
                }
            }
            return result;
        }

        Vec::new()
    }

    fn modified_graph_edges_aux(map: &Vec<Vec<i32>>, curr: usize, dest: usize, target: i32,
                                total_dist: &mut i32, visited: &mut Vec<bool>, modified: &mut Vec<Vec<usize>>) -> bool {

        if curr == dest &&
            ((modified.len() == 0 && *total_dist == target) || *total_dist <= target) {
                return true;
        }

        for (next, &curr_dist) in map[curr].iter().enumerate() {
            if curr_dist == 0 || visited[next] {
                continue;
            }
            
            visited[next] = true;
            if curr_dist < 0 {
                *total_dist += 1;
                modified.push(vec![curr, next]);
                if Self::modified_graph_edges_aux(map, next, dest, target, total_dist, visited, modified) {
                    return true;
                }
                modified.pop();
                *total_dist -= 1;
            } else {
                *total_dist += curr_dist;
                if Self::modified_graph_edges_aux(map, next, dest, target, total_dist, visited, modified) {
                    return true;
                }
                *total_dist -= curr_dist;
            }
            visited[next] = false;
        }

        return false;
    }

    fn shortest_dist(map: &Vec<Vec<i32>>, src: usize, dst: usize) -> i32 {

        let n = map.len();
        let mut dist_map = vec![std::i32::MAX; n];
        dist_map[src] = 0;
        let mut stack = Vec::new();
        stack.push((src, 0));
        while let Some((curr, dist)) = stack.pop() {
            for (next, &edge_len) in map[curr].iter().enumerate() {
                if edge_len <= 0 {
                    continue;
                }
                let next_dist = dist + edge_len;
                if next_dist < dist_map[next] {
                    dist_map[next] = next_dist;
                    stack.push((next, next_dist));
                }
            }
        }

        dist_map[dst]
    }
}

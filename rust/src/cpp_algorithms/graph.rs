use std::collections::HashMap;

/**
 * @brief Function that add edge between two nodes or vertices of graph
 *
 * @param adj1 adjacency list for the direct search
 * @param adj2 adjacency list for the reverse search
 * @param u any node or vertex of graph
 * @param v any node or vertex of graph
 */
fn bidrectional_dijkstra_add_edge(adj1: &mut Vec<Vec<(usize, usize)>>, adj2: &mut Vec<Vec<(usize, usize)>>, u: usize, v: usize, w: usize) {
    adj1[u-1].push((v-1, w));
    adj2[v-1].push((u-1, w));
}

fn bidrectional_dijkstra_shortest_distance(
    workset: &[usize],
    distance: &Vec<Vec<usize>>
) -> usize {
    let mut out = std::usize::MAX;
    for &i in workset.iter() {
        let sum = distance[0][i] + distance[1][i];
        if sum < out {
            out = sum;
        }
    }
    out
}

pub fn bidrectional_dijkstra_solve(adj1: &Vec<Vec<(usize, usize)>>, adj2: &Vec<Vec<(usize, usize)>>, s: usize, t: usize) -> Option<usize> {
    let n = adj1.len();
    let mut dist = vec![vec![std::usize::MAX; n]; 2];

    let mut pq: [std::collections::BTreeSet<(usize, usize)>; 2] = Default::default();
    let mut workset = vec![0; n];
    let mut visited = vec![false; n];

    pq[0].insert((0, s));
    dist[0][s] = 0;

    pq[1].insert((0, t));
    dist[1][t] = 0;

    loop {
        // direct search
        if let Some((current_dist, current_node)) = pq[0].pop_first() { // minimum
            for &(next, weight) in adj1[current_node].iter() {
                if current_dist + weight < dist[0][next] {
                    dist[0][next] = current_dist + weight;
                    pq[0].insert((dist[0][next], next));
                }
            }

            workset.push(current_node);

            if visited[current_node] {
                return Some(bidrectional_dijkstra_shortest_distance(&workset, &dist));
            }
            visited[current_node] = true;
        }

        // reverse search
        if let Some((current_dist, current_node)) = pq[1].pop_first() {
            for &(next, weight) in adj2[current_node].iter() {
                if current_dist + weight < dist[1][next] {
                    dist[1][next] = current_dist + weight;
                    pq[1].insert((dist[1][next], next));
                }
            }
            workset.push(current_node);

            if visited[current_node] {
                return Some(bidrectional_dijkstra_shortest_distance(&workset, &dist));
            }
            visited[current_node] = true;
        } else {
            break
        }
    }

    None
}

#[derive(Default)]
struct BreadthFirstSearch<T> {
    adjacency: HashMap<T, Vec<T>>
}

impl<T: Eq+std::hash::Hash+Copy> BreadthFirstSearch<T> {
    pub fn add_edge(&mut self, u: T, v: T) {
        let vs = self.adjacency.entry(u).or_default();
        vs.push(v);
    }

    pub fn add_edge_bidir(&mut self, u: T, v: T) {
        self.add_edge(u, v);
        self.add_edge(v, u);
    }

    pub fn solve(&mut self, src: T) -> HashMap<T, bool> {
        let mut visited: HashMap<T, bool> = Default::default();

        for (k, vs) in self.adjacency.iter() {
            visited.insert(*k, false);
            for v in vs.iter() {
                visited.insert(*v, false);
            }
        }

        let mut tracker: std::collections::VecDeque<T> = Default::default();

        tracker.push_back(src);
        visited.insert(src, true);

        while tracker.len() > 0 {
            let node = tracker.pop_front().unwrap();
            if let Some(ns) = self.adjacency.get(&node) {
                for neighbour in ns.iter() {
                    if !visited[neighbour] {
                        tracker.push_back(*neighbour);
                        visited.insert(*neighbour, true);
                    }
                }
            }
        }

        visited
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bidrectional_dijkstra_sample() {
        let mut adj1 = vec![vec![]; 4];
        let mut adj2 = vec![vec![]; 4];
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 1, 2, 1);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 4, 1, 2);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 2, 3, 2);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 1, 3, 5);
        let s = 1; let t = 3;
        assert_eq!(bidrectional_dijkstra_solve(&adj1, &adj2, s-1, t-1), Some(3));
        let s = 4; let t = 3;
        assert_eq!(bidrectional_dijkstra_solve(&adj1, &adj2, s-1, t-1), Some(5));

        let mut adj1 = vec![vec![]; 5];
        let mut adj2 = vec![vec![]; 5];
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 1, 2, 4);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 1, 3, 2);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 2, 3, 2);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 3, 2, 1);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 2, 4, 2);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 3, 5, 4);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 5, 4, 1);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 2, 5, 3);
        bidrectional_dijkstra_add_edge(&mut adj1, &mut adj2, 3, 4, 4);
        let s = 1; let t = 5;
        assert_eq!(bidrectional_dijkstra_solve(&adj1, &adj2, s-1, t-1), Some(6));
    }

    #[test]
    fn BreadthFirstSearch_sample() {
        let mut g: BreadthFirstSearch<usize> = BreadthFirstSearch::default();
        g.add_edge_bidir(0, 1);
        g.add_edge_bidir(1, 2);
        g.add_edge_bidir(2, 3);
        let r: HashMap<usize, bool> = vec![(0, true), (1, true), (2, true), (3, true)].into_iter().collect();
        let res = g.solve(2);
        assert_eq!(r, res);

        let mut g: BreadthFirstSearch<&str> = BreadthFirstSearch::default();
        g.add_edge("Gorakhpur", "Lucknow");
        g.add_edge("Gorakhpur", "Kanpur");
        g.add_edge("Lucknow", "Agra");
        g.add_edge("Kanpur", "Agra");
        g.add_edge("Lucknow", "Prayagraj");
        g.add_edge("Agra", "Noida");
        let r: HashMap<&str, bool> = vec![
            ("Gorakhpur" , false),
            ("Lucknow"   , false),
            ("Kanpur"    , true),
            ("Agra"      , true),
            ("Prayagraj" , false),
            ("Noida"     , true),
        ].into_iter().collect();
        let res = g.solve("Kanpur");
        assert_eq!(r, res);

    }
}

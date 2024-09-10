fn floyd_marshall(edge_list: &Vec<(char, char, i32)>) -> Vec<Vec<i32>> {
    let mut graph = vec![vec![1e7 as i32; 26]; 26];

    for (s, t, w) in edge_list.iter() {
        graph[(*s as u8 - 97u8) as usize][(*t as u8 - 97u8) as usize] = *w;
    }

    for k in 0..edge_list.len() {
        for i in 0..edge_list.len() {
            for j in 0..edge_list.len() {
                if graph[i][j] > graph[i][k] + graph[k][j] {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    for i in 0..graph.len() {
        graph[i][i] = 0;
    }

    graph
}

fn main() {
    let source = "aaaa".to_string().chars().collect::<Vec<char>>();
    let target = "bbbb".to_string().chars().collect::<Vec<char>>();

    let original = vec!['a', 'c'];
    let changed = vec!['c', 'b'];
    let cost = vec![1, 2];

    let mut edge_list = Vec::new();
    for (i, char) in original.iter().enumerate() {
        edge_list.push((*char, changed[i], cost[i]));
    }
    // dbg!(&edge_list);

    let mut map = std::collections::HashMap::new();
    for (from, to, price) in edge_list.iter() {
        map.entry(*from).or_insert(vec![]).push((*to, *price));
    }
    // dbg!(&map);

    let mut min = vec![vec![0; 26]; 26];
    let mut heap = std::collections::BinaryHeap::new();
    for (from, next_edges) in map.iter() {
        for next in next_edges.iter() {
            heap.push(next);
        }
        while !heap.is_empty() {
            heap.peek().unwrap_or("Hello");
        }
    }
}
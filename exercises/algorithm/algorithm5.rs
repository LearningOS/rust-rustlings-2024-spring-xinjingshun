/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM DONE
// 广度优先的搜索算法，使用一个队列辅助完成搜索
use std::collections::VecDeque;

// 定义图
struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表表示图的连接关系
}

impl Graph {
    // 创建一个具有n个顶点的新图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // 初始化邻接表
        }
    }

    // 向图中添加一条边
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); // 无向图，两个方向都要添加
        self.adj[dest].push(src);
    }

    // 在图上执行广度优先搜索，并返回访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()]; // 标记顶点是否已访问
        let mut visit_order = vec![]; // 记录访问顺序
        let mut queue = VecDeque::new(); // 辅助队列

        // 将起始顶点加入队列并标记为已访问
        queue.push_back(start);
        visited[start] = true;

        // 开始广度优先搜索
        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 将当前节点加入访问顺序

            // 遍历当前节点的邻居
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    // 如果邻居节点尚未访问，则加入队列并标记为已访问
                    queue.push_back(neighbor);
                    visited[neighbor] = true;
                }
            }
        }

        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}


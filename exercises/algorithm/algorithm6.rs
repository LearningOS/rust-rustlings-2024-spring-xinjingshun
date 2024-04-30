/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

// I AM DONE
//深度优先搜索（DFS）算法。DFS使用递归来实现尽可能深地探索图的每个分支，直到无法再继续深入为止。
use std::collections::HashSet;

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

    // DFS 辅助函数，递归地进行深度优先搜索
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        visited.insert(v); // 将当前节点标记为已访问
        visit_order.push(v); // 将当前节点加入访问顺序

        // 遍历当前节点的邻居
        for &neighbor in &self.adj[v] {
            if !visited.contains(&neighbor) {
                // 如果邻居节点尚未访问，则递归地进行 DFS
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // 在图上执行深度优先搜索，并返回访问顺序
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // 记录已访问的节点
        let mut visit_order = Vec::new(); // 记录访问顺序
        self.dfs_util(start, &mut visited, &mut visit_order); // 调用辅助函数进行 DFS
        visit_order // 返回访问顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}
import tqdm
import sys
from gerrychain import Graph
import networkx as nx

def dfs_paths_rec(cache, graph, node, node_pop, path, pop_col, max_pop):
    max_pop = max_pop - node_pop
    if max_pop > 0:
        for neighbor in graph.neighbors(node):
            if neighbor not in path:
                neighbor_pop = cache[neighbor]
                if max_pop - neighbor_pop <= 0:
                    yield len(path)
                else:
                    path.append(neighbor)
                    for dfs_path in dfs_paths_rec(
                            cache,
                            graph,
                            neighbor,
                            neighbor_pop,
                            path,
                            pop_col,
                            max_pop
                        ):
                        yield dfs_path
                    path.pop(-1)
                        
def max_path_from_node(cache, graph, node, pop_col, max_pop):
    return max(dfs_paths_rec(cache, graph, node, graph.nodes[node][pop_col], [], pop_col, max_pop), default=0)

def max_path(graph, pop_col, max_pop):
    cache = {node: graph.nodes[node][pop_col] for node in graph.nodes}
    return max((max_path_from_node(cache, graph, node, pop_col, max_pop) for node in graph.nodes), default=0)

if __name__ == "__main__":
    graph = Graph.from_json("PA_VTDs.json")
    pop_col = "TOTPOP"
    ideal_pop = 705687.7222222222
    max_pop = ideal_pop*0.02
    node = int(sys.argv[1])

    cache = {node: graph.nodes[node][pop_col] for node in graph.nodes}
    max_path_len = max_path_from_node(cache, graph, node, pop_col, max_pop)
    print(f"Max path length for node {node} is:", max_path_len)

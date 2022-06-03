from gerrychain import Graph
graph = Graph.from_json("PA_VTDs.json")
marked = set()

for node in graph.nodes:
    print(node)
    # if node not in marked and all(x not in marked for x in graph.neighbors(node)):
    #     print(node)
    # else:
    #     marked.add(node)

import os
from igraph import Graph
import pygraphviz as pgv

def convert_dot_to_igraph(input_folder, output_folder):
    if not os.path.exists(input_folder):
        print(f"Error: Input folder '{input_folder}' does not exist!")
        return
    os.makedirs(output_folder, exist_ok=True)

    for file_name in os.listdir(input_folder):
        if file_name.endswith(".dot"):  
            input_path = os.path.join(input_folder, file_name)
            output_path = os.path.join(output_folder, file_name.replace(".dot", ".pkl"))  
            
            try:
                agraph = pgv.AGraph(input_path)
                edges = [(str(e[0]), str(e[1])) for e in agraph.edges()]
                vertices = [str(node) for node in agraph.nodes()]
                igraph_graph = Graph(directed=agraph.is_directed())
                igraph_graph.add_vertices(vertices)
                igraph_graph.add_edges(edges)
                for node in agraph.nodes():
                    for attr, value in node.attr.items():
                        igraph_graph.vs.find(name=str(node)).update_attributes({attr: value})
                for edge in agraph.edges():
                    for attr, value in edge.attr.items():
                        igraph_graph.es[igraph_graph.get_eid(str(edge[0]), str(edge[1]))][attr] = value
                igraph_graph.write_pickle(output_path)
                print(f"Successfully converted: {file_name} -> {output_path}")
            except Exception as e:
                print(f"Error processing {file_name}: {e}")

input_folder = "/mnt/c/Users/irsha/OneDrive/Desktop/Sem 8/Project/symflow/target/out/pdg"
output_folder = "/mnt/c/Users/irsha/OneDrive/Desktop/Sem 8/Project/symflow/target/out/igraph_format/PDG"
convert_dot_to_igraph(input_folder, output_folder)

// Program Dependence Graph (PDG) generation

use std::collections::{HashMap};
use std::io::{self, Write};
use std::fs;
use crate::math::SymCondition;
use crate::flow::{ControlFlowGraph, DataDependencyGraph, AbstractLocation, DependencyNode};
use crate::flow::visualize::{write_header, write_edges, write_footer};

#[derive(Debug, Clone)]
pub struct ProgramDependenceGraph {
    pub nodes: Vec<DependenceNode>,
    pub edges: HashMap<(usize, usize), Vec<PDGEdge>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DependenceNode {
    ControlFlow(u64),
    DataDependency(AbstractLocation),
}

#[derive(Debug, Clone)]
pub struct PDGEdge {
    pub kind: EdgeKind,
    pub condition: SymCondition,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum EdgeKind {
    ControlFlow,
    DataDependency,
}

impl ProgramDependenceGraph {
    pub fn new(cfg: &ControlFlowGraph, ddg: &DataDependencyGraph) -> ProgramDependenceGraph {
        let mut nodes = Vec::with_capacity(cfg.nodes.len() + ddg.nodes.len());
        let mut edges: HashMap<(usize, usize), Vec<PDGEdge>> = HashMap::new();
        let mut ddg_node_mapping = HashMap::new();

        for (index, node) in cfg.nodes.iter().enumerate() {
            let pdg_index = nodes.len();
            nodes.push(DependenceNode::ControlFlow(node.addr));

            for &out_index in &cfg.outgoing[index] {
                edges.entry((pdg_index, out_index))
                    .or_insert(vec![])
                    .push(PDGEdge {
                        kind: EdgeKind::ControlFlow,
                        condition: cfg.edges[&(index, out_index)].clone(),
                    });
            }
        }

        for (index, node) in ddg.nodes.iter().enumerate() {
            if let DependencyNode::Location(location) = node {
                let pdg_index = nodes.len();
                nodes.push(DependenceNode::DataDependency(location.clone()));
                ddg_node_mapping.insert(index, pdg_index);
            }
        }

        for &(start, end) in ddg.edges.keys() {
            if let (Some(&pdg_start), Some(&pdg_end)) = (ddg_node_mapping.get(&start), ddg_node_mapping.get(&end)) {
                edges.entry((pdg_start, pdg_end))
                    .or_insert(vec![])
                    .push(PDGEdge {
                        kind: EdgeKind::DataDependency,
                        condition: ddg.edges[&(start, end)].0.clone(),
                    });
            }
        }

        ProgramDependenceGraph { nodes, edges }
    }

    pub fn visualize<W: Write>(&self, target: W, title: &str) -> io::Result<()> {
        let mut f = target;

        write_header(&mut f, &format!("Program Dependence Graph for {}", title), 40)?;

        for (index, node) in self.nodes.iter().enumerate() {
            match node {
                DependenceNode::ControlFlow(addr) => {
                    writeln!(f, "b{} [label=\"ControlFlow: 0x{:x}\", shape=box]", index, addr)?;
                }
                DependenceNode::DataDependency(location) => {
                    writeln!(f, "b{} [label=\"DataDependency: {}\", shape=ellipse]", index, location)?;
                }
            }
        }

        write_edges(&mut f, &self.edges, |f, ((_, _), edge_list)| {
            for edge in edge_list {
                match edge.kind {
                    EdgeKind::ControlFlow => writeln!(f, "style=solid, color=black")?,
                    EdgeKind::DataDependency => writeln!(f, "style=dashed, color=blue")?,
                }
            }
            Ok(())
        })?;

        write_footer(&mut f)
    }

    pub fn save_as_pdf(&self, filename: &str, title: &str) -> io::Result<()> {
        let path = format!("target/out/pdg/{}.dot", filename);

        fs::create_dir_all("target/out/pdg")?;

        let mut file = fs::File::create(&path)?;
        self.visualize(&mut file, title)?;

        std::process::Command::new("dot")
            .args(&["-Tpdf", &path, "-o", &format!("target/out/pdg/{}.pdf", filename)])
            .status()?;
        Ok(())
    }
}

pub fn generate_and_save_pdg(cfg: &ControlFlowGraph, ddg: &DataDependencyGraph, filename: &str) {
    let pdg = ProgramDependenceGraph::new(cfg, ddg);
    pdg.save_as_pdf(filename, "Program Dependence Graph").expect("Failed to save PDG");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Program;
    use crate::flow::{ControlFlowGraph, DataDependencyGraph};

    fn run_test_pdg(filename: &str) {
        let path = format!("target/bin/{}", filename);

        let program = Program::new(path);

        let cfg = ControlFlowGraph::new(&program);

        let ddg = DataDependencyGraph::new(&cfg);

        let pdg = ProgramDependenceGraph::new(&cfg, &ddg);

        pdg.save_as_pdf(filename, "Program Dependence Graph").unwrap();
    }

    #[test]
    fn pdg_example() {
        run_test_pdg("block-1");
        run_test_pdg("bufs");
        run_test_pdg("paths");
        run_test_pdg("deep");
        run_test_pdg("overwrite");
        run_test_pdg("min");
        run_test_pdg("custom");
        run_test_pdg("checking");
    }
}

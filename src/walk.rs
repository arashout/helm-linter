use gtmpl::node;

pub fn walk(node: &node::Nodes, acc: &mut Vec<Vec<String>>) {
    match node {
        node::Nodes::List(ln) => {
            for n in ln.nodes.clone() {
                walk(&n, acc);
            }
        }
        node::Nodes::Text(_) => {}
        node::Nodes::Pipe(_) => {}
        node::Nodes::Action(an) => {
            for cmd_node in an.pipe.cmds.clone() {
                for n in cmd_node.args {
                    if let node::Nodes::Field(field_node) = n {
                        acc.push(field_node.ident);
                    }
                }
            }
        }
        node::Nodes::Command(cn) => {
            println!("{}", cn);
        }
        node::Nodes::Identifier(_) => {}
        node::Nodes::Variable(_) => {}
        node::Nodes::Dot(_) => {}
        node::Nodes::Nil(_) => {}
        node::Nodes::Field(_) => {}
        node::Nodes::Chain(_) => {}
        node::Nodes::Bool(_) => {}
        node::Nodes::Number(_) => {}
        node::Nodes::String(_) => {}
        node::Nodes::End(_) => {}
        node::Nodes::Else(_) => {}
        node::Nodes::If(_) => {}
        node::Nodes::With(_) => {}
        node::Nodes::Range(_) => {}
        node::Nodes::Template(_) => {}
    }
}

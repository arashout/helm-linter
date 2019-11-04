use std::io::prelude::*;
use std::{fs, fs::File};
use yaml_rust::{YamlLoader, yaml::Yaml};

use gtmpl::node;

fn load_yaml(file_path: &str) -> Vec<yaml_rust::yaml::Yaml> {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&contents).expect("Could not parse yaml");
    docs
}

fn file_to_str(file_path: &std::path::Path) -> String {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Could not read file!");
    contents
}

fn walk(node: &node::Nodes, acc: &mut Vec<Vec<String>>) {
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
                    match n {
                        node::Nodes::Field(field_node) => {
                            acc.push(field_node.ident);
                        }
                        _ => {}
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
        _ => {
            println!("Something else");
        }
    }
}

fn in_values(yams: &Vec<Yaml>, acc: &[String]) -> bool {
    yams.iter()
        .map(|y| in_value(y, acc))
        .fold(false, |b_acc, val| b_acc || val)
}

fn in_value(yam: &Yaml, acc: &[String]) -> bool {
    if acc.len() == 0{
        println!("Acc ended: {:?} with result {:?}", acc, yam);
        return true;
    }

    match yam {
        Yaml::Hash(hm) => {
            let inner_yam = &hm[&make_key(acc[0].to_owned())];
            return in_value(inner_yam, &acc[1..]);
            },
        Yaml::BadValue => {
            println!("Bad Value: {:?}", acc);
            return false;
        }
        _ => {}
    }
    false
}

// TODO: This is retarded...
fn make_key(key: String) -> Yaml {
    Yaml::String(key)
}

fn main() {
    let values_path = "./test/values.yaml";
    let all_values = load_yaml(values_path);

    let template_paths = fs::read_dir("./test/templates").expect("Could not read dir!");

    let mut variables_acc = vec![];
    for path in template_paths {
        let mut t = gtmpl::template::Template::default();
        let tmpl_content = file_to_str(&path.unwrap().path());
        t.parse(tmpl_content).expect("Could not parse template");

        for entry in t.tree_set.iter() {
            let tree = entry.1;
            let on = tree.root.as_ref();
            match on {
                Some(node) => walk(node, variables_acc.as_mut()),
                None => {}
            }
        }
    }

    println!("Values: {:?}", all_values);
    println!("Template References: {:?}", &variables_acc);

    for acc in variables_acc.iter() {
        // Attempt to access Values provided in config
        if acc.get(0).unwrap() == "Values" {
            if !in_values(&all_values, &acc[1..]) {
                println!("Not in yam!");
            }
        }
    }
}

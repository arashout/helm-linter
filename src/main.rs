use std::fs;

use clap::{App, Arg};

mod util;
mod validate;
mod walk;
use validate::{validate_var_chain, ValidationErr};

use colored::*;

// TODO: Eventually move to macro based clap
struct Config {
    values_path: String,
    templates_path: String,
}

fn main() {
    let matches = App::new("helm-linter")
        .version("1.0")
        .author("Arash O. <arash.out@gmail.com>")
        .about("Lints helm templates")
        .arg(
            Arg::with_name("values_path")
                .short("v")
                .long("values_path")
                // .required(true)
                .default_value("test/values.yaml")
                .help("Path to values.yaml")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("templates_path")
                .short("t")
                .long("templates_path")
                // .required(true)
                .default_value("test/templates")
                .help("Path to templates directory")
                .takes_value(true),
        )
        .get_matches();
    let config = Config {
        values_path: matches.value_of("values_path").unwrap().to_owned(),
        templates_path: matches.value_of("templates_path").unwrap().to_owned(),
    };

    let values = util::load_yaml(&config.values_path);
    let template_paths = fs::read_dir(config.templates_path).expect("Could not read dir!");

    let mut variables_chain = vec![];
    for path in template_paths {
        let mut t = gtmpl::template::Template::default();
        let tmpl_content = util::file_to_str(&path.expect("Could not get path").path());
        t.parse(tmpl_content).expect("Could not parse template");

        for entry in t.tree_set.iter() {
            let tree = entry.1;
            let on = tree.root.as_ref();
            if let Some(node) = on {
                walk::walk(node, variables_chain.as_mut())
            }
        }
    }

    println!("Values: {:?}", values);
    println!("Template References: {:?}", &variables_chain);
    let mut errors_vec = vec![];
    for var_chain in variables_chain.iter() {
        if var_chain.get(0).unwrap() == "Values" {
            match validate_var_chain(&values, &var_chain[..]) {
                Ok(_) => (),
                Err(ve) => errors_vec.push(ve),
            }
        }
    }

    if errors_vec.is_empty() {
        println!("{}", "Success!".green());
    } else {
        println!("{}", format!("{} warnings", errors_vec.len()).yellow());
        for ve in errors_vec {
            match ve {
                ValidationErr::MissingKey(s) => println!("{}", s),
                ValidationErr::NotFullyResolved(s) => println!("{}", s),
                ValidationErr::VariableChainNotFound(s) => println!("{}", s),
                ValidationErr::NotImplemented(s) => println!("{}", s),
            }
        }
        std::process::exit(1)
    }
}

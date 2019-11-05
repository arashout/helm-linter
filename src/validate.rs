use serde_yaml::{Value, Mapping};

pub enum ValidationErr {
    VariableChainNotFound(String),
    MissingKey(String),
    NotImplemented(String),
}

pub fn validate_var_chain(values: &Value, var_chain: &[String]) -> Result<String, ValidationErr> {
    match values {
        Value::Mapping(m) => {
            in_mapping(m, &var_chain[..], &var_chain[1..])
        },
        _ => {
            panic!(format!("Value::Mapping not matched in top-level yaml!: {:?}", values));
        }
    }
}

fn in_mapping(m: &Mapping, original_var_chain: &[String], current_var_chain: &[String]) -> Result<String, ValidationErr> {
    let key = Value::String(current_var_chain[0].to_owned());
    let ov = m.get(&key);
    match ov {
        Some(v)=>{
            match v {
                Value::Mapping(_m) => in_mapping(_m, &original_var_chain, &current_var_chain[1..]),
                Value::String(s) => {
                    if current_var_chain.len() > 1 {
                        return Err(ValidationErr::VariableChainNotFound(format!("Could not complete chain! Reached {:?} in {:?}", &current_var_chain, &original_var_chain)));
                    }
                    Ok(format!("{:?} -> {:?}", original_var_chain.join("."), m))
                },
                other => {
                    Err(ValidationErr::NotImplemented(format!("Not implemented: {:?}", other)))
                },
            }
        },
        None => {
            Err(ValidationErr::MissingKey(format!("Missing key at: {:?} in {:?}", &key, &original_var_chain)))
        }
    }
}
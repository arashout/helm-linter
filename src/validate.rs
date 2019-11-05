use serde_yaml::{Mapping, Value};

#[derive(Debug)]
pub enum ValidationErr {
    VariableChainNotFound(String),
    NotEnoughValues(String),
    MissingKey(String),
    NotImplemented(String),
}

pub fn validate_var_chain(values: &Value, var_chain: &[String]) -> Result<String, ValidationErr> {
    match values {
        Value::Mapping(m) => in_mapping(m, &var_chain[..], &var_chain[1..]),
        _ => {
            panic!(format!(
                "Value::Mapping not matched in top-level yaml!: {:?}",
                values
            ));
        }
    }
}

fn in_mapping(
    m: &Mapping,
    original_var_chain: &[String],
    current_var_chain: &[String],
) -> Result<String, ValidationErr> {
    if current_var_chain.len() == 0 {
        return Err(ValidationErr::NotEnoughValues(format!(
            "Could not resolve to primitive! .{} resolves to:\t{:?}",
            &original_var_chain.join("."),
            m,
        )));
    }
    let key = Value::String(current_var_chain[0].to_owned());
    let ov = m.get(&key);
    match ov {
        Some(v) => match v {
            Value::Mapping(_m) => in_mapping(_m, &original_var_chain, &current_var_chain[1..]),
            Value::String(s) => {
                if current_var_chain.len() > 1 {
                    return Err(ValidationErr::VariableChainNotFound(format!(
                        "Could not complete chain! Reached .{} in .{}",
                        &current_var_chain.to_vec().drain(1..).collect::<Vec<String>>().join("."),
                        &original_var_chain.join(".")
                    )));
                }
                Ok(format!("'.{}' -> '{}'", original_var_chain.join("."), s))
            }
            other => Err(ValidationErr::NotImplemented(format!(
                "Not implemented: {:?}",
                other
            ))),
        },
        None => Err(ValidationErr::MissingKey(format!(
            "Missing key at: '.{}' in '.{}'",
            &key.as_str().unwrap(),
            &original_var_chain.join(".")
        ))),
    }
}

use crate::{
    hashmap::{SysctlParameterHashMap, SysctlParameterValue},
    schema::{SchemaHashMap, SchemaType},
};

fn validate_value(path: &str, value: &str, schema_type: SchemaType) -> Result<(), String> {
    match schema_type {
        SchemaType::Bool() => {
            if let Ok(_) = value.parse::<bool>() {
                Ok(())
            } else {
                Err(format!("'{}' has not a bool value '{}'", path, value))
            }
        }
        SchemaType::Integer() => {
            if let Ok(_) = value.parse::<u64>() {
                Ok(())
            } else {
                Err(format!("'{}' has not a integer value '{}'", path, value))
            }
        }
        SchemaType::String() => Ok(()),
    }
}

pub fn validate(sysctl_conf: SysctlParameterHashMap, schema: SchemaHashMap) -> Result<(), String> {
    for s in schema {
        let path = s.0.split(".").collect::<Vec<&str>>();
        match sysctl_conf.get(&path) {
            Some(SysctlParameterValue::V(value)) => validate_value(s.0, value, s.1),
            Some(SysctlParameterValue::M(_)) => Err(format!("'{}' is not a literal value, is a submap", s.0)),
            _ => Err(format!("'{}' is not found", s.0)),
        }?
    }
    Ok(())
}

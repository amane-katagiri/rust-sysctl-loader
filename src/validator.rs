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
            Some(SysctlParameterValue::M(_)) => {
                Err(format!("'{}' is not a literal value, is a submap", s.0))
            }
            _ => Err(format!("'{}' is not found", s.0)),
        }?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn sample1() {
        let result = validate(
            SysctlParameterHashMap {
                items: HashMap::from([
                    ("endpoint", SysctlParameterValue::V("localhost:3000")),
                    ("debug", SysctlParameterValue::V("true")),
                    (
                        "log",
                        SysctlParameterValue::from_map(HashMap::from([
                            ("file", SysctlParameterValue::V("/var/log/console.log")),
                            ("limit", SysctlParameterValue::V("1024")),
                        ])),
                    ),
                ]),
            },
            SchemaHashMap::from([
                ("endpoint", SchemaType::String()),
                ("debug", SchemaType::Bool()),
                ("log.file", SchemaType::String()),
                ("log.limit", SchemaType::Integer()),
            ]),
        );
        assert_eq!(result, Ok(()),);
    }

    #[test]
    fn invalid_sysctl_conf_no_such_token() {
        let result = validate(
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000"))]),
            },
            SchemaHashMap::from([("log.limit", SchemaType::Integer())]),
        );
        assert_eq!(result, Err(format!("'log.limit' is not found")),);
    }

    #[test]
    fn invalid_sysctl_conf_submap_token() {
        let result = validate(
            SysctlParameterHashMap {
                items: HashMap::from([
                    ("endpoint", SysctlParameterValue::V("localhost:3000")),
                    ("debug", SysctlParameterValue::V("true")),
                    (
                        "log",
                        SysctlParameterValue::from_map(HashMap::from([
                            ("file", SysctlParameterValue::V("/var/log/console.log")),
                            ("limit", SysctlParameterValue::V("1024")),
                        ])),
                    ),
                ]),
            },
            SchemaHashMap::from([("log", SchemaType::String())]),
        );
        assert_eq!(
            result,
            Err(format!("'log' is not a literal value, is a submap")),
        );
    }

    #[test]
    fn invalid_sysctl_conf_invalid_bool() {
        let result = validate(
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000"))]),
            },
            SchemaHashMap::from([("endpoint", SchemaType::Bool())]),
        );
        assert_eq!(
            result,
            Err(format!("'endpoint' has not a bool value 'localhost:3000'")),
        );
    }

    #[test]
    fn invalid_sysctl_conf_invalid_integer() {
        let result = validate(
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000"))]),
            },
            SchemaHashMap::from([("endpoint", SchemaType::Integer())]),
        );
        assert_eq!(
            result,
            Err(format!(
                "'endpoint' has not a integer value 'localhost:3000'"
            )),
        );
    }
}

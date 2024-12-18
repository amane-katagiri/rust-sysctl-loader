use std::collections::HashMap;

#[derive(Debug)]
pub enum SchemaType {
    String(),
    Bool(),
    Integer(),
}

pub type SchemaHashMap<'a> = HashMap<&'a str, SchemaType>;

struct SchemaParameter<'a> {
    path: &'a str,
    schema_type: SchemaType,
}

fn parse_token<'a>(path: &'a str, schema_type: &'a str) -> Result<SchemaParameter<'a>, String> {
    let path = path.trim();
    let schema_type = schema_type.trim();
    // invalid token begins or ends with a `.` or has continuous `.`
    if path.starts_with(".") || path.ends_with(".") || path.contains("..") {
        Err(format!(
            "Token '{}' has an invalid hierarchical structure",
            path
        ))
    // valid token
    } else {
        match schema_type {
            "string" => Ok(SchemaParameter {
                path,
                schema_type: SchemaType::String(),
            }),
            "bool" => Ok(SchemaParameter {
                path,
                schema_type: SchemaType::Bool(),
            }),
            "integer" => Ok(SchemaParameter {
                path,
                schema_type: SchemaType::Integer(),
            }),
            _ => Err(format!(
                "'{}' has an invalid schema type '{}' (must be string, bool or integer)",
                path, schema_type
            )),
        }
    }
}

fn parse_line<'a>(line: &'a str) -> Result<Option<SchemaParameter<'a>>, String> {
    let line = line.trim();
    // multiple lines
    if line.contains("\n") || line.contains("\r") {
        Err(format!("'{}' is not a single line", line))
    }
    // empty, comment, or invalid token line
    else if line.len() == 0
        || line.starts_with("#")
        || line.starts_with(";")
        || line.starts_with("-")
    {
        Ok(None)
    // invalid syntax line
    } else if !line.contains("=>") {
        Err(format!("'{}' is not in format `token => value`", line))
    // valid syntax line
    } else {
        let mut parameter = line.splitn(2, "=>");
        let token = parameter.next().unwrap();
        let value = parameter.next().unwrap();
        match parse_token(token, value) {
            Ok(parameter) => Ok(Some(parameter)),
            Err(err) => Err(err),
        }
    }
}

pub fn parse_str<'a>(schema_conf: &'a str) -> Result<SchemaHashMap<'a>, String> {
    let mut parameter = SchemaHashMap::new();
    for line in schema_conf.lines() {
        let parsed = parse_line(line)?;
        if let Some(parsed) = parsed {
            parameter.insert(&parsed.path, parsed.schema_type);
        }
    }
    Ok(parameter)
}

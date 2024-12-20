use crate::hashmap::SysctlParameterHashMap;

struct SysctlParameter<'a> {
    path: Vec<&'a str>,
    value: &'a str,
}

fn parse_token<'a>(token: &'a str, value: &'a str) -> Result<SysctlParameter<'a>, String> {
    let token = token.trim();
    let value = value.trim();
    // invalid token begins or ends with a `.` or has continuous `.`
    if token.starts_with(".") || token.ends_with(".") || token.contains("..") {
        Err(format!(
            "Token '{}' has an invalid hierarchical structure",
            token
        ))
    // valid token
    } else {
        let path = token.split(".").collect::<Vec<&str>>();
        Ok(SysctlParameter { path, value })
    }
}

fn parse_line<'a>(line: &'a str) -> Result<Option<SysctlParameter<'a>>, String> {
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
    } else if !line.contains("=") {
        Err(format!("'{}' is not in format `token = value`", line))
    // valid syntax line
    } else {
        let mut parameter = line.splitn(2, "=");
        let token = parameter.next().unwrap();
        let value = parameter.next().unwrap();
        match parse_token(token, value) {
            Ok(parameter) => Ok(Some(parameter)),
            Err(err) => Err(err),
        }
    }
}

pub fn parse_str<'a>(sysctl_conf: &'a str) -> Result<SysctlParameterHashMap<'a>, String> {
    let mut parameter = SysctlParameterHashMap::new();
    for line in sysctl_conf.lines() {
        let parsed = parse_line(line)?;
        if let Some(parsed) = parsed {
            parameter.insert(&parsed.path, parsed.value);
        }
    }
    Ok(parameter)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hashmap::SysctlParameterValue;
    use std::collections::HashMap;

    #[test]
    fn sample1() {
        let result = parse_str(
            "endpoint = localhost:3000
debug = true
log.file = /var/log/console.log
log.limit = 1024",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([
                    ("endpoint", SysctlParameterValue::V("localhost:3000")),
                    ("debug", SysctlParameterValue::V("true")),
                    (
                        "log",
                        SysctlParameterValue::from_map(HashMap::from([
                            ("file", SysctlParameterValue::V("/var/log/console.log")),
                            ("limit", SysctlParameterValue::V("1024"))
                        ]))
                    )
                ])
            }
        );
    }

    #[test]
    fn sample2() {
        let result = parse_str(
            "endpoint = localhost:3000
# debug = true
log.file = /var/log/console.log
log.limit = 1024
log.name = default.log",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([
                    ("endpoint", SysctlParameterValue::V("localhost:3000")),
                    (
                        "log",
                        SysctlParameterValue::from_map(HashMap::from([
                            ("file", SysctlParameterValue::V("/var/log/console.log")),
                            ("limit", SysctlParameterValue::V("1024")),
                            ("name", SysctlParameterValue::V("default.log"))
                        ]))
                    )
                ])
            }
        );
    }

    #[test]
    fn overwrite() {
        let result = parse_str(
            "endpoint = localhost:3000
endpoint = localhost:3001",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3001")),])
            }
        );
    }

    #[test]
    fn whitespaces() {
        let result = parse_str(
            "  endpoint = localhost:3000  

        ",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000")),])
            }
        );
    }

    #[test]
    fn comments() {
        let result = parse_str(
            "#commentline
;commentline2
  #commentline3
  ;commentline4
endpoint = localhost:3000
",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000")),])
            }
        );
    }

    #[test]
    fn invalid_token_begins_with_hyphen() {
        let result = parse_str(
            "endpoint = localhost:3000
-log.file = /var/log/console.log",
        );
        assert_eq!(
            result.unwrap(),
            SysctlParameterHashMap {
                items: HashMap::from([("endpoint", SysctlParameterValue::V("localhost:3000")),])
            }
        );
    }

    #[test]
    fn invalid_token_begins_with_dot() {
        let result = parse_str(".endpoint = localhost:3000");
        assert_eq!(
            result,
            Err("Token '.endpoint' has an invalid hierarchical structure".to_string())
        );
    }

    #[test]
    fn invalid_token_ends_with_dot() {
        let result = parse_str("endpoint. = localhost:3000");
        assert_eq!(
            result,
            Err("Token 'endpoint.' has an invalid hierarchical structure".to_string())
        );
    }

    #[test]
    fn invalid_token_has_continuous_dots() {
        let result = parse_str("end..point = localhost:3000");
        assert_eq!(
            result,
            Err("Token 'end..point' has an invalid hierarchical structure".to_string())
        );
    }

    #[test]
    fn invalid_syntax() {
        let result = parse_str("end.point.localhost:3000");
        assert_eq!(
            result,
            Err("'end.point.localhost:3000' is not in format `token = value`".to_string())
        );
    }
}

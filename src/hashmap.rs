use std::{collections::HashMap, fmt};

#[derive(PartialEq)]
pub enum SysctlParameterValue<'a> {
    V(&'a str),
    M(Box<SysctlParameterHashMap<'a>>),
}
impl fmt::Debug for SysctlParameterValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::V(value) => write!(f, "\"{}\"", value.replace("\"", "\\\"")),
            Self::M(value) => write!(f, "{:?}", value),
        }
    }
}

#[derive(PartialEq)]
pub struct SysctlParameterHashMap<'a> {
    pub items: HashMap<&'a str, SysctlParameterValue<'a>>,
}
impl<'a> SysctlParameterHashMap<'a> {
    pub fn new() -> Self {
        SysctlParameterHashMap {
            items: HashMap::new(),
        }
    }
    pub fn get(&self, path: &Vec<&'a str>) -> Option<&SysctlParameterValue<'a>> {
        match path.len() {
            0 => None,
            1 => {
                // get map value
                self.items.get(path.first().unwrap())
            }
            _ => {
                let &child_token = path.first().unwrap();
                // get map recursively
                if let Some(SysctlParameterValue::M(child)) = self.items.get(child_token) {
                    let path = match path.get(1..) {
                        Some(path) => path,
                        _ => &[],
                    }
                    .to_vec();
                    child.get(&path)
                } else {
                    None
                }
            }
        }
    }
    pub fn insert(&mut self, path: &Vec<&'a str>, value: &'a str) {
        match path.len() {
            0 => return,
            1 => {
                // set map value
                self.items
                    .insert(path.first().unwrap(), SysctlParameterValue::V(value));
            }
            _ => {
                let &child_token = path.first().unwrap();
                // set initial map
                if !self.items.contains_key(child_token) {
                    self.items.insert(
                        child_token,
                        SysctlParameterValue::M(Box::new(SysctlParameterHashMap::new())),
                    );
                }
                // update map recursively
                if let Some(SysctlParameterValue::M(child)) = self.items.get_mut(child_token) {
                    let path = match path.get(1..) {
                        Some(path) => path,
                        _ => &[],
                    }
                    .to_vec();
                    child.insert(&path, value);
                }
            }
        }
    }
}
impl fmt::Debug for SysctlParameterHashMap<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.items)
    }
}
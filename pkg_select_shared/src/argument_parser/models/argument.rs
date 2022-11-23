use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Argument {
    pub command: Vec<String>,
    pub(in crate::argument_parser) optional: HashMap<String, Option<String>>,
    pub non_optional: Vec<String>
}

impl Default for Argument {
    fn default() -> Self {
        Argument {
            command: vec![],
            optional: HashMap::default(),
            non_optional: vec![]
        }
    }
}

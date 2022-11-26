use crate::argument_parser::models::argument::Argument;

impl Argument {
    pub fn set_optional(&mut self, key: String, value: Option<String>) -> Option<Option<String>> {
        self.optional.insert(key, value)
    }

    pub fn optional_flag(&self, key: String) -> bool {
        self.optional.get(&key).is_some()
    }

    pub fn optional_argument(&self, key: String) -> Option<String> {
        self.optional.get(&key)
            .and_then(| value | value.as_ref().and_then(| value | Some(value.to_string())))
    }
}

use std::collections::HashMap;
use crate::argument_parser::models::argument::Argument;

pub fn parse_args(args_string: String) -> Argument {
    let mut command: Vec<String> = vec![];
    let mut optional: HashMap<String, Option<String>> = HashMap::default();
    let mut non_optional: Vec<String> = vec![];

    let split_non_optional_sign: Vec<&str> = args_string.split("-- ").collect();
    if split_non_optional_sign.len() == 2 {
        let split_non_optional: Vec<&str> = split_non_optional_sign[1].split_whitespace().collect();

        non_optional = merge_by_quote(&split_non_optional);
    }

    let split_whitespace: Vec<&str> = split_non_optional_sign[0].split_whitespace().collect();
    let merged = merge_by_quote(&split_whitespace);

    for value in merged.into_iter() {
        match value.find("-") {
            None => {
                command.push(value);
                continue;
            }
            Some(index) => {
                if index > 0 {
                    command.push(value);
                } else {
                    match value.find("=") {
                        None => {
                            optional.insert(value, None);
                        }
                        Some(index2) => {
                            optional.insert(
                                value[..index2].to_string(),
                                Some(value[index2 + 1..].to_string())
                            );
                        }
                    }
                }
            }
        }
    }

    Argument {
        command,
        optional,
        non_optional
    }
}

fn merge_by_quote(data: &Vec<&str>) -> Vec<String> {
    let mut merged: Vec<String> = vec![];
    let mut merge_start_position: Option<usize> = None;
    let mut working_with_single_quote: Option<bool> = None;

    for (index, value) in data.iter().enumerate() {
        let single_quote_count = value.chars()
            .filter(| value2 | value2.eq(&'\''))
            .count();
        let double_quote_count = value.chars()
            .filter(| value2 | value2.eq(&'"'))
            .count();

        if single_quote_count == 0 && double_quote_count == 0 {
            if merge_start_position.is_none() {
                merged.push(value.to_string());
            }
        } else if single_quote_count > double_quote_count {
            if working_with_single_quote.is_none() {
                working_with_single_quote = Some(true);
            }

            if !working_with_single_quote.unwrap() {
                continue;
            }

            if single_quote_count % 2 == 0 && merge_start_position.is_none() {
                merged.push(value.replace("'", ""));
                continue;
            }

            let merge_start = match merge_start_position {
                None => {
                    merge_start_position = Some(index);
                    continue;
                }
                Some(value) => value,
            };

            let frags: Vec<&str> = (&data)[merge_start..index + 1].iter()
                .map(| value | *value)
                .collect();

            merged.push(frags.join(" ").replace("'", "").replace("\"", ""));
            merge_start_position = None;
            working_with_single_quote = None;

            continue;
        } else {
            if working_with_single_quote.is_none() {
                working_with_single_quote = Some(false);
            }

            if working_with_single_quote.unwrap() {
                continue;
            }

            if double_quote_count % 2 == 0 && merge_start_position.is_none() {
                merged.push(value.replace("\"", ""));
                continue;
            }

            let merge_start = match merge_start_position {
                None => {
                    merge_start_position = Some(index);
                    continue;
                }
                Some(value) => value,
            };

            let frags: Vec<&str> = (&data)[merge_start..index + 1].iter()
                .map(| value | *value)
                .collect();

            merged.push(frags.join(" ").replace("'", "").replace("\"", ""));
            merge_start_position = None;
            working_with_single_quote = None;

            continue;
        }
    }

    match merge_start_position {
        None => {}
        Some(value) => {
            for value2 in (&data)[value..].iter() {
                merged.push(value2.to_string());
            }
        },
    };

    merged
}

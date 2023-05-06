use std::collections::HashMap;
use weather_core::layout::layout_serde::{ItemSerde, LayoutDefaultsSerde, LayoutSerde};
use regex::Regex;

fn strip(line: &str) -> &str { // TODO: Fix function (its actually good for now, but it can be better)
    return line.trim_end();
}

pub fn string_to_item(s: &str) -> ItemSerde {
    let mut new_s: String = s.to_string();
    if !new_s.is_empty() {
        let mut color = None;
        if new_s.chars().next().expect("0th char expected") == '$' {
            new_s = new_s[1..].to_string();
            let mut tmp_color = String::default();
            while new_s.chars().next().expect("ending $ not found") != '$' {
                tmp_color += &new_s.chars().next().unwrap().to_string();
                new_s = new_s[1..].to_string()
            }
            new_s = new_s[1..].to_string();
            color = Some(tmp_color);
        }
        if new_s.chars().next().expect("Oth char expected") == '@' {
            new_s = new_s[1..].to_string();
            let splt: Vec<&str> = new_s.split('|').collect();
            let mut metric: Option<String> = None;
            let mut imperial: Option<String> = None;
            if splt.len() == 2 {
                metric = Some(splt[1].to_string());
                imperial = Some(splt[1].to_string());
            } else if splt.len() == 3 {
                imperial = Some(splt[1].to_string());
                metric = Some(splt[2].to_string());
            }
            return ItemSerde {
                item_type: "variable".to_string(),
                color,
                bg_color: None,
                metric,
                imperial,
                unit_color: None,
                value: splt[0].to_string(),
                args: None,
                kwargs: None,
                scale: None,
            };
        } else if new_s.chars().next().expect("Oth char expected") == '#' {
            new_s = new_s[1..].to_string();
            let mut split: Vec<&str> = new_s.split('|').collect();
            let value = split[0];
            split.remove(0);
            let mut args: Vec<ItemSerde> = Vec::new();
            let mut kwargs: HashMap<String, ItemSerde> = HashMap::new();
            for item in split {
                if item.contains('=') {
                    let temp_item = item.to_string();
                    let kwarg: Vec<&str> = temp_item.split('=').collect();
                    kwargs.insert(
                        kwarg[0].to_string(),
                        string_to_item(kwarg[1]),
                    );
                } else {
                    args.push(string_to_item(item));
                }
            }
            let item: ItemSerde = ItemSerde {
                item_type: "function".to_string(),
                color,
                bg_color: None,
                metric: None,
                imperial: None,
                unit_color: None,
                value: value.to_string(),
                args: Some(args),
                kwargs: Some(kwargs),
                scale: None,
            };
            return item;
        } else if new_s.chars().next().expect("Oth char expected") == '\\' {
            new_s = new_s[1..].to_string();
        }
    }
    ItemSerde {
        item_type: "text".to_string(),
        color: None,
        bg_color: None,
        metric: None,
        imperial: None,
        unit_color: None,
        value: new_s,
        args: None,
        kwargs: None,
        scale: None,
    }
}


fn string_to_row(s: String) -> Vec<ItemSerde> {
    let mut item_list = Vec::new();
    let mut previous_char = '\0';
    let mut current = String::new();
    for c in s.to_string().chars() {
        if (c == '{' || c == '}') && previous_char != '\\' {
            item_list.push(string_to_item(&current));
            current = String::new();
            previous_char = '\0';
        } else {
            current += &c.to_string();
            previous_char = c;
        }
    }
    if !current.is_empty() {
        item_list.push(string_to_item(&current));
    }
    item_list
}

pub fn compile_layout(s: String) -> weather_core::Result<LayoutSerde> {
    let lines: Vec<&str> = s.split("\n").collect();
    let mut rows: Vec<Vec<ItemSerde>> = Vec::new();
    let mut is_layout = false;
    let mut variables: HashMap<&str, String> = HashMap::new();
    for line in lines {
        let stripped_line = strip(line);
        if stripped_line.chars().find(|&x| x != '-' && x != ' ').is_none() {
            is_layout = true;
        }
        else if is_layout {
            rows.push(string_to_row(stripped_line.to_string()));
        }
        else {
            let variable = Regex::new(r#"\w*=\w*"#).unwrap();
            if variable.is_match(line) {
                let split: Vec<&str> = line.split("=").collect();
                let variable = split[0].trim_end().trim_start();
                let value = split[1].trim_end().trim_start();
                variables.insert(variable, value.to_string());
            }
        }
    }
    Ok(LayoutSerde {
        version: variables.get("version").unwrap_or(&weather_core::layout::VERSION.to_string()).parse().unwrap(),
        defaults: LayoutDefaultsSerde {
            variable_color: variables.get("variable_color").unwrap_or(&"FORE_LIGHTGREEN".to_string()).to_string(),
            text_color: variables.get("text_color").unwrap_or(&"FORE_LIGHTBLUE".to_string()).to_string(),
            unit_color: variables.get("unit_color").unwrap_or(&"FORE_MAGENTA".to_string()).to_string(),
            variable_bg_color: variables.get("variable_bg_color").unwrap_or(&"BACK_RESET".to_string()).to_string(),
            text_bg_color: variables.get("text_bg_color").unwrap_or(&"BACK_RESET".to_string()).to_string(),
            unit_bg_color: variables.get("unit_bg_color").unwrap_or(&"BACK_RESET".to_string()).to_string(),
        },
        layout: rows,
    })
}
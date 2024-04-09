use std::collections::HashMap;

use tera::{from_value, to_value, Function, Value};

fn get_required_arg(
    args: &HashMap<String, Value>,
    arg: &str,
    function_name: &str,
) -> tera::Result<Value> {
    match args.get(arg) {
        Some(val) => Ok(val.clone()),
        None => Err(format!("Arg \"{arg}\" not found for {function_name}({arg}=_____)").into()),
    }
}

pub struct Color;

impl Color {
    pub fn new() -> Self {
        Color {}
    }
}

impl Function for Color {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let val = get_required_arg(args, "color", "color")?;
        match from_value::<String>(val.clone()) {
            Ok(v) => Ok(to_value(terminal::color::from_string(&v).unwrap()).unwrap()),
            Err(_) => Err("Could not convert \"color\" from val".into()),
        }
    }
}

pub struct Units;

impl Units {
    pub fn new() -> Self {
        Units {}
    }

    fn temperature(metric: bool) -> tera::Result<Value> {
        if metric {
            return Ok(to_value(format!("°C"))?);
        }
        return Ok(to_value(format!("°F"))?);
    }

    fn wind(metric: bool) -> tera::Result<Value> {
        if metric {
            return Ok(to_value(format!("km/h"))?);
        }
        return Ok(to_value(format!("mph"))?);
    }
}

impl Function for Units {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let val = get_required_arg(args, "metric", "units")?;
        match from_value::<bool>(val.clone()) {
            Ok(metric) => {
                let v = get_required_arg(args, "type", "units")?;
                match from_value::<String>(v.clone()) {
                    Ok(t) => match &*t {
                        "t" | "temp" | "temperature" => Units::temperature(metric),
                        "c" | "clouds" => Ok(to_value(format!("%"))?),
                        "w" | "wind" => Units::wind(metric),
                        _ => Err("Unknown type".into()),
                    },
                    Err(_) => Err("Could not convert \"type\" from val".into()),
                }
            }
            Err(_) => Err("Could not convert \"metric\" from val".into()),
        }
    }
}

pub struct TerminalInfo;

impl TerminalInfo {
    pub fn new() -> Self {
        TerminalInfo {}
    }
}

impl Function for TerminalInfo {
    fn call(&self, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let v = get_required_arg(args, "type", "terminal_info")?;
        match from_value::<String>(v.clone()) {
            Ok(t) => {
                match &*t {
                    "h" | "height" => Ok(to_value(terminal::terminal_size().unwrap().0).unwrap()),
                    "w" | "width" => Ok(to_value(terminal::terminal_size().unwrap().1).unwrap()),
                    "d" | "dimensions" => Ok(to_value(terminal::terminal_size().unwrap()).unwrap()),
                    _ => Err("Unknown type".into()),
                }
            },
            Err(_) => Err("Could not convert \"type\" from val".into()),
        }
    }
}

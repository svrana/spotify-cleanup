use anyhow::{bail, Result};
use regex::Regex;

use spotify_cleanup::{execute_command, stdout_to_string};

#[derive(Clone, Debug)]
pub struct Object {
    pub client_id: String,
    pub pw_type: String,
    pub app_name: String,
}

pub fn list_objects() -> Vec<Object> {
    let output = execute_command("pw-cli", &["ls"]);
    let out_str = stdout_to_string(output);
    let objects = parse_objects(out_str).expect("failed to parse `pw-cli ls` output");
    return objects;
}

pub fn destroy(id: i32) {
    println!("pw-cli destroy {:?}", id);
    execute_command("pw-cli", &["destroy", &id.to_string()]);
}

fn parse_objects(output: String) -> Result<Vec<Object>> {
    let mut objects: Vec<Object> = Vec::new();
    let header = Regex::new(r"id (\d+), type ([\w,-,:/]+)").expect("invalid header regex");
    let details = Regex::new(r"(.*) = (.*)").expect("invalid object details regex");

    for line in output.lines() {
        if header.is_match(line) {
            let caps = header.captures(line).unwrap();
            // TODO: make constructor and use here
            let obj = Object {
                client_id: caps.get(1).unwrap().as_str().to_string(),
                pw_type: caps.get(2).unwrap().as_str().to_string(),
                app_name: "".to_string(),
            };
            objects.push(obj)
        } else {
            if !details.is_match(line) {
                bail!("unexptect line format: {}", line);
            }

            let kv: Vec<&str> = line.split(" = ").collect();
            let keys = *kv.get(0).unwrap();
            let key = keys.trim_start();
            let valuep = *kv.get(1).unwrap();
            let value = valuep.trim_matches(|c| c == '"');
            // TODO: long ass match
            if key == "application.name" {
                let obj = objects
                    .last_mut()
                    .expect("expected an existing pipewire object to exist");
                obj.app_name = value.to_string();
            }
        }
    }

    Ok(objects)
}

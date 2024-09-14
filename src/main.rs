mod pipewire;

fn main() {
    let objects = pipewire::list_objects();
    let mut filtered: Vec<pipewire::Object> = objects
        .into_iter()
        .filter(|o| o.app_name == "spotify" && o.pw_type.starts_with("PipeWire:Interface:Client"))
        .collect();
    // remove the last client so we do not kill any playing music. This assumes pw-cli lists the
    // objects earliest to latest.
    if filtered.len() > 0 {
        println!("ignoring last spotify instance");
        filtered.pop();
    }
    for obj in filtered.iter() {
        let id: i32 = obj.client_id.parse::<i32>().unwrap();
        pipewire::destroy(id)
    }
}

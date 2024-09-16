use std::sync::mpsc;
use std::{cell::Cell, rc::Rc};

use pipewire as pw;

#[derive(Clone, Debug)]
pub struct Object {
    pub client_id: u32,
    pub pw_type: String,
    pub app_name: String,
}

pub fn main() {
    pw::init();

    let mainloop = pw::main_loop::MainLoop::new(None).expect("Failed to create Pipewire mainloop");
    let context = pw::context::Context::new(&mainloop).expect("Failed to create Pipewire context");
    let core = context
        .connect(None)
        .expect("Failed to connect to Pipewire core");
    let registry = core.get_registry().expect("Failed to get Registry");
    let (sender, receiver) = mpsc::channel();

    // Register a callback to the `global` event on the registry, which notifies of any new global objects
    // appearing on the remote.
    // The callback will only get called as long as we keep the returned listener alive.
    let reg_listener = registry
        .add_listener_local()
        .global(move |global| {
            let mut app_name = "";
            if let Some(props) = global.props {
                if props.get("application.name").is_some() {
                    app_name = props
                        .get("application.name")
                        .expect("expected application name");
                }
            }

            let obj = Object {
                client_id: global.id,
                pw_type: global.type_.to_string(),
                app_name: app_name.to_string(),
            };
            sender.send(obj).expect("failed to send")
        })
        .register();

    do_roundtrip(&mainloop, &core);

    std::mem::drop(reg_listener);

    let mut objects: Vec<Object> = Vec::new();
    while let Ok(obj) = receiver.recv() {
        if obj.pw_type == "PipeWire:Interface:Client" && obj.app_name == "spotify" {
            objects.push(obj);
        }
    }

    // not sure what order these are in, but not by object id... seem to be ordered such
    // that the last mentioned one is the one "active" by spotify, such that if you
    // were to destroy it, sound would be lost to spotify
    if let Some(obj) = objects.pop() {
        println!("skipping the last spotify client: {}", obj.client_id);
    }

    for obj in objects {
        println!("destroying object {}", obj.client_id);
        registry
            .destroy_global(obj.client_id)
            .into_result()
            .expect("failed to destroy client");
    }

    // one more roundtrip to make sure we hang around long enough for the objects
    // to be removed
    do_roundtrip(&mainloop, &core);
}

/// Do a single roundtrip to process all events.
fn do_roundtrip(mainloop: &pw::main_loop::MainLoop, core: &pw::core::Core) {
    let done = Rc::new(Cell::new(false));
    let done_clone = done.clone();
    let loop_clone = mainloop.clone();
    // Trigger the sync event. The server's answer won't be processed until we start the main loop,
    // so we can safely do this before setting up a callback. This lets us avoid using a Cell.
    let pending = core.sync(0).expect("sync failed");
    let _listener_core = core
        .add_listener_local()
        .done(move |id, seq| {
            if id == pw::core::PW_ID_CORE && seq == pending {
                done_clone.set(true);
                loop_clone.quit();
            }
        })
        .register();
    while !done.get() {
        mainloop.run();
    }
}

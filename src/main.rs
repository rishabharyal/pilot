use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};
use std::{collections::HashMap, process::Command};
use std::time::Duration;

#[derive(Debug)]
struct KeyAction {
    key: HotKey,
    action: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GlobalHotKeyManager::new()?;
    
    // Simulating a configuration
    let mut hotkey_actions: HashMap<u32, KeyAction> = HashMap::new();
    let alt_c = HotKey::new(Some(Modifiers::ALT), Code::KeyC);
    let alt_a = HotKey::new(Some(Modifiers::ALT), Code::KeyA);
    let alt_p = HotKey::new(Some(Modifiers::ALT), Code::KeyP);

    hotkey_actions.insert(alt_c.id(), KeyAction {
        key: alt_c,
        action: "google-chrome-stable".to_string(),
    });

    hotkey_actions.insert(alt_a.id(), KeyAction {
        key: alt_a,
        action: "rio".to_string(),
    });

    hotkey_actions.insert(alt_p.id(), KeyAction {
        key: alt_p,
        action: "bruno".to_string(),
    });

    // Register all hotkeys
    for hotkey in hotkey_actions.values() {
        manager.register(hotkey.key)?;
    }

    let receiver = GlobalHotKeyEvent::receiver();

    loop {
        match receiver.try_recv() {
            Ok(event) => {
                let event_id = event.id();
                if let Some(action) = hotkey_actions.get(&event_id) {
                    Command::new("sh")
                        .arg("-c")
                        .arg(action.action.clone())
                        .output()
                        .expect("failed to execute process");
                }
            },
            Err(_e) => {
                // either can error occured or unregistered hotkey was pressed
            }
        }
    }
}

use global_hotkey::{hotkey::{Code, HotKey, Modifiers}, GlobalHotKeyEvent, GlobalHotKeyManager};
use std::collections::HashMap;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = GlobalHotKeyManager::new()?;
    
    // Simulating a configuration
    let mut hotkey_actions: HashMap<HotKey, String> = HashMap::new();
    hotkey_actions.insert(HotKey::new(Some(Modifiers::ALT), Code::KeyC), "echo 'Alt+C pressed'".to_string());
    hotkey_actions.insert(HotKey::new(Some(Modifiers::ALT), Code::KeyA), "echo 'Alt+A pressed'".to_string());

    // Register all hotkeys
    for hotkey in hotkey_actions.keys() {
        manager.register(*hotkey)?;
    }

    let receiver = GlobalHotKeyEvent::receiver();

    loop {
        match receiver.try_recv() {
            Ok(event) => {
                println!("Action: {}", event.id());
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

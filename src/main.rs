use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyEventReceiver, GlobalHotKeyManager,
};
use std::error::Error;
use std::{collections::HashMap, process::Command};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;
use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[cfg(target_os = "macos")]
use {
    accessibility_sys::{kAXTrustedCheckOptionPrompt, AXIsProcessTrustedWithOptions},
    core_foundation_sys::{
        base::{CFRelease, TCFTypeRef},
        dictionary::{CFDictionaryAddValue, CFDictionaryCreateMutable},
        number::{kCFBooleanFalse, kCFBooleanTrue},
    },
    std::ptr,
};

#[cfg(target_os = "macos")]
fn check_accessibility(ask_if_not_allowed: bool) -> Result<bool, Box<dyn Error>> {
    let is_allowed;
    unsafe {
        let options =
            CFDictionaryCreateMutable(ptr::null_mut(), 0, std::ptr::null(), std::ptr::null());
        let key = kAXTrustedCheckOptionPrompt;
        let value = if ask_if_not_allowed {
            kCFBooleanTrue
        } else {
            kCFBooleanFalse
        };
        if !options.is_null() {
            CFDictionaryAddValue(options, key.as_void_ptr(), value.as_void_ptr());
            is_allowed = AXIsProcessTrustedWithOptions(options);
            CFRelease(options as *const _);
        } else {
            return Err("options is null".into());
        }
    }
    Ok(is_allowed)
}

#[derive(Debug)]
struct KeyAction {
    key: HotKey,
    #[allow(dead_code)]
    action: String,
}

struct App<'a> {
    window: Option<Window>,
    hotkey_actions: HashMap<u32, KeyAction>,
    receiver: &'a GlobalHotKeyEventReceiver,
    manager: GlobalHotKeyManager,
}

impl<'a> App<'a> {
    fn new() -> Result<Self, Box<dyn Error>> {
        let manager = GlobalHotKeyManager::new()?;
        let mut hotkey_actions = HashMap::new();

        // Initialize hotkeys
        let alt_c = HotKey::new(Some(Modifiers::ALT), Code::KeyO);
        let alt_a = HotKey::new(Some(Modifiers::ALT), Code::KeyA);
        let alt_p = HotKey::new(Some(Modifiers::ALT), Code::KeyP);

        hotkey_actions.insert(
            alt_c.id(),
            KeyAction {
                key: alt_c,
                action: "open -a Bruno".to_string(),
            },
        );
        hotkey_actions.insert(
            alt_a.id(),
            KeyAction {
                key: alt_a,
                action: "open -a Rio".to_string(),
            },
        );
        hotkey_actions.insert(
            alt_p.id(),
            KeyAction {
                key: alt_p,
                action: "open -a Terminal".to_string(),
            },
        );

        // Register all hotkeys
        for hotkey in hotkey_actions.values() {
            manager.register(hotkey.key)?;
        }

        Ok(Self {
            window: None,
            hotkey_actions,
            receiver: GlobalHotKeyEvent::receiver(),
            manager,
        })
    }

    fn handle_hotkey(&self, event_id: u32) {
        if let Some(key_action) = self.hotkey_actions.get(&event_id) {
            println!("Executing action for hotkey: {:?}", event_id);
            if let Err(e) = Command::new(&key_action.action).spawn() {
                eprintln!("Failed to execute command {}: {}", key_action.action, e);
            }
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, _: WindowEvent) {
        match self.receiver.try_recv() {
            Ok(event) => {
                let event_id = event.id();
                self.handle_hotkey(event_id);
            }
            Err(_e) => {
                // either can error occured or unregistered hotkey was pressed
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "macos")]
    {
        let is_allowed = check_accessibility(true).unwrap();
        if !is_allowed {
            println!(
                "Accessibility is not allowed. Please enable accessibility in System Preferences."
            );
            return Ok(());
        }
    }

    // Simulating a configuration
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new()?;
    event_loop.run_app(&mut app)?;

    Ok(())
}

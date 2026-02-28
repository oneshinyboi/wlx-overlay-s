use std::collections::HashMap;
use arboard::{Clipboard, LinuxClipboardKind, SetExtLinux};
use codes_iso_639::part_1::LanguageCode;
use futures::TryFutureExt;
use strum::IntoEnumIterator;
use super_swipe_engine::{EngineLoadError, SwipeEngine};
use swipe_types::types::Point;
use crate::overlays::keyboard::layout;
use crate::overlays::keyboard::layout::KeyCapType;
use crate::subsystem::hid::{get_key_type, KeyType, VirtualKey, XkbKeymap};

pub fn copy_text_to_primary_clipboard(text: &str, clip: &mut Clipboard) {

    println!("{}", std::env::var("WAYLAND_DISPLAY").unwrap());
    clip.set_text(format!("{text} ")).unwrap();
}
pub fn create_new_swipe_engine(keymap: &Option<&XkbKeymap>, layout: &layout::Layout) -> Result<SwipeEngine, EngineLoadError> {
    let layout_name = keymap.and_then(|k| k.get_name()).unwrap_or("us");
    let point_map = build_key_to_char_point_map(*keymap, layout);

    // todo: use the layout_name to choose a sensible language for the swipe engine
    SwipeEngine::new(LanguageCode::En, Some(point_map))
}
fn build_key_to_char_point_map(keymap: Option<&XkbKeymap>, layout: &layout::Layout) -> HashMap<char, Point> {
    let mut map = HashMap::new();

    let has_altgr = keymap.as_ref().is_some_and(|m| XkbKeymap::has_altgr(m));
    let mut pos_x: f32 = 0.0;
    let mut pos_y: f32 = 0.0;

    for (row_idx, row) in layout.main_layout.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {

            let label = layout.get_key_data(keymap, has_altgr, col_idx, row_idx);
            if let Some(label) = label {
                match label.cap_type {
                    KeyCapType::Letter => {
                        if let Some(char) = label.label.iter().next() {
                            let point = Point {x: pos_x as f64, y: pos_y as f64};
                            println!("char: {} \n point: {:?}\n",char, point);
                            map.insert(char.to_ascii_lowercase().chars().next().unwrap(), point);
                        }
                    }
                    _ => {}
                }
            }
            pos_x += layout.key_sizes[row_idx][col_idx];
        }
        pos_y += 1.0;
        pos_x = 0.0;
    }
    map
}

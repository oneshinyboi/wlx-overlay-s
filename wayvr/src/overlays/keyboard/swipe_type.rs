use std::collections::HashMap;
use strum::IntoEnumIterator;
use swipe_types::types::Point;
use crate::overlays::keyboard::layout;
use crate::overlays::keyboard::layout::KeyCapType;
use crate::subsystem::hid::{get_key_type, KeyType, VirtualKey, XkbKeymap};


pub fn build_key_to_char_point_map(keymap: Option<&XkbKeymap>, layout: &layout::Layout) -> HashMap<char, Point> {
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
                            map.insert(char.to_ascii_lowercase().chars().next().unwrap(), Point {x: pos_x as f64, y: pos_y as f64});
                        }
                    }
                    _ => {}
                }
            }
            pos_x += layout.key_sizes[row_idx][col_idx];
        }
        pos_y += layout.row_size;
        pos_x = 0.0;
    }
    map
}

use nanovg_sys::NVGcolor;
use std::collections::HashMap;
use std::os::raw::c_uchar;
use std::sync::Mutex;
pub fn transparent_color() -> NVGcolor {
    nvg_rgba(0, 0, 0, 0)
}

pub const AUTO: f32 = f32::NAN;
pub const YG_UNDEFINED: f32 = f32::NAN;

pub fn nvg_rgb(r: c_uchar, g: c_uchar, b: c_uchar) -> NVGcolor {
    return unsafe { nanovg_sys::nvgRGB(r, g, b) };
}

pub fn nvg_rgba(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> NVGcolor {
    return unsafe { nanovg_sys::nvgRGBA(r, g, b, a) };
}

lazy_static! {
    static ref GLOBAL_THEME: Mutex<HashMap<String, HashMap<String, NVGcolor>>> = Mutex::new(HashMap::from(
        [
            ("LIGHT".into(), HashMap::from([
                // Generic values
                ( "brls/clear".into(), nvg_rgb(235, 235, 235) ),
                ( "brls/background".into(), nvg_rgb(235, 235, 235) ),
                ( "brls/text".into(), nvg_rgb(45, 45, 45) ),
                ( "brls/text_disabled".into(), nvg_rgb(140, 140, 140) ),
                ( "brls/backdrop".into(), nvg_rgba(0, 0, 0, 178) ),
                ( "brls/click_pulse".into(), nvg_rgba(13, 182, 213, 38) ), // same as highlight color1 with different opacity
                ( "brls/accent".into(), nvg_rgb(49, 79, 235) ),

                // Highlight
                ( "brls/highlight/background".into(), nvg_rgb(252, 255, 248) ),
                ( "brls/highlight/color1".into(), nvg_rgb(13, 182, 213) ),
                ( "brls/highlight/color2".into(), nvg_rgb(80, 239, 217) ),

                // AppletFrame
                ( "brls/applet_frame/separator".into(), nvg_rgb(45, 45, 45) ),

                // Sidebar
                ( "brls/sidebar/background".into(), nvg_rgb(240, 240, 240) ),
                ( "brls/sidebar/active_item".into(), nvg_rgb(49, 79, 235) ),
                ( "brls/sidebar/separator".into(), nvg_rgb(208, 208, 208) ),

                // Header
                ( "brls/header/border".into(), nvg_rgb(207, 207, 207) ),
                ( "brls/header/rectangle".into(), nvg_rgb(127, 127, 127) ),
                ( "brls/header/subtitle".into(), nvg_rgb(140, 140, 140) ),

                // Button
                ( "brls/button/primary_enabled_background".into(), nvg_rgb(50, 79, 241) ),
                ( "brls/button/primary_disabled_background".into(), nvg_rgb(201, 201, 209) ),
                ( "brls/button/primary_enabled_text".into(), nvg_rgb(255, 255, 255) ),
                ( "brls/button/primary_disabled_text".into(), nvg_rgb(220, 220, 228) ),

                ( "brls/button/default_enabled_background".into(), nvg_rgb(255, 255, 255) ),
                ( "brls/button/default_disabled_background".into(), nvg_rgb(255, 255, 255) ),
                ( "brls/button/default_enabled_text".into(), nvg_rgb(45, 45, 45) ),
                ( "brls/button/default_disabled_text".into(), nvg_rgb(45, 45, 45) ),

                ( "brls/button/highlight_enabled_text".into(), nvg_rgb(49, 79, 235) ),
                ( "brls/button/highlight_disabled_text".into(), nvg_rgb(49, 79, 235) ),

                ( "brls/button/enabled_border_color".into(), nvg_rgb(45, 45, 45) ),
                ( "brls/button/disabled_border_color".into(), nvg_rgb(45, 45, 45) ),

                // List
                ( "brls/list/listItem_value_color".into(), nvg_rgb(43, 81, 226) ),

                // Slider
                ( "brls/slider/pointer_color".into(), nvg_rgb(255, 255, 255) ),
                ( "brls/slider/pointer_border_color".into(), nvg_rgb(200, 200, 200) ),
                ( "brls/slider/line_filled".into(), nvg_rgb(50, 79, 241) ),
                ( "brls/slider/line_empty".into(), nvg_rgb(140, 140, 140) ),

                // Spinner
                ( "brls/spinner/bar_color".into(), nvg_rgba(131, 131, 131, 80) ),
            ])),
            ("DARK".into(), HashMap::from([
                // Generic values
                ("brls/clear".into(), nvg_rgb(45, 45, 45) ),
                ("brls/background".into(), nvg_rgb(45, 45, 45) ),
                ("brls/text".into(), nvg_rgb(255, 255, 255) ),
                ("brls/text_disabled".into(), nvg_rgb(80, 80, 80) ),
                ("brls/backdrop".into(), nvg_rgba(0, 0, 0, 178) ),
                ("brls/click_pulse".into(), nvg_rgba(25, 138, 198, 38) ), // same as highlight color1 with different opacity
                ("brls/accent".into(), nvg_rgb(0, 255, 204) ),

                // Highlight
                ("brls/highlight/background".into(), nvg_rgb(31, 34, 39) ),
                ("brls/highlight/color1".into(), nvg_rgb(25, 138, 198) ),
                ("brls/highlight/color2".into(), nvg_rgb(137, 241, 242) ),

                // AppletFrame
                ("brls/applet_frame/separator".into(), nvg_rgb(255, 255, 255) ),

                // Sidebar
                ("brls/sidebar/background".into(), nvg_rgb(50, 50, 50) ),
                ("brls/sidebar/active_item".into(), nvg_rgb(0, 255, 204) ),
                ("brls/sidebar/separator".into(), nvg_rgb(81, 81, 81) ),

                // Header
                ("brls/header/border".into(), nvg_rgb(78, 78, 78) ),
                ("brls/header/rectangle".into(), nvg_rgb(160, 160, 160) ),
                ("brls/header/subtitle".into(), nvg_rgb(163, 163, 163) ),

                // Button
                ("brls/button/primary_enabled_background".into(), nvg_rgb(1, 255, 201) ),
                ("brls/button/primary_disabled_background".into(), nvg_rgb(83, 87, 86) ),
                ("brls/button/primary_enabled_text".into(), nvg_rgb(52, 41, 55) ),
                ("brls/button/primary_disabled_text".into(), nvg_rgb(71, 75, 74) ),

                ("brls/button/default_enabled_background".into(), nvg_rgb(80, 80, 80) ),
                ("brls/button/default_disabled_background".into(), nvg_rgb(80, 80, 80) ),
                ("brls/button/default_enabled_text".into(), nvg_rgb(255, 255, 255) ),
                ("brls/button/default_disabled_text".into(), nvg_rgb(255, 255, 255) ),

                ("brls/button/highlight_enabled_text".into(), nvg_rgb(7, 247, 198) ),
                ("brls/button/highlight_disabled_text".into(), nvg_rgb(7, 247, 198) ),

                ("brls/button/enabled_border_color".into(), nvg_rgb(255, 255, 255) ),
                ("brls/button/disabled_border_color".into(), nvg_rgb(255, 255, 255) ),

                // List
                ("brls/list/listItem_value_color".into(), nvg_rgb(88, 195, 169) ),

                // Slider
                ("brls/slider/pointer_color".into(), nvg_rgb(80, 80, 80) ),
                ("brls/slider/pointer_border_color".into(), nvg_rgb(120, 120, 120) ),
                ("brls/slider/line_filled".into(), nvg_rgb(1, 255, 201) ),
                ("brls/slider/line_empty".into(), nvg_rgb(140, 140, 140) ),

                // Spinner
                ("brls/spinner/bar_color".into(), nvg_rgba(192, 192, 192, 80) ),
            ])),
        ]
    ));
}

pub fn theme(theme: &str, key: &str) -> NVGcolor {
    let map = GLOBAL_THEME.lock().unwrap(); // 加锁，获取不可变引用
    map.get(theme)
        .expect(&format!("unknown theme: {}", theme))
        .get(key)
        .unwrap()
        .clone()
}

pub fn add_theme(theme: &str, key: &str, value: NVGcolor) {
    let mut map = GLOBAL_THEME.lock().unwrap();
    map.get_mut(theme)
        .unwrap()
        .insert(key.parse().unwrap(), value);
}

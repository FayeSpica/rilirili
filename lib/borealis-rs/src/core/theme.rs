use nanovg_sys::NVGcolor;
use std::collections::HashMap;
use std::os::raw::c_uchar;
use std::sync::Mutex;
pub fn transparent_color() -> NVGcolor {
    nvgRGBA(0, 0, 0, 0)
}

pub const AUTO: f32 = f32::NAN;
pub const YG_UNDEFINED: f32 = f32::NAN;

pub fn nvgRGB(r: c_uchar, g: c_uchar, b: c_uchar) -> NVGcolor {
    return unsafe { nanovg_sys::nvgRGB(r, g, b) };
}

pub fn nvgRGBA(r: c_uchar, g: c_uchar, b: c_uchar, a: c_uchar) -> NVGcolor {
    return unsafe { nanovg_sys::nvgRGBA(r, g, b, a) };
}

lazy_static! {
    static ref GLOBAL_THEME: Mutex<HashMap<String, HashMap<String, NVGcolor>>> = Mutex::new(HashMap::from(
        [
            ("LIGHT".into(), HashMap::from([
                // Generic values
                ( "brls/clear".into(), nvgRGB(235, 235, 235) ),
                ( "brls/background".into(), nvgRGB(235, 235, 235) ),
                ( "brls/text".into(), nvgRGB(45, 45, 45) ),
                ( "brls/text_disabled".into(), nvgRGB(140, 140, 140) ),
                ( "brls/backdrop".into(), nvgRGBA(0, 0, 0, 178) ),
                ( "brls/click_pulse".into(), nvgRGBA(13, 182, 213, 38) ), // same as highlight color1 with different opacity
                ( "brls/accent".into(), nvgRGB(49, 79, 235) ),

                // Highlight
                ( "brls/highlight/background".into(), nvgRGB(252, 255, 248) ),
                ( "brls/highlight/color1".into(), nvgRGB(13, 182, 213) ),
                ( "brls/highlight/color2".into(), nvgRGB(80, 239, 217) ),

                // AppletFrame
                ( "brls/applet_frame/separator".into(), nvgRGB(45, 45, 45) ),

                // Sidebar
                ( "brls/sidebar/background".into(), nvgRGB(240, 240, 240) ),
                ( "brls/sidebar/active_item".into(), nvgRGB(49, 79, 235) ),
                ( "brls/sidebar/separator".into(), nvgRGB(208, 208, 208) ),

                // Header
                ( "brls/header/border".into(), nvgRGB(207, 207, 207) ),
                ( "brls/header/rectangle".into(), nvgRGB(127, 127, 127) ),
                ( "brls/header/subtitle".into(), nvgRGB(140, 140, 140) ),

                // Button
                ( "brls/button/primary_enabled_background".into(), nvgRGB(50, 79, 241) ),
                ( "brls/button/primary_disabled_background".into(), nvgRGB(201, 201, 209) ),
                ( "brls/button/primary_enabled_text".into(), nvgRGB(255, 255, 255) ),
                ( "brls/button/primary_disabled_text".into(), nvgRGB(220, 220, 228) ),

                ( "brls/button/default_enabled_background".into(), nvgRGB(255, 255, 255) ),
                ( "brls/button/default_disabled_background".into(), nvgRGB(255, 255, 255) ),
                ( "brls/button/default_enabled_text".into(), nvgRGB(45, 45, 45) ),
                ( "brls/button/default_disabled_text".into(), nvgRGB(45, 45, 45) ),

                ( "brls/button/highlight_enabled_text".into(), nvgRGB(49, 79, 235) ),
                ( "brls/button/highlight_disabled_text".into(), nvgRGB(49, 79, 235) ),

                ( "brls/button/enabled_border_color".into(), nvgRGB(45, 45, 45) ),
                ( "brls/button/disabled_border_color".into(), nvgRGB(45, 45, 45) ),

                // List
                ( "brls/list/listItem_value_color".into(), nvgRGB(43, 81, 226) ),

                // Slider
                ( "brls/slider/pointer_color".into(), nvgRGB(255, 255, 255) ),
                ( "brls/slider/pointer_border_color".into(), nvgRGB(200, 200, 200) ),
                ( "brls/slider/line_filled".into(), nvgRGB(50, 79, 241) ),
                ( "brls/slider/line_empty".into(), nvgRGB(140, 140, 140) ),

                // Spinner
                ( "brls/spinner/bar_color".into(), nvgRGBA(131, 131, 131, 80) ),
            ])),
            ("DARK".into(), HashMap::from([
                // Generic values
                ("brls/clear".into(), nvgRGB(45, 45, 45) ),
                ("brls/background".into(), nvgRGB(45, 45, 45) ),
                ("brls/text".into(), nvgRGB(255, 255, 255) ),
                ("brls/text_disabled".into(), nvgRGB(80, 80, 80) ),
                ("brls/backdrop".into(), nvgRGBA(0, 0, 0, 178) ),
                ("brls/click_pulse".into(), nvgRGBA(25, 138, 198, 38) ), // same as highlight color1 with different opacity
                ("brls/accent".into(), nvgRGB(0, 255, 204) ),

                // Highlight
                ("brls/highlight/background".into(), nvgRGB(31, 34, 39) ),
                ("brls/highlight/color1".into(), nvgRGB(25, 138, 198) ),
                ("brls/highlight/color2".into(), nvgRGB(137, 241, 242) ),

                // AppletFrame
                ("brls/applet_frame/separator".into(), nvgRGB(255, 255, 255) ),

                // Sidebar
                ("brls/sidebar/background".into(), nvgRGB(50, 50, 50) ),
                ("brls/sidebar/active_item".into(), nvgRGB(0, 255, 204) ),
                ("brls/sidebar/separator".into(), nvgRGB(81, 81, 81) ),

                // Header
                ("brls/header/border".into(), nvgRGB(78, 78, 78) ),
                ("brls/header/rectangle".into(), nvgRGB(160, 160, 160) ),
                ("brls/header/subtitle".into(), nvgRGB(163, 163, 163) ),

                // Button
                ("brls/button/primary_enabled_background".into(), nvgRGB(1, 255, 201) ),
                ("brls/button/primary_disabled_background".into(), nvgRGB(83, 87, 86) ),
                ("brls/button/primary_enabled_text".into(), nvgRGB(52, 41, 55) ),
                ("brls/button/primary_disabled_text".into(), nvgRGB(71, 75, 74) ),

                ("brls/button/default_enabled_background".into(), nvgRGB(80, 80, 80) ),
                ("brls/button/default_disabled_background".into(), nvgRGB(80, 80, 80) ),
                ("brls/button/default_enabled_text".into(), nvgRGB(255, 255, 255) ),
                ("brls/button/default_disabled_text".into(), nvgRGB(255, 255, 255) ),

                ("brls/button/highlight_enabled_text".into(), nvgRGB(7, 247, 198) ),
                ("brls/button/highlight_disabled_text".into(), nvgRGB(7, 247, 198) ),

                ("brls/button/enabled_border_color".into(), nvgRGB(255, 255, 255) ),
                ("brls/button/disabled_border_color".into(), nvgRGB(255, 255, 255) ),

                // List
                ("brls/list/listItem_value_color".into(), nvgRGB(88, 195, 169) ),

                // Slider
                ("brls/slider/pointer_color".into(), nvgRGB(80, 80, 80) ),
                ("brls/slider/pointer_border_color".into(), nvgRGB(120, 120, 120) ),
                ("brls/slider/line_filled".into(), nvgRGB(1, 255, 201) ),
                ("brls/slider/line_empty".into(), nvgRGB(140, 140, 140) ),

                // Spinner
                ("brls/spinner/bar_color".into(), nvgRGBA(192, 192, 192, 80) ),
            ])),
        ]
    ));
}

pub fn theme(theme: &str, key: &str) -> NVGcolor {
    let map = GLOBAL_THEME.lock().unwrap(); // 加锁，获取不可变引用
    map.get(theme).expect(&format!("unknown theme: {}", theme)).get(key).unwrap().clone()
}

pub fn add_theme(theme: &str, key: &str, value: NVGcolor) {
    let mut map = GLOBAL_THEME.lock().unwrap();
    map.get_mut(theme)
        .unwrap()
        .insert(key.parse().unwrap(), value);
}

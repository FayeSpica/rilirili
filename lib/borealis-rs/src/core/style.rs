use std::collections::HashMap;
use std::sync::Mutex;

pub struct Style;

lazy_static! {
    static ref GLOBAL_STYLE: Mutex<HashMap<String, f32>> = Mutex::new(HashMap::from(
        [
            // Animations
            (String::from("brls/animations/show"), 200.0),
            (String::from("brls/animations/show_slide"), 125.0),

            (String::from("brls/animations/highlight"), 200.0),
            (String::from("brls/animations/highlight_shake"), 15.0),

            (String::from("brls/animations/label_scrolling_timer"), 1500.0),
            (String::from("brls/animations/label_scrolling_speed"), 0.05),

            // Highlight
            (String::from("brls/highlight/stroke_width"), 5.0),
            (String::from("brls/highlight/corner_radius"), 6.0),
            (String::from("brls/highlight/shadow_width"), 2.0),
            (String::from("brls/highlight/shadow_offset"), 10.0),
            (String::from("brls/highlight/shadow_feather"), 10.0),
            (String::from("brls/highlight/shadow_opacity"), 128.0),

            // AppletFrame
            (String::from("brls/applet_frame/padding_sides"), 30.0),

            (String::from("brls/applet_frame/header_height"), 88.0),
            (String::from("brls/applet_frame/header_padding_top_bottom"), 15.0),
            (String::from("brls/applet_frame/header_padding_sides"), 35.0),
            (String::from("brls/applet_frame/header_image_title_spacing"), 18.0),
            (String::from("brls/applet_frame/header_title_font_size"), 28.0),
            (String::from("brls/applet_frame/header_title_top_offset"), 7.0),

            (String::from("brls/applet_frame/footer_height"), 73.0),
            (String::from("brls/applet_frame/footer_padding_top_bottom"), 20.0),
            (String::from("brls/applet_frame/footer_padding_sides"), 25.0),

            // TabFrame
            (String::from("brls/tab_frame/sidebar_width"), 410.0),
            (String::from("brls/tab_frame/content_padding_top_bottom"), 42.0), // unused by the library, here for users
            (String::from("brls/tab_frame/content_padding_sides"), 60.0), // unused by the library, here for users

            // Sidebar
            (String::from("brls/sidebar/border_height"), 16.0),
            (String::from("brls/sidebar/padding_top"), 32.0),
            (String::from("brls/sidebar/padding_bottom"), 47.0),
            (String::from("brls/sidebar/padding_left"), 80.0),
            (String::from("brls/sidebar/padding_right"), 40.0),
            (String::from("brls/sidebar/item_height"), 70.0),
            (String::from("brls/sidebar/item_accent_margin_top_bottom"), 9.0),
            (String::from("brls/sidebar/item_accent_margin_sides"), 8.0),
            (String::from("brls/sidebar/item_accent_rect_width"), 4.0),
            (String::from("brls/sidebar/item_font_size"), 22.0),
            (String::from("brls/sidebar/separator_height"), 30.0),

            // Tab Details
            (String::from("brls/tab_details/padding_top"), 32.0),
            (String::from("brls/tab_details/padding_bottom"), 47.0),
            (String::from("brls/tab_details/padding_left"), 60.0),
            (String::from("brls/tab_details/padding_right"), 80.0),

            // Label
            (String::from("brls/label/default_font_size"), 20.0),
            (String::from("brls/label/default_line_height"), 1.65),
            (String::from("brls/label/scrolling_animation_spacing"), 50.0),
            (String::from("brls/label/highlight_padding"), 2.0),

            // Header
            (String::from("brls/header/padding_top_bottom"), 11.0),
            (String::from("brls/header/padding_right"), 11.0),
            (String::from("brls/header/rectangle_width"), 5.0),
            (String::from("brls/header/rectangle_height"), 33.0),
            (String::from("brls/header/rectangle_margin"), 10.0),
            (String::from("brls/header/font_size"), 18.0),

            // Button
            (String::from("brls/button/padding_top_bottom"), 15.0),
            (String::from("brls/button/padding_sides"), 25.0),
            (String::from("brls/button/corner_radius"), 5.0),
            (String::from("brls/button/text_size"), 18.0),
            (String::from("brls/button/primary_highlight_padding"), 2.0),
            (String::from("brls/button/border_thickness"), 2.0),

            // Generic shadow
            (String::from("brls/shadow/width"), 2.0),
            (String::from("brls/shadow/feather"), 10.0),
            (String::from("brls/shadow/opacity"), 63.75),
            (String::from("brls/shadow/offset"), 10.0),

            // Dropdown
            (String::from("brls/dropdown/listPadding"), 40.0),
            (String::from("brls/dropdown/listPaddingSides"), 232.0),
            (String::from("brls/dropdown/listItemHeight"), 60.0),
            (String::from("brls/dropdown/listItemTextSize"), 20.0),

            (String::from("brls/dropdown/header_height"), 70.0),
            (String::from("brls/dropdown/header_title_font_size"), 24.0),

            // ListItem
            (String::from("brls/listitem/descriptionIndent"), 16.0),
            (String::from("brls/listitem/indent"), 40.0),
            (String::from("brls/listitem/selectRadius"), 15.0),

            // Hints
            (String::from("brls/hints/footer_margin_sides"), 30.0),
            (String::from("brls/hints/footer_padding_sides"), 25.0),
            (String::from("brls/hints/footer_padding_top_bottom"), 8.0),

            // Spinner
            (String::from("brls/spinner/center_gap_multiplier_large"), 0.207),
            (String::from("brls/spinner/bar_width_multiplier_large"), 0.034),
            (String::from("brls/spinner/center_gap_multiplier"), 0.2),
            (String::from("brls/spinner/bar_width_multiplier"), 0.06),
            (String::from("brls/spinner/animation_duration"), 1000.0),

            // Dialog
            (String::from("brls/dialog/paddingTopBottom"), 65.0),
            (String::from("brls/dialog/paddingLeftRight"), 115.0),

            (String::from("brls/dialog/fontSize"), 24.0),
        ]
    ));
}

pub fn style(key: &str) -> f32 {
    let map = GLOBAL_STYLE.lock().unwrap(); // 加锁，获取不可变引用
    map.get(key).unwrap().clone()
}

pub fn add_style(key: &str, value: f32) {
    let mut map = GLOBAL_STYLE.lock().unwrap();
    map.insert(key.parse().unwrap(), value);
}

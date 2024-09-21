use crate::core::activity::Activity;
use crate::core::application;
use crate::core::platform::PlatformDyn;
use crate::core::style::add_style;
use crate::core::theme::{add_theme_color, nvg_rgb, ThemeVariant};
use crate::demo::activity::main_activity::MainActivity;
use crate::demo::tab::captioned_image::CaptionedImage;
use crate::demo::tab::components_tab::ComponentsTab;
use crate::demo::tab::recycling_list_tab::RecyclingListTab;

pub mod activity;
pub mod tab;

pub fn main() {
    let mut application = application::Application::create_window("demo/title");

    application
        .platform_mut()
        .set_theme_variant(ThemeVariant::LIGHT);

    application.set_global_quit(true);

    // Register custom views (including tabs, which are views)
    application.register_xml_view("CaptionedImage", Box::new(CaptionedImage::create));
    application.register_xml_view("RecyclingListTab", Box::new(RecyclingListTab::create));
    application.register_xml_view("ComponentsTab", Box::new(ComponentsTab::create));

    // Add custom values to the theme
    add_theme_color("LIGHT", "captioned_image/caption", nvg_rgb(2, 176, 183));
    add_theme_color("DARK", "captioned_image/caption", nvg_rgb(51, 186, 227));

    // Add custom values to the style
    add_style("about/padding_top_bottom", 50.0);
    add_style("about/padding_sides", 75.0);
    add_style("about/description_margin", 50.0);

    let activity = MainActivity::new(application.video_subsystem().clone());

    application.push_activity(Activity::MainActivity(activity));
    application.set_limited_fps(60);
    application.main_loop();

    info!("main_loop done");
}

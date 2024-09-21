use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use nanovg_sys::NVGcolor;
use crate::core::application::ViewCreatorRegistry;
use crate::core::theme::AUTO;
use crate::core::view_base::{AlignSelf, FocusDirection, PositionType, ShadowType, View, ViewBackground, ViewBase, Visibility};
use crate::core::view_layout::ViewLayout;
use crate::core::view_style::ViewStyle;

pub type AutoAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>)>;
pub type IntAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, i32)>;
pub type FloatAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, f32)>;
pub type StringAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, &str)>;
pub type ColorAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, NVGcolor)>;
pub type BoolAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, bool)>;
pub type FilePathAttributeHandler = Box<dyn Fn(Rc<RefCell<View>>, &str)>;


pub struct AttributeSetter {
    pub auto_attributes: HashMap<String, AutoAttributeHandler>,
    pub percentage_attributes: HashMap<String, FloatAttributeHandler>,
    pub float_attributes: HashMap<String, FloatAttributeHandler>,
    pub string_attributes: HashMap<String, StringAttributeHandler>,
    pub color_attributes: HashMap<String, ColorAttributeHandler>,
    pub bool_attributes: HashMap<String, BoolAttributeHandler>,
    pub file_path_attributes: HashMap<String, FilePathAttributeHandler>,
    pub known_attributes: Vec<String>,
}

impl Default for AttributeSetter {
    fn default() -> Self {
        let mut s = Self {
            auto_attributes: Default::default(),
            percentage_attributes: Default::default(),
            float_attributes: Default::default(),
            string_attributes: Default::default(),
            color_attributes: Default::default(),
            bool_attributes: Default::default(),
            file_path_attributes: Default::default(),
            known_attributes: vec![],
        };
        s.register_common_attributes();
        s
    }
}

impl AttributeSetter {

    fn register_common_attributes(&mut self) {
        // Width
        self.register_auto_xml_attribute("width", Box::new(|view_clone| {
            view_clone.borrow_mut().set_width(AUTO);
        }));

        self.register_float_xml_attribute("width", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_width(value);
        }));

        self.register_percentage_xml_attribute("width", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_width_percentage(value);
        }));

        // Height
        self.register_auto_xml_attribute("height", Box::new(|view_clone| {
            view_clone.borrow_mut().set_height(AUTO);
        }));

        self.register_float_xml_attribute("height", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_height(value);
        }));

        self.register_percentage_xml_attribute("height", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_height_percentage(value);
        }));

        // Max width
        self.register_auto_xml_attribute("maxWidth", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_max_width(AUTO);
        }));

        self.register_float_xml_attribute("maxWidth", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_max_width(value);
        }));

        self.register_percentage_xml_attribute("maxWidth", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_max_width_percentage(value);
        }));

        // Max Height
        self.register_auto_xml_attribute("maxHeight", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_max_height(AUTO);
        }));


        self.register_float_xml_attribute("maxHeight", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_max_height(value);
        }));


        self.register_percentage_xml_attribute("maxHeight", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_max_height_percentage(value);
        }));

        // Grow and shrink

        self.register_float_xml_attribute("grow", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_grow(value);
        }));


        self.register_percentage_xml_attribute("shrink", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_shrink(value);
        }));

        // Alignment

        self.register_string_xml_attribute("alignSelf", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_align_self( match value {
                "auto" => AlignSelf::Auto,
                "flexStart" => AlignSelf::FlexStart,
                "center" => AlignSelf::Center,
                "flexEnd" => AlignSelf::FlexEnd,
                "stretch" => AlignSelf::Stretch,
                "baseline" => AlignSelf::Baseline,
                "spaceBetween" => AlignSelf::SpaceBetween,
                "spaceAround" => AlignSelf::SpaceAround,
                &_ => AlignSelf::Auto,
            });
        }));

        // Margins top

        self.register_float_xml_attribute("marginTop", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_margin_top(value);
        }));


        self.register_auto_xml_attribute("marginTop", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_margin_top(AUTO);
        }));

        // Margins right

        self.register_float_xml_attribute("marginRight", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_margin_right(value);
        }));


        self.register_auto_xml_attribute("marginRight", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_margin_right(AUTO);
        }));

        // Margins bottom

        self.register_float_xml_attribute("marginBottom", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_margin_bottom(value);
        }));


        self.register_auto_xml_attribute("marginBottom", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_margin_bottom(AUTO);
        }));

        // Margins left

        self.register_float_xml_attribute("marginLeft", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_margin_left(value);
        }));


        self.register_auto_xml_attribute("marginLeft", Box::new(|view_clone,| {
            view_clone.borrow_mut().set_margin_left(AUTO);
        }));

        // Line
        self.register_color_xml_attribute("lineColor", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_line_color(value);
        }));


        self.register_float_xml_attribute("lineTop", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_line_top(value);
        }));


        self.register_float_xml_attribute("lineRight", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_line_right(value);
        }));


        self.register_float_xml_attribute("lineBottom", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_line_bottom(value);
        }));


        self.register_float_xml_attribute("lineLeft", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_line_left(value);
        }));

        // Position

        self.register_float_xml_attribute("positionTop", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_top(value);
        }));


        self.register_float_xml_attribute("positionRight", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_right(value);
        }));


        self.register_float_xml_attribute("positionBottom", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_bottom(value);
        }));


        self.register_float_xml_attribute("positionLeft", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_left(value);
        }));


        self.register_percentage_xml_attribute("positionTop", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_top_percentage(value);
        }));


        self.register_percentage_xml_attribute("positionRight", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_right_percentage(value);
        }));


        self.register_percentage_xml_attribute("positionBottom", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_bottom_percentage(value);
        }));


        self.register_percentage_xml_attribute("positionLeft", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_left_percentage(value);
        }));


        self.register_string_xml_attribute("positionType", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_position_type( match value {
                "relative" => PositionType::Relative,
                "absolute" => PositionType::Absolute,
                &_ => PositionType::Relative,
            });
        }));

        // Custom focus routes

        self.register_string_xml_attribute("focusUp", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Up, value);
        }));


        self.register_string_xml_attribute("focusRight", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Right, value);
        }));


        self.register_string_xml_attribute("focusDown", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Down, value);
        }));

        self.register_string_xml_attribute("focusLeft", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_custom_navigation_route_by_id(FocusDirection::Left, value);
        }));

        // Shape
        self.register_color_xml_attribute("backgroundColor", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_background_color(value);
        }));

        self.register_color_xml_attribute("borderColor", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_border_color(value);
        }));


        self.register_float_xml_attribute("borderThickness", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_border_thickness(value);
        }));


        self.register_float_xml_attribute("cornerRadius", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_corner_radius(value);
        }));


        self.register_string_xml_attribute("shadowType", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_shadow_type( match value {
                "none" => ShadowType::None,
                "generic" => ShadowType::Generic,
                "custom" => ShadowType::Custom,
                &_ => ShadowType::None,
            });
        }));

        // Misc

        self.register_string_xml_attribute("visibility", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_visibility( match value {
                "visible" => Visibility::Visible,
                "invisible" => Visibility::Invisible,
                "gone" => Visibility::Gone,
                &_ => Visibility::Visible,
            });
        }));


        self.register_string_xml_attribute("id", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_id(value);
        }));


        self.register_string_xml_attribute("background", Box::new(|view_clone,value| {
            view_clone.borrow_mut().set_background( match value {
                "sidebar" => ViewBackground::SideBar,
                "backdrop" => ViewBackground::BackDrop,
                &_ => ViewBackground::None,
            });
        }));


        self.register_bool_xml_attribute("focusable", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_focusable(value);
        }));


        self.register_bool_xml_attribute("wireframe", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_wireframe_enabled(value);
        }));

        // Highlight

        self.register_bool_xml_attribute("hideHighlightBackground", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_hide_highlight_background(value);
        }));


        self.register_float_xml_attribute("highlightPadding", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_highlight_padding(value);
        }));


        self.register_float_xml_attribute("highlightCornerRadius", Box::new(|view_clone, value| {
            view_clone.borrow_mut().set_highlight_corner_radius(value);
        }));
    }

    fn register_auto_xml_attribute(&mut self, name: &str, handler: AutoAttributeHandler) {
        self.auto_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    fn register_float_xml_attribute(&mut self, name: &str, handler: FloatAttributeHandler) {
        self.float_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    fn register_percentage_xml_attribute(&mut self, name: &str, handler: FloatAttributeHandler) {
        self.percentage_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    fn register_string_xml_attribute(&mut self, name: &str, handler: StringAttributeHandler) {
        self.string_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    fn register_color_xml_attribute(&mut self, name: &str, handler: ColorAttributeHandler) {
        self.color_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    fn register_bool_xml_attribute(&mut self, name: &str, handler: BoolAttributeHandler) {
        self.bool_attributes.insert(name.into(), handler);
        self.known_attributes.push(name.into());
    }

    pub fn apply_xml_attributes(
        &self,
        view: Rc<RefCell<View>>,
        element: roxmltree::Node,
        view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
    ) {
        for attribute in element.attributes() {
            info!(
                "apply_xml_attributes: {} {}",
                attribute.name(),
                attribute.value()
            );
            self.apply_xml_attribute(view.clone(), attribute.name(), attribute.value());
        }
    }

    pub fn apply_xml_attribute(&self, view: Rc<RefCell<View>>, name: &str, value: &str) -> bool {
        // // String -> string
        // if let Some(handler) = view.data().string_attributes.get(name) {
        //     if value.starts_with("@i18n/") {
        //         todo!();
        //         return true;
        //     }
        //
        //     handler(view, value);
        //     return true;
        // }
        //
        // // File path -> file path
        // if value.starts_with("@res/") {
        //     let path = get_file_path_xml_attribute_value(value);
        //
        //     if let Some(handler) = view.data().file_path_attributes.get(name) {
        //         handler(view, value);
        //         return true;
        //     } else {
        //         return false; // unknown res
        //     }
        // } else {
        //     if let Some(handler) = view.data().file_path_attributes.get(name) {
        //         handler(view, value);
        //         return true;
        //     }
        //
        //     // don't return false as it can be anything else
        // }
        //
        // Auto -> auto
        if "auto" == value {
            if let Some(handler) = self.auto_attributes.get(name) {
                handler(view.clone());
                return true;
            } else {
                info!("{:?}", self.auto_attributes.keys());
                return false;
            }
        }
        //
        // // Ends with px -> float
        // if value.ends_with("px") {
        //     // Strip the px and parse the float value
        //     let new_float = &value[..value.len() - 2];
        //
        //     if let Ok(float_value) = f32::from_str(new_float) {
        //         if let Some(handler) = view.data().float_attributes.get(name) {
        //             handler(view, float_value);
        //             return true;
        //         } else {
        //             return false;
        //         }
        //     } else {
        //         return false;
        //     }
        // }
        //
        // // Ends with % -> percentage
        // if value.ends_with("%") {
        //     // Strip the % and parse the float value
        //     let new_float = &value[..value.len() - 1];
        //
        //     if let Ok(float_value) = f32::from_str(new_float) {
        //
        //         if float_value < -100.0 || float_value > 100.0 {
        //             return false;
        //         }
        //
        //         if let Some(handler) = view.data().float_attributes.get(name) {
        //             handler(view, float_value);
        //             return true;
        //         } else {
        //             return false;
        //         }
        //     } else {
        //         return false;
        //     }
        // }
        // // Starts with @style -> float
        // else if value.starts_with("@style/") {
        //     // Parse the style name
        //     let style_name = &value[7..]; // length of "@style/"
        //     let float_value = style(style_name);
        //
        //     if let Some(handler) = view.data().float_attributes.get(name) {
        //         handler(view, float_value);
        //         return true;
        //     } else {
        //         return false;
        //     }
        // }
        // // Starts with with # -> color
        // else if value.starts_with("#") {
        //     // Parse the color
        //     // #RRGGBB format
        //     if value.len() == 7 {
        //         if let Some((r, g, b)) = hex_to_rgb(value) {
        //             if let Some(handler) = view.data().color_attributes.get(name) {
        //                 handler(view, nvg_rgb(r, g, b));
        //                 return true;
        //             } else {
        //                 return false;
        //             }
        //         } else {
        //             return false;
        //         }
        //     }
        //     // #RRGGBBAA format
        //     else if value.len() == 9 {
        //         if let Some((r, g, b, a)) = hex_to_rgba(value) {
        //             if let Some(handler) = view.data().color_attributes.get(name) {
        //                 handler(view, nvg_rgba(r, g, b, a));
        //                 return true;
        //             } else {
        //                 return false;
        //             }
        //         } else {
        //             return false;
        //         }
        //     } else {
        //         return false;
        //     }
        // }
        // // Starts with @theme -> color
        // else if value.starts_with("@theme/") {
        //     // Parse the color name
        //     let style_name = &value[7..]; // length of "@style/"
        //     let value = theme(style_name);
        //
        //     if let Some(handler) = view.data().color_attributes.get(name) {
        //         handler(view, value);
        //         return true;
        //     } else {
        //         return false;
        //     }
        // }
        // // Equals true or false -> bool
        // else if value == "true" || value == "false" {
        //     let bool_value = if value == "true" {
        //         true
        //     } else {
        //         false
        //     };
        //
        //     if let Some(handler) = view.data().bool_attributes.get(name) {
        //         handler(view, bool_value);
        //         return true;
        //     } else {
        //         return false;
        //     }
        // }
        //
        // Valid float -> float, otherwise unknown attribute
        if let Ok(float_value) = f32::from_str(value) {
            if let Some(handler) = self.float_attributes.get(name) {
                handler(view.clone(), float_value);
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
}
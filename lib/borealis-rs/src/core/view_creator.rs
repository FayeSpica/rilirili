use crate::core::activity::Activity;
use crate::core::application::ViewCreatorRegistry;
use crate::core::view_base::{View, ViewBase};
use crate::core::view_box::{BoxEnum, BoxTrait, BoxView};
use crate::core::view_layout::ViewLayout;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
use crate::core::attribute::{apply_xml_attributes, AttributeSetter};

pub(crate) const CUSTOM_RESOURCES_PATH: &str = "resources";

/**
 * Creates a view from the given XML file content.
 *
 * The method handleXMLElement() is executed for each child node in the XML.
 *
 * Uses the internal lookup table to instantiate the views.
 * Use registerXMLView() to add your own views to the table so that
 * you can use them in your own XML files.
 */
pub fn create_from_xml_string(
    xml_data: String,
    view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
) -> Rc<RefCell<View>> {
    // trace!("create_from_xml_string: {}", xml_data);
    let xml_data = xml_data.replace("brls:", "");
    let document = roxmltree::Document::parse(&xml_data).unwrap();
    let element = document.root_element();
    create_from_xml_element(element, view_creator_registry)
}

/**
 * Creates a view from the given XML file path.
 *
 * The method handleXMLElement() is executed for each child node in the XML.
 *
 * Uses the internal lookup table to instantiate the views.
 * Use registerXMLView() to add your own views to the table so that
 * you can use them in your own XML files.
 */
pub fn create_from_xml_file(
    name: PathBuf,
    view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
) -> Rc<RefCell<View>> {
    trace!("create_from_xml_file: {:?}", name);
    create_from_xml_string(
        std::fs::read_to_string(name).unwrap(),
        view_creator_registry,
    )
}

/**
 * Creates a view from the given XML resource file name.
 *
 * The method handleXMLElement() is executed for each child node in the XML.
 *
 * Uses the internal lookup table to instantiate the views.
 * Use registerXMLView() to add your own views to the table so that
 * you can use them in your own XML files.
 */
pub fn create_from_xml_resource(
    name: PathBuf,
    view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
) -> Rc<RefCell<View>> {
    let path_buf: PathBuf = PathBuf::from(CUSTOM_RESOURCES_PATH);
    create_from_xml_file(path_buf.join("xml").join(name), view_creator_registry)
}

pub fn create_from_xml_element(
    element: roxmltree::Node,
    view_creator_registry: &Rc<RefCell<ViewCreatorRegistry>>,
) -> Rc<RefCell<View>> {
    // 在此处理 XML 元素并返回 View 实例
    // 例如，解析节点，生成视图
    // info!("create_from_xml_element: {:?}", element);
    let view_name = element.tag_name().name();

    // Special case where element name is brls:View: create from given XML file.
    // XML attributes are explicitely not passed down to the created view.
    // To create a custom view from XML that you can reuse in other XML files,
    // make a class inheriting brls::Box and use the inflateFromXML* methods.
    let mut view = if view_name == "View" {
        if let Some(xml_attribute) = element.attribute("xml") {
            create_from_xml_file(
                get_file_path_xml_attribute_value(xml_attribute)
                    .parse()
                    .unwrap(),
                view_creator_registry,
            )
        } else {
            panic!(r#"View XML tag must have an "xml" attribute"#)
        }
    } else {
        // Otherwise look in the register
        let r = view_creator_registry.borrow();
        let view_creator = r.xml_view_creator(view_name);
        let viw_creator = view_creator.expect(&format!("view {} not found", view_name));
        let mut tmp_view = viw_creator();
        // Register common XML attributes
        tmp_view.borrow_mut().set_view(Some(tmp_view.clone()));
        apply_xml_attributes(tmp_view.clone(), element, view_creator_registry);

        tmp_view
    };
    for child in element.children() {
        view.borrow_mut()
            .handle_xml_attributes(child, view_creator_registry);
    }

    view
}

const BRLS_RESOURCES: &str = "./resources/";

pub fn get_file_path_xml_attribute_value(value: &str) -> String {
    if value.starts_with("@res/") {
        return format!("{}{}", BRLS_RESOURCES, &value[5..]);
    }
    format!("{}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_xml() {
        let view = create_from_xml_string(
            r#"
            <brls:View xml="@res/xml/tabs/layout.xml" />
        "#
            .into(),
            &Rc::new(RefCell::new(ViewCreatorRegistry::new())),
        );
    }
}

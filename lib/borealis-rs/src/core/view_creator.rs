use crate::core::activity::Activity;
use crate::core::view_base::{View, ViewBase};
use crate::core::view_box::{BoxEnum, BoxTrait, BoxView};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
const CUSTOM_RESOURCES_PATH: &str = "resources";

pub trait ViewCreator {
    /**
     * Creates a view from the given XML file content.
     *
     * The method handleXMLElement() is executed for each child node in the XML.
     *
     * Uses the internal lookup table to instantiate the views.
     * Use registerXMLView() to add your own views to the table so that
     * you can use them in your own XML files.
     */
    fn create_from_xml_string(&self, xml: String) -> Rc<RefCell<View>> {
        todo!()
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
    fn create_from_xml_file(&self, name: PathBuf) -> Rc<RefCell<View>> {
        trace!("create_from_xml_file: {:?}", name);

        // let file = File::open(name).expect("Unable to open file");
        // let file = BufReader::new(file);
        //
        // let mut reader = Reader::from_reader(file);
        // reader.config_mut().trim_text(true);
        //
        // let mut buf = Vec::new();
        // loop {
        //     buf.clear();
        //     match reader.read_event_into(&mut buf).unwrap() {
        //         Event::Eof => break,
        //         Event::Start(ref e) => {
        //             trace!("{:?} start", e.name());
        //
        //             for attr in e.attributes() {
        //                 if let Ok(attr) = attr {
        //                     trace!(
        //                         "              {:?}={:?}",
        //                         attr.key,
        //                         String::from_utf8(Vec::from(attr.value))
        //                     )
        //                 }
        //             }
        //         }
        //         Event::End(ref e) => {
        //             trace!("{:?} end", e.name());
        //         }
        //         _ => {}
        //     }
        // }

        let box_view = BoxView::new(0.0, 0.0, 0.0, 0.0);
        let mut box_enum = BoxEnum::Box(box_view);
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(
            BoxView::new(100.0, 100.0, 80.0, 80.0),
        )))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(
            BoxView::new(200.0, 100.0, 80.0, 80.0),
        )))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(
            BoxView::new(300.0, 100.0, 80.0, 80.0),
        )))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(
            BoxView::new(400.0, 100.0, 80.0, 80.0),
        )))));
        box_enum.add_view(Rc::new(RefCell::new(View::Box(BoxEnum::Box(
            BoxView::new(500.0, 100.0, 80.0, 80.0),
        )))));
        let view = Rc::new(RefCell::new(View::Box(box_enum)));
        let view_self = view.clone();
        view.borrow_mut().set_view(Some(view_self));
        view
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
    fn create_from_xml_resource(&self, name: PathBuf) -> Rc<RefCell<View>> {
        let path_buf: PathBuf = PathBuf::from(CUSTOM_RESOURCES_PATH);
        self.create_from_xml_file(path_buf.join("xml").join(name))
    }
}

impl ViewCreator for Activity {}

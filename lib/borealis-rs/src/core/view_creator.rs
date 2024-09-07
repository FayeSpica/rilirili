use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::core::activity::Activity;
use crate::core::view_base::View;

const CUSTOM_RESOURCES_PATH: &str = "resources";

pub trait ViewCreator {
    fn create_from_xml_resource(&self, name: PathBuf) -> View {
        let path_buf: PathBuf = PathBuf::from(CUSTOM_RESOURCES_PATH);
        self.create_from_xml_file(path_buf.join("xml").join(name))
    }

    fn create_from_xml_file(&self, name: PathBuf) -> View {
        trace!("create_from_xml_file: {:?}", name);

        let file = File::open(name).expect("Unable to open file");
        let file = BufReader::new(file);

        let mut reader = Reader::from_reader(file);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        loop {
            buf.clear();
            match reader.read_event_into(&mut buf).unwrap() {
                Event::Eof => break,
                Event::Start(ref e) => {
                    trace!("{:?} start", e.name());

                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            trace!("              {:?}={:?}", attr.key, String::from_utf8(Vec::from(attr.value)))
                        }
                    }
                },
                Event::End(ref e) => {
                    trace!("{:?} end", e.name());
                },
                _ => {}
            }
        }

        todo!()
    }
}

impl ViewCreator for Activity {}
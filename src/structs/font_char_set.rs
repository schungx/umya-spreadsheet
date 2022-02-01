// family
use super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct FontCharSet {
    pub(crate) val: Int32Value,
}
impl FontCharSet {
    pub fn get_val(&self) -> &i32 {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value: i32) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"val") {
            Some(v) => {
                self.val.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // charset
        if &self.val.has_value() == &true {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("val", &self.val.get_value_string()));
            write_start_tag(writer, "charset", attributes, true);
        }
    }
}
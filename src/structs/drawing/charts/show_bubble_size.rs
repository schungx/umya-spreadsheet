// c:showBubbleSize
use super::super::super::BooleanValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ShowBubbleSize {
    val: BooleanValue,
}
impl ShowBubbleSize {
    pub fn get_val(&self)-> &bool {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:bool)-> &mut ShowBubbleSize {
        self.val.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        self.val.set_value_string(get_attribute(e, b"val").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:showBubbleSize
        write_start_tag(writer, "c:showBubbleSize", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}

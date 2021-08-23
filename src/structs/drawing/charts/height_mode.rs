// c:hMode
use super::LayoutModeValues;
use super::super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct HeightMode {
    val: EnumValue<LayoutModeValues>,
}
impl HeightMode {
    pub fn get_val(&self)-> &LayoutModeValues {
        &self.val.get_value()
    }

    pub fn set_val(&mut self, value:LayoutModeValues)-> &mut HeightMode {
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
        // c:hMode
        write_start_tag(writer, "c:hMode", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}

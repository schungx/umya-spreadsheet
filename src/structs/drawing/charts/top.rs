// c:y
use super::super::super::DoubleValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Top {
    val: DoubleValue,
}
impl Top {
    pub fn get_val(&self)-> &f64 {
        &self.val.get_value()
    }
    
    pub fn set_val(&mut self, value:f64)-> &mut Top {
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
        // c:y
        write_start_tag(writer, "c:y", vec![
            ("val", &self.val.get_value_string()),
        ], true);
    }
}
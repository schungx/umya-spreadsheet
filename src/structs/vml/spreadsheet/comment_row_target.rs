use crate::helper::coordinate::*;
use crate::reader::driver::*;
use crate::structs::UInt32Value;
use crate::traits::AdjustmentValue;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct CommentRowTarget {
    value: UInt32Value,
}

impl CommentRowTarget {
    #[inline]
    pub fn get_value(&self) -> &u32 {
        self.value.get_value()
    }

    #[inline]
    pub fn set_value(&mut self, value: u32) -> &mut Self {
        self.value.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn adjustment_insert_row(&mut self, num_rows: &u32) {
        let value = self.value.get_value() + num_rows;
        self.value.set_value(value);
    }

    #[inline]
    pub(crate) fn adjustment_remove_row(&mut self, num_row: &u32) {
        if self.value.get_value() > num_row {
            let value = self.value.get_value() - num_row;
            self.value.set_value(value);
        } else {
            self.value.set_value(1);
        }
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.value.set_value_string(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"x:Row" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "x:Row")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // x:Row
        write_start_tag(writer, "x:Row", vec![], false);
        write_text_node(writer, self.value.get_value_string());
        write_end_tag(writer, "x:Row");
    }
}
impl AdjustmentValue for CommentRowTarget {
    #[inline]
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_insert_coordinate(&(self.value.get_value() + &1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.value.set_value(
            adjustment_remove_coordinate(&(self.value.get_value() + &1), root_num, offset_num) - 1,
        );
    }

    #[inline]
    fn is_remove_value(&self, root_num: &u32, offset_num: &u32) -> bool {
        is_remove_coordinate(&(self.value.get_value() + 1), root_num, offset_num)
    }
}

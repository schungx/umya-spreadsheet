// colItems
use structs::RowItem;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ColumnItems {
    list: Vec<RowItem>,
}
impl ColumnItems {
    pub fn get_list(&self) -> &Vec<RowItem> {
        &self.list
    }

    pub fn get_list_mut(&mut self) -> &mut Vec<RowItem> {
        &mut self.list
    }

    pub fn add_list_mut(&mut self, value: RowItem) -> &mut Self {
        self.list.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"i" {
                    let mut obj = RowItem::default();
                    obj.set_attributes(reader, e, true);
                    self.add_list_mut(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"i" {
                    let mut obj = RowItem::default();
                    obj.set_attributes(reader, e, false);
                    self.add_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colItems" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colItems")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // colItems
        write_start_tag(writer, "colItems", vec![
            ("count", self.list.len().to_string().as_str())
        ], false);

        // i
        for i in &self.list {
            i.write_to(writer);
        }

        write_end_tag(writer, "colItems");
    }
}

use std::collections::HashMap;
use std::io::Cursor;
use std::io::Write;
use quick_xml::{
    events::BytesText,
    events::Event as XmlEvent,
    reader::Reader as XmlReader,
    writer::Writer as XmlWriter,
    name::QName,
    Result as XmlResult,
};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

#[derive(Debug)]
pub enum GmdValue {
    Bool(bool),
    Str(String),
    Int(i32),
    Real(f32),
    Dict(HashMap<String, GmdValue>),
}

pub fn gmd_to_bytes(value: GmdValue) -> Option<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    cursor.write(br#"<?xml version="1.0"?>"#).ok()?;
    let mut writer = XmlWriter::new(cursor);
    
    writer.create_element("plist")
        .with_attribute(("version", "1.0"))
        .with_attribute(("gjver", "2.0"))
        .write_inner_content(|writer| {
            write_value(writer, &value)
        }).ok()?;
    
    Some(writer.into_inner().into_inner())
}

fn write_value<W: Write>(writer: &mut XmlWriter<W>, value: &GmdValue) -> XmlResult<()> {
    match value {
        GmdValue::Bool(true) => { writer.create_element("t ").write_empty()?; },
        GmdValue::Bool(false) => { writer.create_element("f ").write_empty()?; },
        GmdValue::Str(v) => {
            writer.create_element("s")
                .write_text_content(BytesText::new(v))?;
        },
        GmdValue::Int(v) => {
            writer.create_element("i")
                .write_text_content(BytesText::new(&v.to_string()))?;
        },
        GmdValue::Real(v) => {
            writer.create_element("r")
                .write_text_content(BytesText::new(&v.to_string()))?;
        },
        GmdValue::Dict(v) => { write_dict(writer, v)?; },
    }
    
    Ok(())
}

fn write_dict<W: Write>(writer: &mut XmlWriter<W>, dict: &HashMap<String, GmdValue>) -> XmlResult<()> {
    writer.create_element("dict")
        .write_inner_content(|writer| {
            for (k, v) in dict.iter() {
                writer.create_element("k")
                    .write_text_content(BytesText::new(k))?;
                write_value(writer, v)?;
            }
            Ok(())
        }).map(|_| ())
}

pub fn gmd_from_bytes(bytes: &[u8]) -> Option<GmdValue> {
    let mut reader = XmlReader::from_reader(bytes);
    
    gmd_from_xml_reader(reader)
}

fn gmd_from_xml_reader(mut reader: XmlReader<&[u8]>) -> Option<GmdValue> {
    let event = next_gmd_event(&mut reader)?;
    // next element should be <plist> or any data tag
    match event {
        XmlEvent::Start(event) => {
            let tag = event.name();
            match tag.0 {
                b"plist" => gmd_from_xml_reader(reader),
                _ => { parse_value(&mut reader, tag) },
            }
        },
        XmlEvent::Empty(event) => {
            parse_value(&mut reader, event.name())
        }
        _ => None,
    }
}

fn parse_value(reader: &mut XmlReader<&[u8]>, tag: QName) -> Option<GmdValue> {
    match tag.0 {
        b"dictionary" | b"dict" | b"d" => parse_dict(reader),
        b"string" | b"s" => reader.read_text(tag).ok()
            .map(|x| GmdValue::Str(x.into_owned())),
        b"integer" | b"i" => reader.read_text(tag).ok()?
            .parse().ok()
            .map(|x| GmdValue::Int(x)),
        b"real" | b"r" => reader.read_text(tag).ok()?
            .parse().ok()
            .map(|x| GmdValue::Real(x)),
        b"true" | b"t" => Some(GmdValue::Bool(true)),
        b"false" | b"f" => Some(GmdValue::Bool(false)),
        _ => None,
    }
}

fn parse_dict(reader: &mut XmlReader<&[u8]>) -> Option<GmdValue> {
    let mut dict = HashMap::<String, GmdValue>::new();
    // looping through key/value pairs until closing tag
    loop {
        // either closing tag or <k>
        let event = next_gmd_event(reader)?;
        let key = match event {
            XmlEvent::Start(event) => {
                let tag = event.name();
                match tag.0 {
                    b"k" | b"key" => {
                        reader.read_text(tag).ok()?.into_owned()
                    },
                    _ => { return None; },
                }
            },
            XmlEvent::End(event) => { break Some(GmdValue::Dict(dict)); },
            _ => { return None; },
        };
        // can be any value
        let event = next_gmd_event(reader)?;
        let value = match event {
            XmlEvent::Start(event) | XmlEvent::Empty(event) => {
                parse_value(reader, event.name())?
            },
            _ => { return None; },
        };
        dict.insert(key, value);
    }
}

// ignores events that don't matter, returns None on events that shouldn't be there
fn next_gmd_event<'a>(reader: &mut XmlReader<&'a [u8]>) -> Option<XmlEvent<'a>> {
    loop {
        let event = reader.read_event().ok()?;
        break match event {
            XmlEvent::Start(_) => Some(event),
            XmlEvent::End(_) => Some(event),
            XmlEvent::Empty(_) => Some(event),
            XmlEvent::Text(_) => Some(event),
            XmlEvent::Comment(_) => { continue; },
            XmlEvent::CData(_) => None,
            XmlEvent::Decl(_) => { continue; },
            XmlEvent::PI(_) => None,
            XmlEvent::DocType(_) => { continue; },
            XmlEvent::Eof => Some(event),
        };
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn read_gmd() {
        
        let gmd = br#"<?xml version="1.0"?><plist version="1.0" gjver="2.0"><dict><k>awawa</k><i>68</i><k>auaua</k><t /><k>avava</k><f /><k>ayaya</k><s>:]</s></dict></plist>"#;
        
        let read = gmd_from_bytes(&gmd[..]).unwrap();
        println!("{:?}", read);
        
        let write = gmd_to_bytes(read).unwrap();
        println!("{}", std::str::from_utf8(&write).unwrap());
        
    }
}

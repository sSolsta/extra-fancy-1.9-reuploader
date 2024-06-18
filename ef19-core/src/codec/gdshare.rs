use crate::codec::{escaped_string, escaped_string_quotes};
use std::collections::HashMap;
use std::fmt;
use std::io::Cursor;
use std::io::Write;
use std::io::Error as IoError;
use quick_xml::{
    events::BytesText,
    events::Event as XmlEvent,
    reader::Reader as XmlReader,
    writer::Writer as XmlWriter,
    name::QName,
    Result as XmlResult,
    Error as XmlError,
};

#[derive(Debug)]
pub enum GmdValue {
    Bool(bool),
    Str(String),
    Int(i32),
    Real(f32),
    Dict(HashMap<String, GmdValue>),
}

#[derive(Debug)]
pub enum FormatError {
    Start(String),
    End(String),
    Empty(String),
    Text(String),
    CData,
    PI,
    Eof,
}
impl fmt::Display for FormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FormatError::Start(s) => write!(f, "unexpected or unrecognised element <{s}>"),
            FormatError::End(s) => write!(f, "unexpected closing tag </{s}>"),
            FormatError::Empty(s) => write!(f, "unexpected or unrecognised element <{s}/>"),
            FormatError::Text(s) => write!(f, "text found where it shouldn't be: \"{s}\""),
            FormatError::CData => write!(f, "CDATA element found"),
            FormatError::PI => write!(f, "<?...?> element found"),
            FormatError::Eof => write!(f, "end of file reached earlier than expected"),
        }
    }
}
impl std::error::Error for FormatError {}
impl From<XmlEvent<'_>> for FormatError {
    fn from(event: XmlEvent<'_>) -> Self {
        match event {
            XmlEvent::Start(e) => FormatError::Start(escaped_string(e.name().0)),
            XmlEvent::End(e) => FormatError::End(escaped_string(e.name().0)),
            XmlEvent::Empty(e) => FormatError::Empty(escaped_string(e.name().0)),
            XmlEvent::Text(e) => {
                if (&*e).len() < 20 { FormatError::Text(escaped_string_quotes(&*e)) }
                else {
                    let mut string = escaped_string_quotes(&e[..17]);
                    string.push_str("...");
                    FormatError::Text(string)
                }
            },
            XmlEvent::CData(e) => FormatError::CData,
            XmlEvent::PI(e) => FormatError::PI,
            XmlEvent::Eof => FormatError::Eof,
            _ => { panic!("{event:?} not implemented for unexpected event error"); }
        }
    }
}
impl From<QName<'_>> for FormatError {
    fn from(name: QName<'_>) -> Self {
        // assume start tag, it doesn't matter too much
        FormatError::Start(escaped_string(name.0))
    }
}

#[derive(Debug)]
pub enum GmdError {
    Xml(XmlError),
    Io(IoError),
    Format(FormatError),
    InvalidInt(String),
    InvalidReal(String),
}
impl fmt::Display for GmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GmdError::Xml(e) => write!(f, "xml decode error: {e}"),
            GmdError::Io(e) => write!(f, "I/O error: {e}"),
            GmdError::Format(e) => write!(f, "format error: {e}"),
            GmdError::InvalidInt(e) => write!(f, "invalid value for integer: {e}"),
            GmdError::InvalidReal(e) => write!(f, "invalid value for real: {e}"),
        }
    }
}
impl std::error::Error for GmdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GmdError::Xml(e) => Some(e),
            GmdError::Io(e) => Some(e),
            GmdError::Format(e) => Some(e),
            GmdError::InvalidInt(_) => None,
            GmdError::InvalidReal(_) => None,
        }
    }
}
impl From<XmlError> for GmdError {
    fn from(e: XmlError) -> Self {
        GmdError::Xml(e)
    }
}
impl From<IoError> for GmdError {
    fn from(e: IoError) -> Self {
        GmdError::Io(e)
    }
}
impl From<FormatError> for GmdError {
    fn from(e: FormatError) -> Self {
        GmdError::Format(e)
    }
}
pub type GmdResult<T> = std::result::Result<T, GmdError>;

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

pub fn gmd_from_bytes(bytes: &[u8]) -> GmdResult<GmdValue> {
    let mut reader = XmlReader::from_reader(bytes);
    
    gmd_from_xml_reader(reader)
}

fn gmd_from_xml_reader(mut reader: XmlReader<&[u8]>) -> GmdResult<GmdValue> {
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
        _ => Err(GmdError::from(FormatError::from(event))),
    }
}

fn parse_value(reader: &mut XmlReader<&[u8]>, tag: QName) -> GmdResult<GmdValue> {
    match tag.0 {
        b"dictionary" | b"dict" | b"d" => parse_dict(reader),
        b"string" | b"s" => Ok(GmdValue::Str(reader.read_text(tag)?.into_owned())),
        b"integer" | b"i" => {
            let text = reader.read_text(tag)?;
            match text.parse() {
                Ok(v) => Ok(GmdValue::Int(v)),
                Err(..) => Err(GmdError::InvalidInt(text.into_owned())),
            }
        },
        b"real" | b"r" => {
            let text = reader.read_text(tag)?;
            match text.parse() {
                Ok(v) => Ok(GmdValue::Real(v)),
                Err(..) => Err(GmdError::InvalidReal(text.into_owned())),
            }
        },
        b"true" | b"t" => Ok(GmdValue::Bool(true)),
        b"false" | b"f" => Ok(GmdValue::Bool(false)),
        _ => Err(GmdError::from(FormatError::from(tag))),
    }
}

fn parse_dict(reader: &mut XmlReader<&[u8]>) -> GmdResult<GmdValue> {
    let mut dict = HashMap::<String, GmdValue>::new();
    // looping through key/value pairs until closing tag
    loop {
        // either closing tag or <k>
        let event = next_gmd_event(reader)?;
        let key = match event {
            XmlEvent::Start(ref e) => {
                let tag = e.name();
                match tag.0 {
                    b"k" | b"key" => {
                        reader.read_text(tag)?.into_owned()
                    },
                    _ => { return Err(GmdError::from(FormatError::from(event))); },
                }
            },
            XmlEvent::End(_) => { break Ok(GmdValue::Dict(dict)); },
            _ => { return Err(GmdError::from(FormatError::from(event))); },
        };
        // can be any value
        let event = next_gmd_event(reader)?;
        let value = match event {
            XmlEvent::Start(e) | XmlEvent::Empty(e) => {
                parse_value(reader, e.name())?
            },
            _ => { return Err(GmdError::from(FormatError::from(event))); },
        };
        dict.insert(key, value);
    }
}

// ignores events that don't matter, returns None on events that shouldn't be there
fn next_gmd_event<'a>(reader: &mut XmlReader<&'a [u8]>) -> GmdResult<XmlEvent<'a>> {
    loop {
        let event = reader.read_event()?;
        println!("{:?}", event.clone().into_owned());
        break match event {
            XmlEvent::Start(_) => Ok(event),
            XmlEvent::End(_) => Ok(event),
            XmlEvent::Empty(_) => Ok(event),
            XmlEvent::Text(_) => Ok(event),
            XmlEvent::Comment(_) => { continue; },
            XmlEvent::CData(_) => Err(GmdError::from(FormatError::from(event))),
            XmlEvent::Decl(_) => { continue; },
            XmlEvent::PI(_) => Err(GmdError::from(FormatError::from(event))),
            XmlEvent::DocType(_) => { continue; },
            XmlEvent::Eof => Ok(event),
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

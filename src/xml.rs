use std::collections::HashMap;
use std::io::Cursor;

use once_cell::sync::Lazy;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use regex::Regex;

use crate::MinifyError;

struct XmlMinifier {
    ignore_tags: Vec<Box<[u8]>>,
    minified_tags: HashMap<Box<[u8]>, u8>,
}

impl XmlMinifier {
    pub fn new() -> Self {
        Self {
            ignore_tags: vec![
                b"baseAddress".to_vec().into_boxed_slice(),
                b"addressBlock".to_vec().into_boxed_slice(),
                b"addressOffset".to_vec().into_boxed_slice(),
                b"offset".to_vec().into_boxed_slice(),
                b"size".to_vec().into_boxed_slice(),
                b"usage".to_vec().into_boxed_slice(),
                b"access".to_vec().into_boxed_slice(),
                b"resetValue".to_vec().into_boxed_slice(),
                b"bitOffset".to_vec().into_boxed_slice(),
                b"bitWidth".to_vec().into_boxed_slice(),
            ],
            minified_tags: HashMap::new(),
        }
    }

    /// gets the minified tag for the given tag, or creates a new one if it does not exist
    fn get_minified_tag_idx(&mut self, tag: impl AsRef<[u8]>) -> u8 {
        let tag = tag.as_ref();
        match self.minified_tags.get(tag) {
            Some(&idx) => idx,
            None => {
                let tag = tag.to_vec().into_boxed_slice();
                let idx = self.minified_tags.len() as u8;
                self.minified_tags.insert(tag, idx);
                idx
            }
        }
    }

    fn get_minified_tag(&mut self, tag: impl AsRef<[u8]>) -> char {
        let idx = self.get_minified_tag_idx(tag);
        (idx + b'a') as char
    }

    /// returns a string [minified_tag] -> [original_tag]
    pub fn tags(&self) -> String {
        let mut tags = self.minified_tags.iter().collect::<Vec<_>>();
        tags.sort_by_key(|(_, &idx)| idx);
        tags.iter()
            .map(|(tag, &idx)| format!("{}={}", (idx + b'a') as char, String::from_utf8_lossy(tag)))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn minify(&mut self, xml: &str) -> Result<String, MinifyError> {
        let mut reader = quick_xml::Reader::from_str(xml);
        let mut writer = quick_xml::Writer::new(Cursor::new(Vec::new()));
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Eof => break,
                Event::Start(e) => {
                    let name = e.name().into_inner().to_vec().into_boxed_slice();
                    if self.ignore_tags.contains(&name) {
                        reader.read_to_end(e.name())?;
                        continue;
                    }
                    let minified = self.get_minified_tag(e.name());
                    let elem = BytesStart::new(minified.to_string());
                    writer.write_event(Event::Start(elem))?;
                }
                Event::End(e) => {
                    let minified = self.get_minified_tag(e.name());
                    let elem = BytesEnd::new(minified.to_string());
                    writer.write_event(Event::End(elem))?;
                }
                Event::Text(text) => {
                    static MULTIPLE_SPACE: Lazy<Regex> =
                        Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

                    let text = text.into_inner();
                    let text = String::from_utf8(text.to_vec())?;
                    let text = text.replace('\t', " ");
                    let text = text.replace('\r', "");
                    let text = text.replace('\n', " ");
                    let text = MULTIPLE_SPACE.replace_all(&text, "");
                    let text = text.trim().to_string();

                    let elem = BytesText::new(text.as_ref());
                    writer.write_event(Event::Text(elem))?;
                }

                // There are several other `Event`s we do not consider here
                _ => (),
            }
        }

        let result = writer.into_inner().into_inner();
        let result = String::from_utf8(result)?;

        Ok(result)
    }
}

pub fn minify(xml: &str) -> Result<String, MinifyError> {
    let mut minifier = XmlMinifier::new();
    let minified = minifier.minify(xml)?;
    let tags = minifier.tags();
    Ok(format!("{tags}\n{minified}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minify() {
        let xml = r#"
            <xml>
                <foo>
                    <bar>baz</bar>
                </foo>
            </xml>
        "#;

        let minified = minify(xml).unwrap();
        assert_eq!(minified, "a=xml,b=foo,c=bar\n<a><b><c>baz</c></b></a>")
    }
}

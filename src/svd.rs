use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::MinifyError;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Peripheral {
    name: String,
    description: String,

    group_name: String,
    base_address: String,
    // address_block: AddressBlock,
    interrupt: Interrupt,
    registers: Registers,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AddressBlock {
    offset: String,
    size: String,
    usage: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Interrupt {
    name: String,
    description: String,
    value: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Registers {
    register: Vec<Register>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Register {
    name: String,
    display_name: String,
    description: String,
    // address_offset: String,
    // size: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // access: Option<String>,
    reset_value: String,
    fields: Fields,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Fields {
    field: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Field {
    name: String,
    description: String,
    // bit_offset: u32,
    // bit_width: u32,
}

fn from_svd(svd: &str) -> Result<Peripheral, MinifyError> {
    let peripheral: Peripheral = serde_xml_rs::from_str(svd)?;
    Ok(peripheral)
}

fn norm(s: &mut String) {
    static MULTI_SPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s{2,}").unwrap());
    let desc = s.replace('\n', " ");
    let desc = MULTI_SPACE.replace_all(&desc, " ").to_string();
    *s = desc.trim().to_string();
}

pub fn minify(svd: &str) -> Result<String, MinifyError> {
    let mut peripheral = from_svd(svd)?;

    norm(&mut peripheral.description);

    for register in &mut peripheral.registers.register {
        norm(&mut register.description);
        for field in &mut register.fields.field {
            norm(&mut field.description);
        }
    }

    // let minified = ron::ser::to_string_pretty(&peripheral, ron::ser::PrettyConfig::default())?;
    let minified = serde_json::to_string(&peripheral)?; // ron::ser::to_string_pretty(&peripheral, ron::ser::PrettyConfig::default())?;
    Ok(minified)
}

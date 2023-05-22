use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NativeParam {
  #[serde(rename = "type")]
  pub ty:      String,
  pub name:    String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Native {
  pub name:        String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sch_comment: Option<String>,
  pub params:      Vec<NativeParam>,
  pub return_type: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
  #[serde(flatten)]
  pub natives: HashMap<u64, Native>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumValue {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub value:   Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StructField {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment:       Option<String>,
  pub type_name:     String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub array_size:    Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub default_value: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TypeDefinition {
  Enum {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    values:  IndexMap<String, EnumValue>
  },
  Struct {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<String>,
    fields:  IndexMap<String, StructField>
  },
  NativeType {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment:   Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias_for: Option<String>
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstDefinition {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment:   Option<String>,
  pub type_name: String,
  pub value:     String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRoot {
  pub types:     IndexMap<String, TypeDefinition>,
  pub constants: IndexMap<String, ConstDefinition>,
  pub natives:   IndexMap<String, Native>
}

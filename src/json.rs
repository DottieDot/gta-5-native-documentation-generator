use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::{crossmap::get_orig_native_hash, parser::model as sch};

#[derive(Serialize, Deserialize, Debug)]
pub struct NativeParam {
  #[serde(rename = "type")]
  type_:   String,
  name:    String,
  #[serde(skip_serializing_if = "Option::is_none")]
  default: Option<String>
}

impl From<sch::FunctionParameter> for NativeParam {
  fn from(value: sch::FunctionParameter) -> Self {
    Self {
      type_:   value.type_.into(),
      name:    value.name,
      default: value.default_value.map(|v| v.to_string())
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Native {
  name:        String,
  #[serde(skip_serializing_if = "Option::is_none")]
  sch_comment: Option<String>,
  params:      Vec<NativeParam>,
  return_type: String
}

impl From<sch::NativeDeclaration> for Native {
  fn from(value: sch::NativeDeclaration) -> Self {
    Self {
      name:        value.definition.name,
      sch_comment: if value.comments.is_empty() {
        None
      } else {
        Some(value.comments.join("\r\n"))
      },
      params:      value
        .definition
        .params
        .into_iter()
        .map(NativeParam::from)
        .collect(),
      return_type: value
        .definition
        .return_type
        .unwrap_or_else(|| "void".to_owned())
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Namespace {
  #[serde(flatten)]
  pub natives: HashMap<u64, Native>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnumValue {
  #[serde(skip_serializing_if = "Option::is_none")]
  comment: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  value:   Option<String>
}

impl From<sch::EnumField> for EnumValue {
  fn from(value: sch::EnumField) -> Self {
    Self {
      comment: value.comment,
      value:   value.value.map(|v| v.to_string())
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StructField {
  #[serde(skip_serializing_if = "Option::is_none")]
  comment:       Option<String>,
  type_name:     String,
  #[serde(skip_serializing_if = "Option::is_none")]
  array_size:    Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  default_value: Option<String>
}

impl From<sch::StructField> for StructField {
  fn from(value: sch::StructField) -> Self {
    Self {
      comment:       value.comment,
      type_name:     value.type_name,
      array_size:    value.array_size.map(|s| s.to_string()),
      default_value: value.default_value.map(|v| v.to_string())
    }
  }
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
  NativeType
}

impl From<sch::EnumDeclaration> for TypeDefinition {
  fn from(value: sch::EnumDeclaration) -> Self {
    TypeDefinition::Enum {
      comment: if value.comments.is_empty() {
        None
      } else {
        Some(value.comments.join("\r\n"))
      },
      values:  value
        .values
        .into_iter()
        .map(|v| (v.name.clone(), EnumValue::from(v)))
        .collect::<IndexMap<_, _>>()
    }
  }
}

impl From<sch::StructDeclaration> for TypeDefinition {
  fn from(value: sch::StructDeclaration) -> Self {
    Self::Struct {
      comment: if value.comments.is_empty() {
        None
      } else {
        Some(value.comments.join("\r\n"))
      },
      fields:  value
        .fields
        .into_iter()
        .map(|f| (f.name.clone(), StructField::from(f)))
        .collect::<IndexMap<_, _>>()
    }
  }
}

impl From<sch::NativeTypeDeclaration> for TypeDefinition {
  fn from(_: sch::NativeTypeDeclaration) -> Self {
    Self::NativeType
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstDefinition {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub comment:   Option<String>,
  pub type_name: String,
  pub value:     String
}

impl From<sch::ConstDeclaration> for ConstDefinition {
  fn from(value: sch::ConstDeclaration) -> Self {
    Self {
      comment:   value.comment,
      type_name: value.type_name,
      value:     value.value.to_string()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DocumentRoot {
  pub types:     IndexMap<String, TypeDefinition>,
  pub constants: IndexMap<String, ConstDefinition>,
  pub natives:   IndexMap<String, Native>
}

impl From<Vec<sch::Declaration>> for DocumentRoot {
  fn from(mut value: Vec<sch::Declaration>) -> Self {
    let mut types: IndexMap<String, TypeDefinition> = Default::default();
    let mut constants: IndexMap<String, ConstDefinition> = Default::default();
    let mut natives: IndexMap<String, Native> = Default::default();

    // rust-analyzer bug, it gets confused by `sch::Declaration::Const`
    #[allow(unreachable_code)]
    for decl in value.drain(0..) {
      match decl {
        sch::Declaration::Enum(enum_decl) => {
          types.insert(enum_decl.name.clone(), enum_decl.into());
        }
        sch::Declaration::Struct(struct_decl) => {
          types.insert(struct_decl.name.clone(), struct_decl.into());
        }
        sch::Declaration::Comment(_) => {}
        sch::Declaration::Using(_) => {}
        sch::Declaration::Function(_) => {}
        sch::Declaration::Native(native) => {
          if let Some(hash) = get_orig_native_hash(native.native_hash) {
            natives.insert(format!("0x{:016X}", hash), native.into());
          }
        }
        sch::Declaration::NativeType(type_decl) => {
          types.insert(type_decl.name.clone(), type_decl.into());
        }
        sch::Declaration::Const(const_decl) => {
          constants.insert(const_decl.name.clone(), const_decl.into());
        }
      }
    }

    Self {
      types,
      constants,
      natives
    }
  }
}

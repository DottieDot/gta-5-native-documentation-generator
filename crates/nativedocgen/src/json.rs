use indexmap::IndexMap;
use nativedocgen_model::{
  ConstDefinition, DocumentRoot, EnumValue, Native, NativeParam, StructField, TypeDefinition
};

use crate::{crossmap::get_orig_native_hash, parser::model as sch};

impl From<sch::FunctionParameter> for NativeParam {
  fn from(value: sch::FunctionParameter) -> Self {
    Self {
      ty:      value.type_.into(),
      name:    value.name,
      default: value.default_value.map(|v| v.to_string())
    }
  }
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

impl From<sch::EnumField> for EnumValue {
  fn from(value: sch::EnumField) -> Self {
    Self {
      comment: value.comment,
      value:   value.value.map(|v| v.to_string())
    }
  }
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
  fn from(value: sch::NativeTypeDeclaration) -> Self {
    Self::NativeType {
      comment:   value.comment,
      alias_for: value.alias_for
    }
  }
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

pub fn to_document_root(mut value: Vec<sch::Declaration>) -> DocumentRoot {
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

  DocumentRoot {
    types,
    constants,
    natives
  }
}

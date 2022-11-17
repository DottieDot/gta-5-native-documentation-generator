#[derive(Debug)]
pub enum Literal {
  Int(i32),
  Float(f32),
  Bool(bool),
  Hash(String)
}

#[derive(Debug)]
pub struct EnumField {
  pub comment: Option<String>,
  pub name:    String,
  pub value:   Option<LiteralOrConst>
}

#[derive(Debug)]
pub struct EnumDeclaration {
  pub comments: Vec<String>,
  pub name:     String,
  pub values:   Vec<EnumField>
}

#[derive(Debug)]
pub struct StructField {
  pub name:          String,
  pub type_name:     String,
  pub array_size:    Option<LiteralOrConst>,
  pub default_value: Option<LiteralOrConst>,
  pub comment:       Option<String>
}

#[derive(Debug)]
pub struct StructDeclaration {
  pub comments: Vec<String>,
  pub name:     String,
  pub fields:   Vec<StructField>
}

#[derive(Debug)]
pub enum LiteralOrConst {
  Literal(Literal),
  ConstValue(String)
}

#[derive(Debug)]
pub struct FunctionParameterType {
  pub base_type: String,
  pub is_ref:    bool,
  pub is_array:  bool
}

#[derive(Debug)]
pub struct FunctionParameter {
  pub name:          String,
  pub type_:         FunctionParameterType,
  pub default_value: Option<LiteralOrConst>
}

#[derive(Debug)]
pub struct FunctionDefinition {
  pub name:        String,
  pub return_type: Option<String>,
  pub params:      Vec<FunctionParameter>
}

#[derive(Debug)]
pub struct FunctionDeclaration {
  pub comments:   Vec<String>,
  pub definition: FunctionDefinition,
  pub body:       String
}

#[derive(Debug)]
pub struct NativeDeclaration {
  pub comments:    Vec<String>,
  pub definition:  FunctionDefinition,
  pub native_hash: u64
}

#[derive(Debug)]
pub struct ConstDeclaration {
  pub comment:   Option<String>,
  pub type_name: String,
  pub name:      String,
  pub value:     LiteralOrConst
}

#[derive(Debug)]
pub struct NativeTypeDeclaration {
  pub name: String
}

#[derive(Debug)]
pub enum Declaration {
  Enum(EnumDeclaration),
  Struct(StructDeclaration),
  Comment(String),
  Using(String),
  Function(FunctionDeclaration),
  Native(NativeDeclaration),
  NativeType(NativeTypeDeclaration),
  Const(ConstDeclaration)
}

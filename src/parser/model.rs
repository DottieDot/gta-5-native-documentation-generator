use std::fmt::Display;

#[derive(Debug)]
pub enum Literal {
  Int(i32),
  Float(f32),
  Bool(bool),
  Hash(String)
}

impl Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Literal::Int(i) => write!(f, "{i}"),
      Literal::Float(float) => write!(f, "{float}"),
      Literal::Bool(b) => write!(f, "{b}"),
      Literal::Hash(h) => write!(f, "HASH(\"{h}\")")
    }
  }
}

#[derive(Debug)]
pub struct EnumField {
  pub comment: Option<String>,
  pub name:    String,
  pub value:   Option<Expression>
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
  pub array_size:    Option<Expression>,
  pub default_value: Option<Expression>,
  pub comment:       Option<String>
}

#[derive(Debug)]
pub struct StructDeclaration {
  pub comments: Vec<String>,
  pub name:     String,
  pub fields:   Vec<StructField>
}

#[derive(Debug)]
pub enum Expression {
  Literal(Literal),
  Identifier(String),
  Add(Box<Expression>, Box<Expression>),
  Subtract(Box<Expression>, Box<Expression>),
  Multiply(Box<Expression>, Box<Expression>),
  Divide(Box<Expression>, Box<Expression>),
  BitOr(Box<Expression>, Box<Expression>),
  Parentheses(Box<Expression>)
}

impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Expression::Literal(l) => write!(f, "{l}"),
      Expression::Identifier(i) => write!(f, "{i}"),
      Expression::Add(l, r) => write!(f, "{l} + {r}"),
      Expression::Subtract(l, r) => write!(f, "{l} - {r}"),
      Expression::Multiply(l, r) => write!(f, "{l} * {r}"),
      Expression::Divide(l, r) => write!(f, "{l} / {r}"),
      Expression::BitOr(l, r) => write!(f, "{l} | {r}"),
      Expression::Parentheses(e) => write!(f, "({e})")
    }
  }
}

// impl From<Expression> for String {
//   fn from(value: Expression) -> Self {
//     match value {
//       Expression::Literal(l) => l.into(),
//       Expression::Identifier(i) => i,
//       Expression::Add(l, r) => format!("{} + {}", *l, *r),
//       Expression::Subtract(l, r) => format!("{} - {}", *l, *r),
//       Expression::Multiply(l, r) => format!("{} * {}", *l, *r),
//       Expression::Divide(l, r) => format!("{} / {}", *l, *r),
//       Expression::BitOr(l, r) => format!("{} | {}", *l, *r),
//       Expression::Parentheses(e) => format!("({})", *e)
//     }
//   }
// }

#[derive(Debug)]
pub struct FunctionParameterType {
  pub base_type: String,
  pub is_ref:    bool,
  pub is_array:  bool
}

impl From<FunctionParameterType> for String {
  fn from(
    FunctionParameterType {
      base_type,
      is_ref,
      is_array
    }: FunctionParameterType
  ) -> Self {
    let ref_str = is_ref.then_some("&").unwrap_or("");
    let array_str = is_array.then_some("[]").unwrap_or("");

    format!("{base_type}{array_str}{ref_str}")
  }
}

#[derive(Debug)]
pub struct FunctionParameter {
  pub name:          String,
  pub type_:         FunctionParameterType,
  pub default_value: Option<Expression>
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
  pub value:     Expression
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

pub mod model;

use model::{
  ConstDeclaration, Declaration, EnumDeclaration, EnumField, FunctionDeclaration,
  FunctionDefinition, FunctionParameter, FunctionParameterType, Literal, LiteralOrConst,
  NativeDeclaration, NativeTypeDeclaration, StructDeclaration, StructField
};

peg::parser! {
  pub grammar sch_parser() for str {
    pub rule sch() -> Vec<Declaration>
      = eol()? declarations:(declaration() ** eol()) eol()? eof() {
        declarations
      }

    rule declaration() -> Declaration
      = using_declaration()
      / native_declaration()
      / native_type_declaration()
      / function_declaration()
      / struct_declaration()
      / enum_declaration()
      / const_declaration()
      / comment_declaration()

    rule native_type_declaration() -> Declaration
      = native_type:native_type() {
        Declaration::NativeType(native_type)
      }

    rule native_type() -> NativeTypeDeclaration
      = "NATIVE" _ ident:identifier() {
        NativeTypeDeclaration { name: ident }
      }


    rule function_declaration() -> Declaration
      = func:function() {
        Declaration::Function(func)
      }

    rule function() -> FunctionDeclaration
      = comments:comments() eol()? definition:function_definition() body:$((!("ENDFUNC" / "ENDPROC") v:[_] {v})+) ("ENDFUNC" / "ENDPROC") {
        FunctionDeclaration { comments, definition, body: body.to_owned() }
      }

    rule comment_declaration() -> Declaration
      = text:comment() {
        Declaration::Comment(text.to_owned())
      }

    rule comments() -> Vec<String>
      = comments:(comment() ** eol()) {
        comments
      }

    rule comment() -> String
      = "//" "/"? text:text_until_eol() {
        text
      }

    rule using_declaration() -> Declaration
      = "USING" _ s:string_literal() {
        Declaration::Using(s)
      }

    rule const_declaration() -> Declaration
      = const_decl:const() {
        Declaration::Const(const_decl)
      }

    rule const() -> ConstDeclaration
      = "CONST_" type_name:identifier() _ name:identifier() _ value:literal_or_const() _ comment:comment()? {
        ConstDeclaration { comment, type_name, name, value }
      }

    rule struct_declaration() -> Declaration
      = struct_decl:struct() {
        Declaration::Struct(struct_decl)
      }

    rule struct() -> StructDeclaration
      = comments:comments() eol()? "STRUCT" _ name:identifier() eol() fields:(struct_field() ** eol()) eol() "ENDSTRUCT" {
        StructDeclaration { name, fields, comments }
      }

    rule struct_field() -> StructField
      = _ comment_a:struct_field_comment()? _ type_name:identifier() _ field_name:identifier() _ size:struct_field_array_size()? _ default:struct_field_default()? _ comment_b:comment()? {
        StructField { name: field_name, type_name, array_size: size, default_value: default, comment: comment_b.or(comment_a) }
      }

    rule struct_field_comment() -> String
      = comment:comments() eol() {
        comment.join("\r\n")
      }

    rule struct_field_array_size() -> LiteralOrConst
      = "[" size:literal_or_const() "]" {
        size
      }

    rule struct_field_default() -> LiteralOrConst
      = "=" _ default:literal_or_const() {
        default
      }

    rule enum_declaration() -> Declaration
      = enum_decl:(enum() / hash_enum() / strict_enum()) {
        Declaration::Enum(enum_decl)
      }

    rule enum() -> EnumDeclaration
      = comments:comments() eol()? "ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration { comments, ..content }
      }

    rule strict_enum() -> EnumDeclaration
      = comments:comments() eol()? "STRICT_ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration { comments, ..content }
      }

    rule hash_enum() -> EnumDeclaration
      = comments:comments() eol()? "HASH_ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration {
          comments,
          values: content.values
            .into_iter()
            .map(|f| EnumField {
              value: Some(LiteralOrConst::Literal(Literal::Hash(f.name.clone()))),
              ..f
            }).collect(),
          ..content
        }
      }

    rule enum_content() -> EnumDeclaration
      =  _ name:identifier() _ (comment() / eol())* _ fields:enum_items() _ (comment() / eol())* {
        EnumDeclaration { name, values: fields, comments: vec![] }
      }

    rule enum_items() -> Vec<EnumField>
      = fields:(enum_field() ** (_ (eol() / comment())* _ "," (eol() / comment())* _)) {
        fields
      }

    rule enum_field() -> EnumField
      = enum_field_with_value()
      / enum_field_without_value()

    rule enum_field_without_value() -> EnumField
      = comment:(i:comment() eol() {i})? name:identifier() {
        EnumField { comment, name, value: None }
      }

    rule enum_field_with_value() -> EnumField
      = comment:(i:comment() eol() {i})? name:identifier() _ "=" _ value:literal_or_const() {
        EnumField { comment, name, value: Some(value) }
      }

    rule native_declaration() -> Declaration
      = native:native() {
        Declaration::Native(native)
      }

    rule native() -> NativeDeclaration
      = comments:comments() eol()? "NATIVE" _ function:function_definition() _ "=" _ hash:native_hash() {
        NativeDeclaration { comments: comments, definition: function, native_hash: hash }
      }

    rule function_definition() -> FunctionDefinition
      = function_definition_void()
      / function_definition_with_return_type()

    rule function_definition_void() -> FunctionDefinition
      = "DEBUGONLY"? _ "PROC" _ name:identifier() _ params:function_params() {
        FunctionDefinition { name, return_type: None, params }
      }

    rule function_definition_with_return_type() -> FunctionDefinition
      = "DEBUGONLY"? _ "FUNC" _ return_type:identifier() _ name:identifier() _ params:function_params() {
        FunctionDefinition { name, return_type: Some(return_type), params}
      }

    rule function_params() -> Vec<FunctionParameter>
      = "(" _ params:(function_param() ** (eol()? _ "," _ eol()?)) _ ")" {
        params
      }

    rule function_param() -> FunctionParameter
      = type_name:function_param_type() _ is_ref:("&")? _ name:identifier() _ is_array:("[]")? _ default_value:default_function_param_value()? {
        FunctionParameter {
          name: name,
          type_: FunctionParameterType {
            is_ref: is_ref.is_some(),
            base_type: type_name,
            is_array: is_array.is_some()
          },
          default_value
        }
      }

      rule default_function_param_value() -> LiteralOrConst
        = "=" _ v:literal_or_const() {
          v
        }

      rule literal_or_const() -> LiteralOrConst
        = literal_or_const_literal()
        / literal_or_const_const()

      rule literal_or_const_const() -> LiteralOrConst
        = c:identifier() {
          LiteralOrConst::ConstValue(c)
        }

    rule literal_or_const_literal() -> LiteralOrConst
      = v:literal() {
        LiteralOrConst::Literal(v)
      }

    rule function_param_type() -> String
      = type_name:$(identifier()) {
        type_name.to_owned()
      }

    rule literal() -> Literal
      = hash_literal()
      / float_literal()
      / int_literal()
      / bool_literal()

    rule hash_literal() -> Literal
      = "HASH(" _ string:string_literal() _ ")" {
        Literal::Hash(string)
      }

    rule float_literal() -> Literal
      = f:float() {
        Literal::Float(f)
      }

    rule float() -> f32
      = quiet!{ n:$(['-']?['0'..='9']+(['.']['0'..='9']+)?) {
        ? n.parse().or(Err("i32"))
      }}
      / expected!("float")

    rule bool_literal() -> Literal
      = b:bool() {
        Literal::Bool(b)
      }

    rule bool() -> bool
      = bool_true()
      / bool_false()

    rule bool_true() -> bool
      = ("TRUE" / "true") {
        true
      }

    rule bool_false() -> bool
      = ("FALSE" / "false") {
        false
      }

    rule int_literal() -> Literal
      = i:integer() {
        Literal::Int(i)
      }

    rule integer() -> i32
      = quiet!{ n:$(['-']?['0'..='9']+) {
        ? n.parse().or(Err("i32"))
      }}
      / expected!("integer")

    rule native_hash() -> u64
      = quiet!{ "\"0x" n:$(['0'..='9' | 'a'..='f' | 'A'..='F']+) "\"" {
        ? u64::from_str_radix(n, 16).or(Err("u64"))
      } }
      / expected!("native hash")

    rule identifier() -> String
      =
      ! "ENDSTRUCT"
      ! "ENDENUM"
        ident:any_identifier() {
        ident
      }

    rule text_until_eol() -> String
      = text:$([^'\r' | '\n']*) {
        text.to_owned()
      }

    rule any_identifier() -> String
      = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '!' | '.' | '+']*) {
        n.to_owned()
      }}
      / expected!("identifier")

    rule string_literal() -> String
      = quiet!{"\"" n:$([^'"']*) "\"" {
        n.to_owned()
      }}
      / expected!("string literal")

    rule _() = quiet!{[' ' | '\t']*}

    rule eol()
      = quiet!{['\r' | '\n' | ' ' | '\t']+}
      / expected!("end of line")

    rule eof()
      = ![_]
  }
}

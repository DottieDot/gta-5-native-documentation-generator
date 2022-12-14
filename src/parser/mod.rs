pub mod model;

use model::{
  ConstDeclaration, Declaration, EnumDeclaration, EnumField, Expression, FunctionDeclaration,
  FunctionDefinition, FunctionParameter, FunctionParameterType, Literal, NativeDeclaration,
  NativeTypeDeclaration, StructDeclaration, StructField
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
      = "NATIVE" _ ident:identifier() _ alias:type_alias()? _ comment:comment()? {
        NativeTypeDeclaration { name: ident, alias_for: alias, comment }
      }

    rule type_alias() -> String
      = ":" _ ident:identifier() {
        ident
      }


    rule function_declaration() -> Declaration
      = func:function() {
        Declaration::Function(func)
      }

    rule function() -> FunctionDeclaration
      = comments:comments() one_eol()? definition:function_definition() body:$((!("ENDFUNC" / "ENDPROC") v:[_] {v})+) ("ENDFUNC" / "ENDPROC") {
        FunctionDeclaration { comments, definition, body: body.to_owned() }
      }

    rule comment_declaration() -> Declaration
      = text:comment() {
        Declaration::Comment(text.to_owned())
      }

    rule comments() -> Vec<String>
      = comments:(comment() ** (one_eol() _)) {
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
      = "CONST_" type_name:identifier() _ name:identifier() _ value:expression() _ comment:comment()? {
        ConstDeclaration { comment, type_name, name, value }
      }

    rule struct_declaration() -> Declaration
      = struct_decl:struct() {
        Declaration::Struct(struct_decl)
      }

    rule struct() -> StructDeclaration
      = comments:comments() one_eol()? _ "STRUCT" _ name:identifier() eol() fields:(struct_field() ** eol()) eol() "ENDSTRUCT" {
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

    rule struct_field_array_size() -> Expression
      = "[" size:expression() "]" {
        size
      }

    rule struct_field_default() -> Expression
      = "=" _ default:expression() {
        default
      }

    rule enum_declaration() -> Declaration
      = enum_decl:(enum() / hash_enum() / strict_enum()) {
        Declaration::Enum(enum_decl)
      }

    rule enum() -> EnumDeclaration
      = comments:comments() one_eol()? _ "ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration { comments, ..content }
      }

    rule strict_enum() -> EnumDeclaration
      = comments:comments() one_eol()? _ "STRICT_ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration { comments, ..content }
      }

    rule hash_enum() -> EnumDeclaration
      = comments:comments() one_eol()? _ "HASH_ENUM" content:enum_content() "ENDENUM" {
        EnumDeclaration {
          comments,
          values: content.values
            .into_iter()
            .map(|f| EnumField {
              value: Some(Expression::Literal(Literal::Hash(f.name.clone()))),
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
      = fields:(enum_field() ** (_ (comment() / eol())* _ "," (comment() / eol())* _)) {
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
      = comment:(i:comment() eol() {i})? name:identifier() _ "=" _ value:expression() {
        EnumField { comment, name, value: Some(value) }
      }

    rule native_declaration() -> Declaration
      = native:native() {
        Declaration::Native(native)
      }

    rule native() -> NativeDeclaration
      = comments:comments() one_eol()? "NATIVE" _ function:function_definition() _ "=" _ hash:native_hash() {
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
      = "(" _ params:((function_param() / function_param_varargs()) ** (eol()? _ "," _ eol()?)) _ ")" {
        params
      }

    rule function_param_varargs() -> FunctionParameter
      = varargs:varargs() {
        FunctionParameter { name: varargs.clone(), type_: FunctionParameterType { base_type: varargs, is_ref: false, is_array: false }, default_value: None }
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

    rule default_function_param_value() -> Expression
      = "=" _ v:expression() {
        v
      }

    rule function_param_type() -> String
      = type_name:$(identifier()) {
        type_name.to_owned()
      }

    rule expression() -> Expression
      = precedence! {
        x:(@) _ "+" _ y:@ { Expression::Add(x.into(), y.into()) }
        x:(@) _ "-" _ y:@ { Expression::Subtract(x.into(), y.into()) }
        --
        x:(@) _ "*" _ y:@ { Expression::Multiply(x.into(), y.into()) }
        x:(@) _ "/" _ y:@ { Expression::Divide(x.into(), y.into()) }
        --
        x:(@) _ "|" _ y:@ { Expression::BitOr(x.into(), y.into()) }
        --
        literal:literal() { Expression::Literal(literal) }
        identifier:identifier() { Expression::Identifier(identifier) }
        "(" _ expr:expression() _ ")" { Expression::Parentheses(expr.into()) }
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

    rule varargs() -> String
      = quiet!{ s:$("VARARGS" ['0'..='9']?) {
        s.to_owned()
      } }
      / expected!("varags")

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

    rule one_eol()
      = quiet!{"\r"? "\n"}
      / expected!("end of line")

    rule eof()
      = ![_]
  }
}

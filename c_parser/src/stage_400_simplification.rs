pub use crate::stage_320_parsing::PreprocessorDirective;
use crate::{
    stage_200_lexing::{Keyword, LexedToken as LT, Punctuator as Pr},
    stage_340_parsing::{Declaration, DeclaratorWithInit, ExternalDecl340, StructMember, StructMemberDeclaration, TranslationUnit340},
};
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationUnit400(pub Vec<ExternalDecl400>);
#[derive(Debug, Clone, PartialEq)]
pub enum ExternalDecl400 {
    Comment(String),
    PPInclude(crate::stage_320_parsing::IncludeDirective),
    PPDefine(crate::stage_320_parsing::DefineDirective),
    Declaration(Declaration400),
}
pub fn simplification(tu: TranslationUnit340) -> TranslationUnit400 {
    TranslationUnit400(
        tu.0.into_iter()
            .filter_map(|d| {
                Some(match d {
                    ExternalDecl340::Comment(s) => ExternalDecl400::Comment(s),
                    ExternalDecl340::Preprocessor(p) => match p {
                        PreprocessorDirective::Include(include_directive) => ExternalDecl400::PPInclude(include_directive),
                        PreprocessorDirective::Define(define_directive) => ExternalDecl400::PPDefine(define_directive),
                        PreprocessorDirective::Undef(_) => return None,
                        PreprocessorDirective::Other(lexed_tokens) => {
                            todo!("Other: {lexed_tokens:?}")
                        }
                    },
                    ExternalDecl340::Declaration(Declaration { specifiers, declarators }) => {
                        let mut storage = None;
                        let mut is_typedef = false;
                        let mut is_const = false;
                        let mut r#type = None;

                        let kind = {
                            let mut is_unsigned = false;
                            let mut kind = None;
                            for specifier in specifiers {
                                match specifier {
                                    crate::stage_340_parsing::SpecifierPiece::Storage(keyword) => {
                                        let s = match keyword {
                                            Keyword::Static => Storage::Static,
                                            Keyword::Extern => Storage::Extern,
                                            Keyword::Typedef => {
                                                assert!(!is_typedef);
                                                is_typedef = true;
                                                continue;
                                            }
                                            x => panic!("Unknown storage: {x:?}"),
                                        };
                                        assert!(storage.is_none());
                                        storage = Some(s);
                                    }
                                    crate::stage_340_parsing::SpecifierPiece::Qualifier(keyword) => match keyword {
                                        Keyword::Const => {
                                            assert!(!is_const);
                                            is_const = true;
                                        }
                                        x => panic!("Unknown qualifier: {x:?}"),
                                    },
                                    crate::stage_340_parsing::SpecifierPiece::Type(keyword) => {
                                        let t = match keyword {
                                            Keyword::Unsigned => {
                                                assert!(!is_unsigned);
                                                is_unsigned = true;
                                                continue;
                                            }
                                            Keyword::Void => {
                                                assert!(!is_unsigned);
                                                PrimitiveType::Void
                                            }
                                            Keyword::Int => {
                                                if is_unsigned {
                                                    PrimitiveType::UInt
                                                } else {
                                                    PrimitiveType::Int
                                                }
                                            }
                                            Keyword::Char => {
                                                if is_unsigned {
                                                    PrimitiveType::UChar
                                                } else {
                                                    PrimitiveType::Char
                                                }
                                            }
                                            Keyword::Short => {
                                                if is_unsigned {
                                                    PrimitiveType::UShort
                                                } else {
                                                    PrimitiveType::Short
                                                }
                                            }
                                            Keyword::Long => {
                                                assert!(!is_unsigned);
                                                PrimitiveType::Long
                                            }
                                            Keyword::Float => {
                                                assert!(!is_unsigned);
                                                PrimitiveType::Float
                                            }
                                            x => {
                                                panic!("Unknown type: {x:?}");
                                            }
                                        };
                                        assert!(r#type.is_none());
                                        r#type = Some(t);
                                    }
                                    crate::stage_340_parsing::SpecifierPiece::Struct { tag, fields } => {
                                        assert!(kind.is_none());
                                        let fields = fields
                                            .unwrap_or_default()
                                            .into_iter()
                                            .map(|x| match x {
                                                StructMember::Declaration(declaration) => simplify_struct_field(declaration),
                                                StructMember::Unparsed(tokens) => {
                                                    panic!("Unparsed: {tokens:?}")
                                                }
                                            })
                                            .collect::<Vec<_>>();

                                        kind = Some(Kind::Struct {
                                            global_variable_name: tag,
                                            fields,
                                        });
                                    }
                                    crate::stage_340_parsing::SpecifierPiece::Union { tag, fields } => {
                                        assert!(kind.is_none());
                                        let fields = fields
                                            .unwrap_or_default()
                                            .into_iter()
                                            .map(|x| match x {
                                                StructMember::Declaration(declaration) => simplify_struct_field(declaration),
                                                StructMember::Unparsed(tokens) => {
                                                    panic!("Unparsed: {tokens:?}")
                                                }
                                            })
                                            .collect::<Vec<_>>();

                                        kind = Some(Kind::Union {
                                            global_variable_name: tag,
                                            fields,
                                        });
                                    }
                                    crate::stage_340_parsing::SpecifierPiece::Enum { tag, enumerators } => {
                                        assert!(tag.is_none());
                                        assert!(kind.is_none());

                                        let mut lines = Vec::new();
                                        {
                                            let mut current_line = Vec::new();
                                            for entry in enumerators.unwrap() {
                                                match entry {
                                                    LT::Newline => {
                                                        continue;
                                                    }
                                                    LT::Punctuator(Pr::Comma) => {
                                                        if !current_line.is_empty() {
                                                            lines.push(std::mem::take(&mut current_line));
                                                        }
                                                        continue;
                                                    }
                                                    _ => current_line.push(entry),
                                                }
                                            }
                                            if !current_line.is_empty() {
                                                lines.push(current_line);
                                            }
                                        }

                                        let mut variants = Vec::new();

                                        for mut line in lines {
                                            let mut comments = Vec::new();
                                            line.retain(|x| {
                                                if let LT::LineComment(s) = x {
                                                    comments.push(s.clone());
                                                    false
                                                } else {
                                                    true
                                                }
                                            });
                                            let LT::Identifier(tag) = line.remove(0) else {
                                                panic!("Expected identifier")
                                            };
                                            let value = if let Some(x) = line.first() {
                                                assert_eq!(x, &LT::Punctuator(Pr::Equal));
                                                line.remove(0);
                                                assert!(!line.is_empty());
                                                line
                                            } else {
                                                Default::default()
                                            };
                                            variants.push(EnumVariant { tag, comments, value });
                                        }
                                        kind = Some(Kind::Enum { variants });
                                    }
                                }
                            }

                            kind
                        };

                        let storage = storage;
                        let is_typedef = is_typedef;
                        let is_const = is_const;
                        let r#type = r#type;
                        let declaration = if let Some(kind) = kind {
                            let type_definition = if declarators.is_empty() {
                                None
                            } else {
                                let [declarator]: [_; 1] = declarators.try_into().unwrap();
                                let DeclaratorWithInit {
                                    mut declarator,
                                    ast: _,
                                    initializer,
                                } = declarator;
                                let initializer = {
                                    if let Some(initializer) = initializer {
                                        parse_initializer(initializer)
                                    } else {
                                        Default::default()
                                    }
                                };
                                let is_packaged = {
                                    if Some(&LT::Identifier("PACKEDATTR".into())) == declarator.get(0) {
                                        declarator.remove(0);
                                        true
                                    } else {
                                        false
                                    }
                                };
                                let LT::Identifier(type_name) = declarator.remove(0) else {
                                    panic!("Expected identifier")
                                };
                                let array = {
                                    if let Some(LT::Punctuator(Pr::RBracket)) = declarator.pop() {
                                        assert_eq!(declarator.remove(0), LT::Punctuator(Pr::LBracket));
                                        if let Some(LT::Identifier(array)) = declarator.pop() {
                                            assert!(declarator.is_empty());
                                            TypeDefinitionArray::ArrayWithLength(array)
                                        } else {
                                            TypeDefinitionArray::ArrayWithoutLength
                                        }
                                    } else {
                                        TypeDefinitionArray::NoArray
                                    }
                                };

                                Some(TypeDefinition {
                                    type_name,
                                    array,
                                    initializer,
                                    is_packaged,
                                })
                            };
                            Declaration400::Type {
                                kind,
                                type_definition,
                                storage,
                                is_typedef,
                                is_const,
                                r#type,
                            }
                        } else {
                            let mut leading_comments = Vec::new();
                            let (r#type, variables) = combine_type_declarators(&mut leading_comments, declarators, r#type.map(TypeName::Primitive));

                            Declaration400::GlobalVariable {
                                r#type,
                                variables,
                                leading_comments,
                                is_typedef,
                                storage,
                                is_const,
                            }
                        };
                        ExternalDecl400::Declaration(declaration)
                    }
                    ExternalDecl340::FunctionDefinition { signature_tokens, body } => {
                        return None;
                        //println!("FunctionDefinition: {signature_tokens:?} {body:?}")
                    }
                    ExternalDecl340::UnparsedDeclaration(lexed_tokens) => {
                        panic!("This never occurs: UnparsedDeclaration: {lexed_tokens:?}")
                    }
                })
            })
            .collect(),
    )
}

#[derive(Debug, Clone, PartialEq)]
enum Declaration400 {
    Type {
        kind: Kind,
        type_definition: Option<TypeDefinition>,
        r#type: Option<PrimitiveType>,
        is_typedef: bool,
        storage: Option<Storage>,
        is_const: bool,
    },
    GlobalVariable {
        r#type: TypeName,
        variables: Vec<String>,
        is_typedef: bool,
        storage: Option<Storage>,
        is_const: bool,
        leading_comments: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum TypeDefinitionArray {
    NoArray,
    ArrayWithoutLength,
    ArrayWithLength(String),
}
#[derive(Debug, Clone, PartialEq)]
struct TypeDefinition {
    type_name: String,
    array: TypeDefinitionArray,
    initializer: Vec<LineItem>,
    is_packaged: bool,
}
#[derive(Debug, Clone, PartialEq)]
enum LineItem {
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral { value: usize },
    Minus,
    Plus,
    Divide,
    Multiply,
    Braced(Vec<LineItem>),
    Comma,
    LParen,
    RParen,
    Dot,
    Comment(String),
}
fn parse_initializer(mut initializer: Vec<LT>) -> Vec<LineItem> {
    fn parse_items(initializer: &mut Vec<LT>) -> Vec<LineItem> {
        let mut items = Vec::new();
        loop {
            if initializer.is_empty() {
                return items;
            }
            let current = initializer.remove(0);
            match current {
                LT::Newline => continue,
                LT::Punctuator(Pr::RBrace) => {
                    return items;
                }
                LT::Punctuator(Pr::LBrace) => {
                    let line = parse_items(initializer);
                    items.push(LineItem::Braced(line));
                }
                LT::IntegerLiteral { value, suffix } => {
                    assert!(suffix.is_none());
                    dbg!(&value);
                    items.push(LineItem::IntegerLiteral {
                        value: value.parse().unwrap(),
                    });
                }
                LT::StringLiteral(s) => {
                    items.push(LineItem::StringLiteral(s));
                }
                LT::Identifier(s) => {
                    items.push(LineItem::Identifier(s));
                }
                LT::Punctuator(x) => {
                    let item = match x {
                        Pr::Minus => LineItem::Minus,
                        Pr::Plus => LineItem::Plus,
                        Pr::Slash => LineItem::Divide,
                        Pr::Star => LineItem::Multiply,
                        Pr::Comma => LineItem::Comma,
                        Pr::LParen => LineItem::LParen,
                        Pr::RParen => LineItem::RParen,
                        Pr::Dot => LineItem::Dot,
                        _ => panic!("Unknown punctuator: {x:?}"),
                    };
                    items.push(item);
                }
                LT::LineComment(s) | LT::BlockComment(s) => {
                    items.push(LineItem::Comment(s));
                }
                x => panic!("Unknown token: {x:?}"),
            }
        }
    }

    parse_items(&mut initializer)
}

#[derive(Debug, Clone, PartialEq)]
enum Storage {
    Static,
    Extern,
}

#[derive(Debug, Clone, PartialEq)]
enum PrimitiveType {
    Void,
    UChar,
    SChar,
    Char,
    UShort,
    SShort,
    Short,
    UInt,
    SInt,
    Int,
    Long,
    Float,
}
impl PrimitiveType {
    fn from_keyword(keyword: Keyword, is_signed: bool, is_unsigned: bool) -> Self {
        assert!(!(is_signed && is_unsigned));
        match keyword {
            Keyword::Void => {
                assert!(!is_signed && !is_unsigned);
                Self::Void
            }
            Keyword::Char => {
                if is_unsigned {
                    Self::UChar
                } else if is_signed {
                    Self::SChar
                } else {
                    Self::Char
                }
            }
            Keyword::Int => {
                if is_unsigned {
                    Self::UInt
                } else if is_signed {
                    Self::SInt
                } else {
                    Self::Int
                }
            }
            Keyword::Short => {
                if is_unsigned {
                    Self::UShort
                } else if is_signed {
                    Self::SShort
                } else {
                    Self::Short
                }
            }
            Keyword::Long => {
                assert!(!is_unsigned && !is_signed);
                Self::Long
            }
            Keyword::Float => {
                assert!(!is_unsigned && !is_signed);
                Self::Float
            }
            x => panic!("Unknown keyword: {x:?}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArrayLength {
    String(String),
    Integer(usize),
}
#[derive(Debug, Clone, PartialEq)]
enum TypeName {
    Primitive(PrimitiveType),
    Defined(String),
    DefinedOnceInAllOfDoom(String),
    Pointer(Box<TypeName>),
    Array(Box<TypeName>, ArrayLength),
    FunctionPointerNoArguments { returns: Box<TypeName> },
    Unparsed { r#type: Option<Box<TypeName>>, lexed_tokens: Vec<LT> },
}
impl From<PrimitiveType> for TypeName {
    fn from(value: PrimitiveType) -> Self {
        TypeName::Primitive(value)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct StructFields {
    r#type: TypeName,
    field_names: Vec<String>,
    comments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
struct EnumVariant {
    tag: String,
    comments: Vec<String>,
    value: Vec<LT>,
}
#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Struct {
        global_variable_name: Option<String>,
        fields: Vec<StructFields>, //empty means:
    },
    Union {
        global_variable_name: Option<String>,
        fields: Vec<StructFields>,
    },
    Enum {
        variants: Vec<EnumVariant>,
    },
}

#[derive(Debug)]
enum DeclaratorHelper2 {
    FunctionPointer { name: String },
    Identifier2 { name1: String, name2: String },
    Identifier { name: String },
    Pointer { name: String },
    TypedPointer { r#type: String, name: String },
    DoublePointer2 { name: String },
    TypeDoublePointer { r#type: String, name: String },
    Array { array: String, length: String },
    TypedArray { r#type: String, array: String, length: String },
    ArrayInteger { array: String, value: usize },
    ArrayOfArrayInteger { array: String, length1: usize, length2: usize },
    BitField { r#type: String, name: String, value: usize },
    ArrayPointer { array: String, length: String },
    MultipleNamesPossibleWithType(Vec<Vec<String>>),
    Unparsed(Vec<LT>),
}
fn simplify_struct_field(declaration: Box<StructMemberDeclaration>) -> StructFields {
    let StructMemberDeclaration {
        mut leading_comments,
        declaration,
    } = *declaration;
    let Declaration { specifiers, declarators } = declaration;
    let r#type = {
        let mut r#type = None;
        let mut is_signed = false;
        let mut is_unsigned = false;
        for specifier in specifiers {
            let t: TypeName = match specifier {
                crate::stage_340_parsing::SpecifierPiece::Type(keyword) => {
                    match keyword {
                        Keyword::Signed => {
                            assert!(!is_signed);
                            assert!(r#type.is_none());
                            is_signed = true;
                            continue;
                        }
                        Keyword::Unsigned => {
                            assert!(!is_unsigned);
                            assert!(r#type.is_none());
                            is_unsigned = true;
                            continue;
                        }
                        x => PrimitiveType::from_keyword(x, is_signed, false),
                    }
                }
                .into(),
                crate::stage_340_parsing::SpecifierPiece::Struct { tag, fields } => {
                    let tag = tag.unwrap();
                    assert!(fields.is_none());
                    TypeName::Defined(tag)
                }
                // This happens exactly once
                crate::stage_340_parsing::SpecifierPiece::Union { tag, fields } => {
                    assert!(tag.is_none());
                    let expected = vec![
                        StructMember::Declaration(
                            StructMemberDeclaration {
                                leading_comments: [].into(),
                                declaration: Declaration {
                                    specifiers: [].into(),
                                    declarators: [DeclaratorWithInit {
                                        declarator: [LT::Identifier("mobj_t".into()), LT::Punctuator(Pr::Star), LT::Identifier("thing".into())]
                                            .into(),
                                        ast: None,
                                        initializer: None,
                                    }]
                                    .into(),
                                },
                            }
                            .into(),
                        ),
                        StructMember::Declaration(
                            StructMemberDeclaration {
                                leading_comments: [].into(),
                                declaration: Declaration {
                                    specifiers: [].into(),
                                    declarators: [DeclaratorWithInit {
                                        declarator: [LT::Identifier("line_t".into()), LT::Punctuator(Pr::Star), LT::Identifier("line".into())].into(),
                                        ast: None,
                                        initializer: None,
                                    }]
                                    .into(),
                                },
                            }
                            .into(),
                        ),
                    ];
                    assert_eq!(fields, Some(expected));
                    TypeName::DefinedOnceInAllOfDoom("union { mobj_t* thing; line_t* line;}".to_string())
                }
                x => panic!("Unknown specifier: {x:?}"),
            };
            assert!(r#type.is_none());
            r#type = Some(t);
        }
        if is_signed || is_unsigned {
            assert!(r#type.is_some());
        }
        r#type
    };

    let (r#type, field_names) = combine_type_declarators(&mut leading_comments, declarators, r#type);

    //dbg!(&declarators);
    StructFields {
        comments: leading_comments,
        r#type, //declaration,
        field_names,
    }
}

fn combine_type_declarators(
    leading_comments: &mut Vec<String>,
    declarators: Vec<DeclaratorWithInit>,
    r#type: Option<TypeName>,
) -> (TypeName, Vec<String>) {
    let declarators = extract_declarators(declarators);

    use DeclaratorHelper2 as DH2;
    let (r#type, field_names) = if let Some(r#type) = r#type {
        match declarators {
            DH2::FunctionPointer { name } => (TypeName::FunctionPointerNoArguments { returns: Box::new(r#type) }, vec![name]),
            DH2::Identifier { name } => (r#type, vec![name]),
            DH2::Pointer { name } => (TypeName::Pointer(Box::new(r#type)), vec![name]),
            DH2::DoublePointer2 { name } => (TypeName::Pointer(Box::new(TypeName::Pointer(Box::new(r#type)))), vec![name]),
            DH2::Array { array, length } => (TypeName::Array(Box::new(r#type), ArrayLength::String(length)), vec![array]),
            DH2::ArrayInteger { array, value } => (TypeName::Array(Box::new(r#type), ArrayLength::Integer(value)), vec![array]),
            DH2::ArrayOfArrayInteger { array, length1, length2 } => {
                let inner = TypeName::Array(Box::new(r#type), ArrayLength::Integer(length1));
                let outer = TypeName::Array(Box::new(inner), ArrayLength::Integer(length2));
                (outer, vec![array])
            }
            DH2::ArrayPointer { array, length } => (
                TypeName::Pointer(Box::new(TypeName::Array(Box::new(r#type), ArrayLength::String(length)))),
                vec![array],
            ),
            DH2::MultipleNamesPossibleWithType(items) => {
                let field_names = items
                    .into_iter()
                    .map(|x| {
                        let [x] = x.try_into().unwrap();
                        x
                    })
                    .collect::<Vec<_>>();
                (r#type, field_names)
            }
            DH2::Unparsed(lexed_tokens) => (
                TypeName::Unparsed {
                    r#type: Some(Box::new(r#type)),
                    lexed_tokens,
                },
                vec![],
            ),
            x => panic!("Unexpcted type declarator: {:?}: {x:?}", r#type),
        }
    } else {
        match declarators {
            DH2::Identifier2 { name1, name2 } => (TypeName::Defined(name1), vec![name2]),
            DH2::TypedPointer { r#type, name } => (TypeName::Pointer(Box::new(TypeName::Defined(r#type))), vec![name]),
            DH2::TypeDoublePointer { r#type, name } => (
                TypeName::Pointer(Box::new(TypeName::Pointer(Box::new(TypeName::Defined(r#type))))),
                vec![name],
            ),
            DH2::TypedArray { r#type, array, length } => (
                TypeName::Array(Box::new(TypeName::Defined(r#type)), ArrayLength::String(length)),
                vec![array],
            ),
            DH2::BitField { r#type, name, value } => {
                leading_comments.push(format!("bit field: {} {name}:{value}", r#type));
                (TypeName::Defined(r#type), vec![name])
            }
            DH2::MultipleNamesPossibleWithType(mut items) => {
                let mut field_names = Vec::new();
                let [r#type, field] = items.remove(0).try_into().unwrap();
                field_names.extend(items.into_iter().map(|x| {
                    let [x] = x.try_into().unwrap();
                    x
                }));
                field_names.push(field);
                (TypeName::Defined(r#type), field_names)
            }
            DH2::Unparsed(lexed_tokens) => (TypeName::Unparsed { r#type: None, lexed_tokens }, vec![]),
            x => panic!("Unexpcted untype declarator:{x:?}"),
        }
    };
    (r#type, field_names)
}

fn extract_declarators(mut declarators: Vec<DeclaratorWithInit>) -> DeclaratorHelper2 {
    if declarators.len() > 1 {
        let mut declarators_parsed = Vec::new();
        for DeclaratorWithInit {
            declarator,
            ast,
            initializer,
        } in declarators
        {
            assert!(initializer.is_none());
            let declarators = declarator
                .into_iter()
                .map(|x| match x {
                    LT::Identifier(s) => s,
                    _ => panic!("Unknown declarator: {x:?}"),
                })
                .collect::<Vec<_>>();
            if let Some(ast) = ast {
                assert!(ast.pointer_levels.is_empty());
                match ast.direct {
                    crate::stage_340_parsing::DirectDeclarator::Identifier(s) => {
                        assert_eq!(&declarators, &[s]);
                    }
                    _ => panic!("Unknown direct declarator: {ast:?}"),
                }
            }
            declarators_parsed.push(declarators);
        }
        DeclaratorHelper2::MultipleNamesPossibleWithType(declarators_parsed)
    } else if let Some(DeclaratorWithInit {
        declarator,
        ast: _,
        initializer,
    }) = declarators.pop()
    {
        let initializer = {
            if let Some(initializer) = initializer {
                parse_initializer(initializer)
            } else {
                Default::default()
            }
        };
        // assert!(initializer.is_none());
        match declarator.as_slice() {
            [] => panic!("Empty declarator"),
            [
                LT::Punctuator(Pr::LParen),
                LT::Punctuator(Pr::Star),
                LT::Identifier(x),
                LT::Punctuator(Pr::RParen),
                LT::Punctuator(Pr::LParen),
                LT::Punctuator(Pr::RParen),
            ] => DeclaratorHelper2::FunctionPointer { name: x.to_string() },
            [LT::Identifier(s)] => DeclaratorHelper2::Identifier { name: s.to_string() },
            [LT::Identifier(s1), LT::Identifier(s2)] => DeclaratorHelper2::Identifier2 {
                name1: s1.to_string(),
                name2: s2.to_string(),
            },
            [LT::Punctuator(p), LT::Identifier(s)] if *p == Pr::Star => DeclaratorHelper2::Pointer { name: s.to_string() },
            [LT::Identifier(t), LT::Punctuator(Pr::Star), LT::Identifier(s)] => DeclaratorHelper2::TypedPointer {
                r#type: t.to_string(),
                name: s.to_string(),
            },
            [LT::Punctuator(Pr::Star), LT::Punctuator(Pr::Star), LT::Identifier(s)] => DeclaratorHelper2::DoublePointer2 { name: s.to_string() },
            [LT::Identifier(t), LT::Punctuator(Pr::Star), LT::Punctuator(Pr::Star), LT::Identifier(s)] => DeclaratorHelper2::TypeDoublePointer {
                r#type: t.to_string(),
                name: s.to_string(),
            },
            [
                LT::Identifier(array),
                LT::Punctuator(Pr::LBracket),
                LT::Identifier(length),
                LT::Punctuator(Pr::RBracket),
            ] => DeclaratorHelper2::Array {
                array: array.to_string(),
                length: length.to_string(),
            },
            [
                LT::Identifier(r#type),
                LT::Identifier(array),
                LT::Punctuator(Pr::LBracket),
                LT::Identifier(length),
                LT::Punctuator(Pr::RBracket),
            ] => DeclaratorHelper2::TypedArray {
                r#type: r#type.to_string(),
                array: array.to_string(),
                length: length.to_string(),
            },
            [
                LT::Identifier(array),
                LT::Punctuator(Pr::LBracket),
                LT::IntegerLiteral { value, suffix: None },
                LT::Punctuator(Pr::RBracket),
            ] => DeclaratorHelper2::ArrayInteger {
                array: array.to_string(),
                value: value.parse::<usize>().unwrap(),
            },
            [
                LT::Punctuator(Pr::Star),
                LT::Identifier(array),
                LT::Punctuator(Pr::LBracket),
                LT::Identifier(length),
                LT::Punctuator(Pr::RBracket),
            ] => DeclaratorHelper2::ArrayPointer {
                array: array.to_string(),
                length: length.to_string(),
            },
            [
                LT::Identifier(array),
                LT::Punctuator(Pr::LBracket),
                LT::IntegerLiteral { value: v1, suffix: None },
                LT::Punctuator(Pr::RBracket),
                LT::Punctuator(Pr::LBracket),
                LT::IntegerLiteral { value: v2, suffix: None },
                LT::Punctuator(Pr::RBracket),
            ] => DeclaratorHelper2::ArrayOfArrayInteger {
                array: array.to_string(),
                length1: v1.parse::<usize>().unwrap(),
                length2: v2.parse::<usize>().unwrap(),
            },
            [
                LT::Identifier(r#type),
                LT::Identifier(name),
                LT::Punctuator(Pr::Colon),
                LT::IntegerLiteral { value: v, suffix: None },
            ] => DeclaratorHelper2::BitField {
                r#type: r#type.to_string(),
                name: name.to_string(),
                value: v.parse::<usize>().unwrap(),
            },
            x => DeclaratorHelper2::Unparsed(x.to_vec()),
        }
    } else {
        panic!("No declarators");
    }
}

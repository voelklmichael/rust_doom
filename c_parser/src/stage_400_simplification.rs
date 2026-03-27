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
                                crate::stage_340_parsing::SpecifierPiece::Union { tag, fields } => {}
                                crate::stage_340_parsing::SpecifierPiece::Enum { tag, enumerators } => {}
                                crate::stage_340_parsing::SpecifierPiece::TypedefName(name) => {}
                            }
                        }

                        return None;
                        //todo!("Declaration: {decl:?}")
                    }
                    ExternalDecl340::FunctionDefinition { signature_tokens, body } => {
                        return None;
                        //println!("FunctionDefinition: {signature_tokens:?} {body:?}")
                    }
                    ExternalDecl340::UnparsedDeclaration(lexed_tokens) => {
                        todo!("UnparsedDeclaration: {lexed_tokens:?}")
                    }
                })
            })
            .collect(),
    )
}

enum Storage {
    Static,
    Extern,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum ArrayLength {
    String(String),
    Integer(usize),
}
#[derive(Debug)]
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
pub struct StructFields {
    comments: Vec<String>,
    r#type: TypeName,
    field_names: Vec<String>,
}
enum Kind {
    Struct {
        global_variable_name: Option<String>,
        fields: Vec<StructFields>, //empty means:
    },
}

fn simplify_struct_field(declaration: Box<StructMemberDeclaration>) -> StructFields {
    let StructMemberDeclaration {
        leading_comments,
        declaration,
    } = *declaration;
    let Declaration { specifiers, mut declarators } = declaration;
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
    let declarators = {
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
            ast,
            initializer,
        }) = declarators.pop()
        {
            assert!(initializer.is_none());
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
    };

    if r#type.is_none() {
        //dbg!(&tag);
    }

    dbg!(&r#type);
    dbg!(&declarators);
    use DeclaratorHelper2 as DH2;
    let (r#type, field_names) = if let Some(r#type) = r#type {
        match declarators {
            DH2::FunctionPointer { name } => (TypeName::FunctionPointerNoArguments { returns: Box::new(r#type) }, vec![name]),
            DH2::Identifier2 { name1, name2 } => todo!(),
            DH2::Identifier { name } => (r#type, vec![name]),
            DH2::Pointer { name } => (TypeName::Pointer(Box::new(r#type)), vec![name]),
            DH2::TypedPointer { r#type, name } => todo!(),
            DH2::DoublePointer2 { name } => todo!(),
            DH2::TypeDoublePointer { r#type, name } => todo!(),
            DH2::Array { array, length } => (TypeName::Array(Box::new(r#type), ArrayLength::String(length)), vec![array]),
            DH2::TypedArray { r#type, array, length } => todo!(),
            DH2::ArrayInteger { array, value } => (TypeName::Array(Box::new(r#type), ArrayLength::Integer(value)), vec![array]),
            DH2::ArrayOfArrayInteger { array, length1, length2 } => {
                let inner = TypeName::Array(Box::new(r#type), ArrayLength::Integer(length1));
                let outer = TypeName::Array(Box::new(inner), ArrayLength::Integer(length2));
                (outer, vec![array])
            }
            DH2::BitField { r#type, name, value } => todo!(),
            DH2::ArrayPointer { array, length } => (
                TypeName::Pointer(Box::new(TypeName::Array(Box::new(r#type), ArrayLength::String(length)))),
                vec![array],
            ),
            DH2::MultipleNamesPossibleWithType(items) => {
                dbg!(&items, &r#type);
                let field_names = items
                    .into_iter()
                    .map(|x| {
                        let [x] = x.try_into().unwrap();
                        x
                    })
                    .collect::<Vec<_>>();
                (r#type, field_names)
            }
            DeclaratorHelper2::Unparsed(lexed_tokens) => (
                TypeName::Unparsed {
                    r#type: Some(Box::new(r#type)),
                    lexed_tokens,
                },
                vec![],
            ),
        }
    } else {
        match declarators {
            DH2::FunctionPointer { name } => todo!(),
            DH2::Identifier2 { name1, name2 } => (TypeName::Defined(name1), vec![name2]),
            DH2::Identifier { name } => todo!(),
            DH2::Pointer { name } => todo!(),
            DH2::TypedPointer { r#type, name } => (TypeName::Pointer(Box::new(TypeName::Defined(r#type))), vec![name]),
            DH2::DoublePointer2 { name } => todo!(),
            DH2::TypeDoublePointer { r#type, name } => todo!(),
            DH2::Array { array, length } => todo!(),
            DH2::TypedArray { r#type, array, length } => (
                TypeName::Array(Box::new(TypeName::Defined(r#type)), ArrayLength::String(length)),
                vec![array],
            ),
            DH2::ArrayInteger { array, value } => todo!(),
            DH2::ArrayOfArrayInteger { array, length1, length2 } => todo!(),
            DH2::BitField { r#type, name, value } => todo!(),
            DH2::ArrayPointer { array, length } => todo!(),
            DH2::MultipleNamesPossibleWithType(mut items) => {
                dbg!(&items);
                let mut field_names = Vec::new();
                let [r#type, field] = items.remove(0).try_into().unwrap();
                field_names.extend(items.into_iter().map(|x| {
                    let [x] = x.try_into().unwrap();
                    x
                }));
                field_names.push(field);
                (TypeName::Defined(r#type), field_names)
            }
            DH2::Unparsed(lexed_tokens) => todo!(),
        }
    };

    //dbg!(&declarators);
    StructFields {
        comments: leading_comments,
        r#type, //declaration,
        field_names,
    }
}

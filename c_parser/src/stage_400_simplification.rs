pub use crate::stage_320_parsing::PreprocessorDirective;
use crate::{
    stage_200_lexing::{Keyword, LexedToken},
    stage_340_parsing::{
        Declaration, DeclaratorWithInit, ExternalDecl340, StructMember, StructMemberDeclaration, TranslationUnit340,
    },
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
                        PreprocessorDirective::Include(include_directive) => {
                            ExternalDecl400::PPInclude(include_directive)
                        }
                        PreprocessorDirective::Define(define_directive) => ExternalDecl400::PPDefine(define_directive),
                        PreprocessorDirective::Undef(_) => return None,
                        PreprocessorDirective::Other(lexed_tokens) => {
                            todo!("Other: {lexed_tokens:?}")
                        }
                    },
                    ExternalDecl340::Declaration(Declaration {
                        specifiers,
                        declarators,
                    }) => {
                        enum Storage {
                            Static,
                            Extern,
                        }
                        let mut storage = None;
                        let mut is_typedef = false;
                        let mut is_const = false;
                        let mut r#type = None;
                        let mut is_unsigned = false;

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
                        enum TypeName {
                            Primitive(PrimitiveType),
                            Struct(String),
                            DefinedOnceInAllOfDoom(String),
                        }
                        impl From<PrimitiveType> for TypeName {
                            fn from(value: PrimitiveType) -> Self {
                                TypeName::Primitive(value)
                            }
                        }
                        pub struct StructFields {
                            comments: Vec<String>,
                            r#type: Option<TypeName>,
                        }
                        enum Kind {
                            Struct {
                                global_variable_name: Option<String>,
                                fields: Vec<StructFields>, //empty means:
                            },
                        }
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
                                    let fields =
                                        fields
                                            .unwrap_or_default()
                                            .into_iter()
                                            .map(|x| match x {
                                                StructMember::Declaration(declaration) => {
                                                    let StructMemberDeclaration {
                                                        leading_comments,
                                                        declaration,
                                                    } = *declaration;
                                                    let Declaration {
                                                        specifiers,
                                                        declarators,
                                                    } = declaration;
                                                    let r#type =
                                                        {
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
                                                        crate::stage_340_parsing::SpecifierPiece::Struct {
                                                            tag,
                                                            fields,
                                                        } => {
                                                            let tag = tag.unwrap();
                                                            assert!(fields.is_none());
                                                            TypeName::Struct(tag)
                                                        }
                                                        // This happens exactly once
                                                        crate::stage_340_parsing::SpecifierPiece::Union {
                                                            tag,
                                                            fields,
                                                        } => {
                                                            assert!(tag.is_none());
                                                            let expected = vec![
                                                                StructMember::Declaration(
                                                                    StructMemberDeclaration {
                                                                        leading_comments: [].into(),
                                                                        declaration: Declaration {
                                                                            specifiers: [].into(),
                                                                            declarators: [DeclaratorWithInit {
                                                                                declarator: [
                                                                                    LexedToken::Identifier(
                                                                                        "mobj_t".into(),
                                                                                    ),
                                                                                    LexedToken::Punctuator("*".into()),
                                                                                    LexedToken::Identifier(
                                                                                        "thing".into(),
                                                                                    ),
                                                                                ]
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
                                                                                declarator: [
                                                                                    LexedToken::Identifier(
                                                                                        "line_t".into(),
                                                                                    ),
                                                                                    LexedToken::Punctuator("*".into()),
                                                                                    LexedToken::Identifier(
                                                                                        "line".into(),
                                                                                    ),
                                                                                ]
                                                                                .into(),
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
                                                            dbg!(TypeName::DefinedOnceInAllOfDoom(
                                                                "union { mobj_t* thing; line_t* line;}"
                                                                .to_string(),
                                                            ))
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
                                                    let declarators = {
                                                        for DeclaratorWithInit {
                                                            declarator,
                                                            ast,
                                                            initializer,
                                                        } in declarators
                                                        {
                                                            dbg!(&declarator);
                                                        }
                                                    };

                                                    if r#type.is_none() {
                                                        dbg!(&tag);
                                                        panic!("r#type is none");
                                                    }

                                                    //dbg!(&declarators);
                                                    StructFields {
                                                        comments: leading_comments,
                                                        r#type, //declaration,
                                                    }
                                                }
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
                                crate::stage_340_parsing::SpecifierPiece::TypedefName(_) => {}
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

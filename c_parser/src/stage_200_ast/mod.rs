use crate::stage_110_preprocessor::Stage110Preprocessor;

mod code;
mod comment;
mod define;
mod include;
mod undef;

/// Content chunk inside struct/function bodies - allows Comments interleaved with Code.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BodyChunk {
    Comment(String),
    Code(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Stage200Ast {
    Comment(String),
    Include {
        path: String,
        is_system: bool,
    },
    GlobalConstant {
        name: String,
        value: String,
    },
    MacroFunction {
        name: String,
        argument_names: Vec<String>,
        body: String,
    },
    TypedefEnum {
        name: String,
        variants: Vec<EnumVariant>,
    },
    TypedefStruct {
        name: String,
        body: Vec<BodyChunk>,
    },
    TypedefUnion {
        name: String,
        body: Vec<BodyChunk>,
    },
    StructDef {
        name: String,
        body: Vec<BodyChunk>,
    },
    UnionDef {
        name: String,
        body: Vec<BodyChunk>,
    },
    EnumDef {
        name: String,
        variants: Vec<EnumVariant>,
    },
    FunctionDecl {
        return_type: String,
        name: String,
        params: String,
    },
    FunctionDef {
        return_type: String,
        name: String,
        params: String,
        body: Vec<BodyChunk>,
    },
    TypedefSimple {
        base_type: String,
        name: String,
    },
    /// Variable or other declaration we don't parse in detail
    OtherDecl(String),
    /// Unparsed code - for feedback on what constructs need implementation
    Unparsed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
}

/// Stream of tokens that allows reading block bodies across Code/Comment boundaries.
pub(super) struct TokenStream {
    tokens: Vec<Stage110Preprocessor>,
    token_idx: usize,
}

impl TokenStream {
    fn new(tokens: Vec<Stage110Preprocessor>) -> Self {
        Self {
            tokens,
            token_idx: 0,
        }
    }

    fn current(&self) -> Option<&Stage110Preprocessor> {
        self.tokens.get(self.token_idx)
    }

    pub(super) fn advance(&mut self) {
        self.token_idx += 1;
    }

    pub(super) fn advance_by(&mut self, n: usize) {
        self.token_idx += n;
    }

    /// Advance to the next Code token, skipping Comments. Returns the code string.
    /// Leaves stream positioned at that Code token. Caller must advance past it when done.
    pub(super) fn advance_to_next_code(&mut self) -> Option<String> {
        self.advance();
        while let Some(Stage110Preprocessor::Comment(_)) = self.current() {
            self.advance();
        }
        match self.current() {
            Some(Stage110Preprocessor::Code(c)) => Some(c.clone()),
            _ => None,
        }
    }

    /// Read balanced braces from current position. Consumes tokens as needed.
    /// Returns (chunks, tokens_consumed) or None. tokens_consumed includes the current Code.
    pub(super) fn read_balanced_block(
        &mut self,
        code: &str,
        start_byte: usize,
    ) -> Option<(Vec<BodyChunk>, usize)> {
        let mut chunks = Vec::new();
        let mut buffer: Vec<u8> = code.as_bytes().to_vec();
        let mut i = start_byte;
        let start_token = self.token_idx;

        if i >= buffer.len() || buffer[i] != b'{' {
            return None;
        }
        i += 1;
        let mut depth: u32 = 1;
        let mut current_code_start = i;

        loop {
            while i < buffer.len() && depth > 0 {
                match buffer[i] {
                    b'{' => {
                        depth = depth.saturating_add(1);
                        if depth > 1_000_000 {
                            return None;
                        }
                        i += 1;
                    }
                    b'}' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            if current_code_start < i {
                                let s = String::from_utf8_lossy(&buffer[current_code_start..i])
                                    .to_string();
                                if !s.trim().is_empty() {
                                    chunks.push(BodyChunk::Code(s));
                                }
                            }
                            let consumed = self.token_idx - start_token + 1;
                            return Some((chunks, consumed));
                        }
                        i += 1;
                    }
                    b'"' => {
                        i += 1;
                        while i < buffer.len() && buffer[i] != b'"' {
                            if buffer[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < buffer.len() {
                            i += 1;
                        }
                    }
                    b'\'' => {
                        i += 1;
                        while i < buffer.len() && buffer[i] != b'\'' {
                            if buffer[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < buffer.len() {
                            i += 1;
                        }
                    }
                    _ => i += 1,
                }
            }

            if depth == 0 {
                break;
            }

            if current_code_start < i {
                let s = String::from_utf8_lossy(&buffer[current_code_start..i]).to_string();
                if !s.trim().is_empty() {
                    chunks.push(BodyChunk::Code(s));
                }
            }

            self.advance();
            match self.current() {
                Some(Stage110Preprocessor::Comment(_c)) => {
                    chunks.push(BodyChunk::Comment(_c.clone()));
                    self.advance();
                }
                Some(Stage110Preprocessor::Code(c)) => {
                    buffer = c.as_bytes().to_vec();
                    current_code_start = 0;
                    i = 0;
                }
                _ => return None,
            }
        }

        let consumed = self.token_idx - start_token + 1;
        Some((chunks, consumed))
    }
}

impl Stage200Ast {
    pub fn parse(tokens: Vec<Stage110Preprocessor>) -> Vec<Self> {
        let mut results = Vec::new();
        let mut stream = TokenStream::new(tokens);

        while let Some(token) = stream.current() {
            match token {
                Stage110Preprocessor::Comment(..) => {
                    if let Some(ast) = comment::handle(token) {
                        results.push(ast);
                    }
                    stream.advance();
                }
                Stage110Preprocessor::Include { .. } => {
                    if let Some(ast) = include::handle(token) {
                        results.push(ast);
                    }
                    stream.advance();
                }
                Stage110Preprocessor::Define { .. } => {
                    if let Some(ast) = define::handle(token) {
                        results.push(ast);
                    }
                    stream.advance();
                }
                Stage110Preprocessor::Undef { .. } => {
                    stream.advance();
                }
                Stage110Preprocessor::Code(code) => {
                    let code = code.clone();
                    let parsed = code::handle(&code, &mut stream);
                    results.extend(parsed);
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests;

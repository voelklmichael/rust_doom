pub(crate) enum Stage100Comments {
    Comment(String),
    NonComment(String),
}

impl Stage100Comments {
    pub fn parse(content: &str) -> Vec<Self> {
        let mut result = Vec::new();
        let mut i = 0;
        let bytes = content.as_bytes();

        while i < bytes.len() {
            // Start of line/block comment (not inside string/char)
            if bytes[i] == b'/' && i + 1 < bytes.len() {
                match bytes[i + 1] {
                    b'/' => {
                        let start = i;
                        i += 2;
                        while i < bytes.len() && bytes[i] != b'\n' {
                            i += 1;
                        }
                        if i < bytes.len() {
                            i += 1; // include the newline
                        }
                        result.push(Stage100Comments::Comment(content[start..i].to_string()));
                        continue;
                    }
                    b'*' => {
                        let start = i;
                        i += 2;
                        while i + 1 < bytes.len() {
                            if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                                i += 2;
                                break;
                            }
                            i += 1;
                        }
                        result.push(Stage100Comments::Comment(content[start..i].to_string()));
                        continue;
                    }
                    _ => {}
                }
            }

            // Non-comment: scan until start of comment or end, skip string/char literals
            let start = i;
            while i < bytes.len() {
                match bytes[i] {
                    b'"' => {
                        i += 1;
                        while i < bytes.len() && bytes[i] != b'"' {
                            if bytes[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < bytes.len() {
                            i += 1;
                        }
                    }
                    b'\'' => {
                        i += 1;
                        while i < bytes.len() && bytes[i] != b'\'' {
                            if bytes[i] == b'\\' {
                                i += 1;
                            }
                            i += 1;
                        }
                        if i < bytes.len() {
                            i += 1;
                        }
                    }
                    b'/' if i + 1 < bytes.len() => match bytes[i + 1] {
                        b'/' | b'*' => break,
                        _ => i += 1,
                    },
                    _ => i += 1,
                }
            }
            if start < i {
                result.push(Stage100Comments::NonComment(
                    content[start..i].to_string(),
                ));
            }
        }

        result
    }
}

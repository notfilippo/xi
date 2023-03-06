use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::token::Token;

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected character")]
#[diagnostic(code(ix::lexer::unexpected_char))]
pub struct UnexpectedCharacter {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("malformed float precision")]
#[diagnostic(code(ix::lexer::malformed_precision))]
pub struct MalformedFloatPrecision {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("malformed number")]
#[diagnostic(code(ix::lexer::malformed_number))]
pub struct MalformedNumber {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unterminated sequence")]
#[diagnostic(code(ix::lexer::unterminated_sequence))]
pub struct UnterminatedSequence {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected token")]
#[diagnostic(code(ix::lexer::unexpected_token))]
pub struct UnexpectedToken {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
    #[help]
    pub help: String
}

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected eof")]
#[diagnostic(code(ix::lexer::unexpected_eof))]
pub struct UnexpectedEof {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("expected token")]
#[diagnostic(code(ix::lexer::expected_token))]
pub struct ExpectedToken {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unsupported operation")]
#[diagnostic(code(ix::interpreter::unsupported_operation))]
pub struct UnsupportedOperation {
    #[label("here")]
    pub span: SourceSpan,
    #[source_code]
    pub src: String,
}

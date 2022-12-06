use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected character")]
#[diagnostic(code(ix::lexer::unexpected_char))]
pub struct UnexpectedCharacter {
    #[source_code]
    pub src: String,
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("malformed number")]
#[diagnostic(code(ix::lexer::malformed_number))]
pub struct MalformedNumber {
    #[source_code]
    pub src: String,
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unterminated sequence")]
#[diagnostic(code(ix::lexer::unterminated_sequence))]
pub struct UnterminatedSequence {
    #[source_code]
    pub src: String,
    #[label("here")]
    pub span: SourceSpan,
}

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected character")]
#[diagnostic(code(ix::lexer::unexpected_char))]
pub struct UnexpectedCharacter {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("malformed float precision")]
#[diagnostic(code(ix::lexer::malformed_precision))]
pub struct MalformedFloatPrecision {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("malformed number")]
#[diagnostic(code(ix::lexer::malformed_number))]
pub struct MalformedNumber {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unterminated sequence")]
#[diagnostic(code(ix::lexer::unterminated_sequence))]
pub struct UnterminatedSequence {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected token")]
#[diagnostic(code(ix::lexer::unexpected_token))]
pub struct UnexpectedToken {
    #[label("here")]
    pub span: SourceSpan,
    #[help]
    pub help: String,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unexpected eof")]
#[diagnostic(code(ix::lexer::unexpected_eof))]
pub struct UnexpectedEof {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("expected token")]
#[diagnostic(code(ix::lexer::expected_token))]
pub struct ExpectedToken {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unsupported operation")]
#[diagnostic(code(ix::interpreter::unsupported_operation))]
pub struct UnsupportedOperation {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("undefined value")]
#[diagnostic(code(ix::env::undefined_value))]
pub struct UndefinedValue {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("invalid assignment target")]
#[diagnostic(code(ix::interpreter::invalid_assignment_target))]
pub struct InvalidAssignmentTarget {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("only functions and classes can be called")]
#[diagnostic(code(ix::interpreter::callee_type_error))]
pub struct CalleeTypeError {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("illegal to read local variable in its own initializer")]
#[diagnostic(code(ix::resolver::read_local_variable_in_own_initializer))]
pub struct ReadLocalVariableInOwnInitializer {
    #[label("here")]
    pub span: SourceSpan,
}

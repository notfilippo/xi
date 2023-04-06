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
#[diagnostic(code(ix::parser::unexpected_token))]
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
#[error("undefined property")]
#[diagnostic(code(ix::interpreter::undefined_property))]
pub struct UndefinedProperty {
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
#[error("only instances have properties")]
#[diagnostic(code(ix::interpreter::instance_type_error))]
pub struct InstanceTypeError {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("only lists and dicts can be indexed")]
#[diagnostic(code(ix::interpreter::list_type_error))]
pub struct IndexTypeError {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("lists can only be indexed with integers up to `usize`")]
#[diagnostic(code(ix::interpreter::invalid_index_error))]
pub struct ListIndexInvalidError {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("dict key does not exist")]
#[diagnostic(code(ix::interpreter::invalid_index_error))]
pub struct ListIndexOutOfBoundsError {
    #[label("here")]
    pub span: SourceSpan,
}

#[derive(Error, Debug, Diagnostic)]
#[error("dict key does not exist")]
#[diagnostic(code(ix::interpreter::invalid_index_error))]
pub struct DictKeyError {
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

pub enum ParseResult<T, E> {
    Ok(T),
    Err(E),
}

/// Type alias for ParseResult and ParseError
pub type PResult<T, E> = ParseResult<T, E>;

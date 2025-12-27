pub trait TokenType {}

pub trait LexerTrait<T>
where
    T: TokenType,
{
    fn next_token(&mut self) -> T;
    fn peek_next(&self) -> (T, usize);
    fn go_to(&mut self, i: usize) -> ();
}
pub trait DebugLexerTrait<T>: LexerTrait<T>
where
    T: TokenType,
{
    fn next_token_dbg(&mut self) -> T;
    fn recap(&self) -> &str;
    fn get_index(&self) -> usize;
}

pub trait TreeChecker<N>
where
    N: AstNodeTrait,
{
    type CheckError;
    fn check(&mut self, ast: &N) -> Result<(), Self::CheckError>;
}

pub trait AstNodeTrait {}

// pub trait OptPassTrait {}

pub trait CodeGen<N>
where
    N: AstNodeTrait,
{
    fn generate(self, ast: &N) -> ();
}

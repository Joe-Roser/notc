# Name

Plan:
lexer
parser
codegen
optimiser

### Required to function

lexer:
- next_token(&mut self) -> Token;
- peek_next(&mut self) -> (Token, i);

AstNode:
- evaluate(&self) -> Result<Literal, ()>; [For Interpreting]: #

Interpreter:

CodeGenerator:
- generate(&self, ASTNode) -> Repr;

###  Optimisers

ASTOptimiser:
- optimise(&mut self, node: dyn AstNode);

IROptimiser:
- optimise(&mut self, code: Asm) -> Repr;

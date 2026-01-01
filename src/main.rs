use notc::codegen::c::CCodeGen;
use notc::lexer::Lexer;
use notc::parsing::SpannedAstTree;
use notc::traits::{CodeGen, TreeChecker};
use notc::tree_checker::{NameResolver, TypeChecker};
use std::rc::Rc;

fn main() {
    let source_file: Rc<str> = std::fs::read_to_string("input.nc").unwrap().into();

    let mut lexer = Lexer::from_rc_str(source_file.clone());
    let mut ast = SpannedAstTree::from_rc_str(source_file.clone());

    let r = ast.parse_all(&mut lexer).expect("Parsing error");

    let mut nr = NameResolver::from_rc_str(source_file.clone()).pre_intern(notc::PRIMATIVE_TYPES);
    let ast = dbg!(nr.resolve(ast));

    TypeChecker::new().check(&ast).expect("Type error");

    let out = std::fs::File::create("input.c").unwrap();
    CCodeGen::new(source_file.clone(), out)
        .generate(&ast)
        .expect("Error writing to file");
}

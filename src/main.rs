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

    let r = ast.parse_all(&mut lexer);
    ast.debug_ast_result(&r);
    r.unwrap();

    let ast = NameResolver::from_rc_str(source_file.clone())
        .pre_intern(notc::PRIMATIVE_TYPES)
        .resolve(ast);
    dbg!(&ast);

    let r = TypeChecker::new().check(&ast);
    TypeChecker::debug_check_result(&r, source_file.clone());
    r.unwrap();

    let out = std::fs::File::create("input.c").unwrap();
    let r = CCodeGen::new(source_file.clone(), out).generate(&ast);
    println!("{:?}", r);
}

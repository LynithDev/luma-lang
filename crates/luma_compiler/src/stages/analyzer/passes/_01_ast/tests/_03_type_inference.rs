use pretty_assertions::assert_eq;

use crate::{TypeKind, ast::*};

use crate::stages::analyzer::passes::_01_ast::tests::{analyze_source, extract_stmt};

#[test]
pub fn basic_var_inference() {
    let ast = analyze_source(r#"
        // 1. default type inference
        var a = 10;
        
        // 2. explicit type inference
        var b: u64 = 10;

        // 3. implicit float inference
        var c = 3.14;
    "#).expect("failed to analyze source");

    // 1. default type inference
    extract_stmt!(
        StmtKind::Var(VarDeclStmt {
            initializer, ty: var_ty, ..
        }) = ast[0] 
    );

    // checking initializer type
    assert_eq!(
        initializer.ty,
        Some(TypeKind::Int32)
    );

    // checking variable type
    assert_eq!(
        var_ty.as_ref().expect("variable type should be inferred").kind,
        TypeKind::Int32
    );

    // 2. explicit type inference
    extract_stmt!(
        StmtKind::Var(VarDeclStmt {
            initializer, ..
        }) = ast[1] 
    );

    // checking initializer type
    assert_eq!(
        initializer.ty,
        Some(TypeKind::UInt64)
    );


    // 3. implicit float inference
    extract_stmt!(
        StmtKind::Var(VarDeclStmt {
            initializer, ty: var_ty, ..
        }) = ast[2] 
    );

    // checking initializer type
    assert_eq!(
        initializer.ty,
        Some(TypeKind::Float32)
    );

    // checking variable type
    assert_eq!(
        var_ty.as_ref().expect("variable type should be inferred").kind,
        TypeKind::Float32
    );
}

#[test]
fn func_type_inference() {
    let ast = analyze_source(r#"
        func test(): i8 {
            var a = 5;
            a
        };
    "#).expect("failed to analyze source");

    extract_stmt!(
        StmtKind::Func(FuncDeclStmt {
            body, ..
        }) = ast[0] 
    );

    let stmt = match &body.item {
        ExprKind::Block(block) => block.statements.last().unwrap().item.clone(),
        _ => panic!("expected function body to be a block expression"),
    };

    let StmtKind::Var(decl) = stmt else {
        panic!("expected last statement in function body to be a variable declaration");
    };

    assert_eq!(
        decl.initializer.ty,
        Some(TypeKind::Int8)
    );
}

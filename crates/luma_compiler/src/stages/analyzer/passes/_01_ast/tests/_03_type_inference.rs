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
    assert!(matches!(
        initializer.ty,
        Some(TypeKind::Int32)
    ));

    // checking variable type
    assert!(matches!(
        var_ty.as_ref().expect("variable type should be inferred").kind,
        TypeKind::Int32
    ));


    // 2. explicit type inference
    extract_stmt!(
        StmtKind::Var(VarDeclStmt {
            initializer, ..
        }) = ast[1] 
    );

    // checking initializer type
    assert!(matches!(
        initializer.ty,
        Some(TypeKind::UInt64)
    ));


    // 3. implicit float inference
    extract_stmt!(
        StmtKind::Var(VarDeclStmt {
            initializer, ty: var_ty, ..
        }) = ast[2] 
    );

    // checking initializer type
    assert!(matches!(
        initializer.ty,
        Some(TypeKind::Float32)
    ));

    // checking variable type
    assert!(matches!(
        var_ty.as_ref().expect("variable type should be inferred").kind,
        TypeKind::Float32
    ));
}

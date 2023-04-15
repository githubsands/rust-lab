
fn entire_function_scope() {
    let x = 1;
    {
        let y = 2;
        println!("{}, {}", x, y);
    }
}

fn function_body_scope() {
    let x = 1;
    {
        let y = 2;
        println!("{}, {}", x, y);
    }
    // y is out of scope here
    println!("{}", x);
}

fn expression_statement_scope() {
    let x = 1;
    {
        let y = 2;
        println!("{}, {}", x, y);
    }
    let z = {
        let a = 3;
        x + a // the parent of this expression is the scope of the statement
    };
    println!("{}", z);
}

fn let_statement_scope() {
    let x = 1;
    {
        let y = 2;
        println!("{}, {}", x, y);
    }
    let z: i32;
    {
        let a = 3;
        z = x + a; // the parent of this initializer is the let statement's scope
    }
    println!("{}", z);
}

fn main(){}

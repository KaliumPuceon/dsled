use std::rc::Rc;

struct Term {

    operation: String       // Function call for this operation

}

struct Const {

    parent: Option<Rc<Term>>,
    value: f32,

}

enum TermVal {

    Term(Term),
    Const(Const),

}


fn mul(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn div(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn add(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn sub(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn sin(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn cos(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn tan(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

fn abs(terms: Vec<Term>) -> Result<Term,String> {
    todo!();
}

const OPS:  [( &str, fn(Vec<Term>) -> Result<Term,String> ); 8] = [
    ("*", mul),
    ("/", div),
    ("+", add),
    ("-", sub),
    ("sin", sin),
    ("cos", cos),
    ("tan", tan),
    ("abs", abs),
];

fn process_term(opstring: String, remainder: String) -> Term {

    todo!();

}

fn parse(code: String, parent: Option<Rc<Term>>) -> Term {

    /*
     * * Strip newlines and split on whitespace and ( ) to create list of token strings
     *
     * * Pop and discard chars until ( , then get keyword, first thing you see until next whitespace or bracket is the operation
     *
     * * If you see a number or constant, pop to children and continue
     *
     * * Elif you see another ( or keyword, recurse on that String before adding it to the children
     *
     * * Elif you see a ), you're done. Make sure you wipe out the text you've parsed and return Term
     *
     * THOUGHTS:
     * oh hey yeah you can totally simplify this bottom-up in a single pass with how this is
     * implemented, leaf terms will always be fully defined first and can be simplified before
     * handing the final Term up. Later though.
     *
     */
    
    
    let sliced = code.split(&[' ']);

    for k in sliced {

        println!("{}", k);

    }

    todo!();

}

fn main() {
    let input = "( + 3 ( * 2 3 ) 4 4 ( sin 0.3 ) )";
    parse(input.to_string(), None);
    println!("Parsing code {}", input);
}

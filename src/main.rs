use std::collections::VecDeque;

struct Block {

    operation: fn(Vec<Term>),       // Function call for this operation
    children: Vec<Term>,            // Argterms of this term

}

enum Term {

    Constant(f32),
    Var(String),
    Subterm(Vec<Block>),

}


fn mul(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn div(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn add(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn sub(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn sin(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn cos(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn tan(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

fn abs(terms: Vec<Term>) -> Result<Block,String> {
    todo!();
}

const OPS:  [( &str, fn(Vec<Term>) -> Result<Block,String> ); 8] = [
    ("*", mul),
    ("/", div),
    ("+", add),
    ("-", sub),
    ("sin", sin),
    ("cos", cos),
    ("tan", tan),
    ("abs", abs),
];

// Take in a Vec of Strings, and break it up into a vec of Terms which contain their own
// vecs of Terms. If something is not a subterm, just append it to the current running vec of
// termvals. If you're entering a new subterm, split it out and parse that recursively.
fn process_term(opstring: &mut VecDeque<&str>) -> Result<(Vec<Term>, i32), String> {

    let mut expression: Vec<Term>;

    let mut before_term = true;
    let mut in_term = false;
    let mut end_of_term = false;
    let mut found_op = false;

    let mut result_op: fn(Vec<Term>) -> Result<Block, String>;
    let mut result_children: Vec<Term>;

    while !end_of_term || before_term {

        if let Some(next) = opstring.pop_front() {

            if next == "(" && !in_term {
                println!("first paren in term");
                in_term = true;
                before_term = false;
            }

            else if next == ")" && in_term {
                end_of_term = true;
                in_term = false;
            }

            else if next == "(" && in_term {
                opstring.push_front("(");
                println!("Passing down {:?}", opstring);
                if let Ok(subterm) = process_term(opstring) {
                    
                }
            }

            else if let Some(matched_fun) = OPS.iter().find(|op| (op.0 == next)) {
                println!("matched on a function {}", matched_fun.0);
                result_op = matched_fun.1;
            }
            else {

                if let Ok(num) = next.parse::<f32>() {
                    
                }

            }

            println!("{}",next);
        }
        else 
        {

            end_of_term = true;
            return Err("Badly matched parens somewhere".to_string())
        
        }

    }

    println!("end of term, returning up");
    Err("Not Working Yet".to_string())

}

fn parse(code: &str) -> Result<(Vec<Term>, i32), String>  {

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
    
    
    let lisp = code.to_string();
    let mut sliced = lisp.split(&[' ']).collect();

    if let Err(output) = process_term(&mut sliced) {

        println!("Error {}", output);

    }


    Err("Not Working Yet".to_string())

}

fn main() {
    //let input = "( + 3 ( * 2 3 ) 4 5 ( sin 0.3 ) )";
    let input = "( + 13 ( - 4 5 ) )";
    println!("parsing {}", input);
    parse(input);
    println!("Parsing code {}", input);
}

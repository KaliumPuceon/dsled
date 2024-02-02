use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Block {

    operation: fn(VecDeque<Term>) -> Result<Term, String>,       // Function call for this operation
    children: VecDeque<Term>,            // Argterms of this term

}

#[derive(Clone, Debug)]
enum Term {

    Constant(f32),
    Var(String),
    Subterm(Block),

}

fn compute(mut block: Block) -> Term{

    let solved_children = block.children.iter_mut();
    solved_children.for_each(|term| {

        if let Term::Subterm(subterm_block) = term {

            *term = compute(subterm_block.clone());

        }

    });
    return (block.operation)(block.children).unwrap();

}

fn mul(mut terms: VecDeque<Term>) -> Result<Term,String> {
    
    if let Some(Term::Constant(mut acc)) = terms.pop_front() {
        while let Some(Term::Constant(x)) = terms.pop_front(){
            acc *= x;
        }
    return Ok(Term::Constant(acc))
    }
    Err("Multiply failed".to_string())
}

fn div(mut terms: VecDeque<Term>) -> Result<Term,String> {
    if let Some(Term::Constant(numerator)) = terms.pop_front() {
        if let Some(Term::Constant(denominator)) = terms.pop_front() {
            return Ok(Term::Constant(numerator/denominator))
        }
    }
    Err("Divide failed failed".to_string())
}

fn add(mut terms: VecDeque<Term>) -> Result<Term,String> {

    if let Some(Term::Constant(mut acc)) = terms.pop_front() {
        while let Some(Term::Constant(x)) = terms.pop_front(){
            acc += x;
        }
    return Ok(Term::Constant(acc))
    }
    Err("Add failed failed".to_string())

}

fn sub(mut terms: VecDeque<Term>) -> Result<Term,String> {

    if let Some(Term::Constant(mut acc)) = terms.pop_front() {
        while let Some(Term::Constant(x)) = terms.pop_front(){
            acc -= x;
        }
    return Ok(Term::Constant(acc))
    }
    Err("Sub failed".to_string())

}

fn sin(mut terms: VecDeque<Term>) -> Result<Term,String> {
    if let Some(Term::Constant(x)) = terms.pop_front() {
        return Ok(Term::Constant(x.sin()))
    }
    Err("Sine failed".to_string())
}

fn cos(mut terms: VecDeque<Term>) -> Result<Term,String> {
    if let Some(Term::Constant(x)) = terms.pop_front() {
        return Ok(Term::Constant(x.cos()))
    }
    Err("Cosine failed".to_string())
}

fn tan(mut terms: VecDeque<Term>) -> Result<Term,String> {
    if let Some(Term::Constant(x)) = terms.pop_front() {
        return Ok(Term::Constant(x.tan()))
    }
    Err("Tangent failed".to_string())
}

fn abs(mut terms: VecDeque<Term>) -> Result<Term,String> {
    if let Some(Term::Constant(x)) = terms.pop_front() {
        return Ok(Term::Constant(x.abs()))
    }
    Err("Abs failed".to_string())
}

const OPS:  [( &str, fn(VecDeque<Term>) -> Result<Term ,String> ); 8] = [
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
fn process_term(opstring: &mut VecDeque<&str>) -> Result<Block, String> {

    let mut before_term = true;
    let mut in_term = false;
    let mut end_of_term = false;
    let mut found_op = false;

    let mut result_op: Option<fn(VecDeque<Term>) -> Result<Term, String>> = None;
    let mut result_children: VecDeque<Term> = vec![].into();

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
                    result_children.push_back(Term::Subterm(subterm));
                }
            }

            else if let Some(matched_fun) = OPS.iter().find(|op| (op.0 == next)) {
                if !found_op {
                    println!("matched on a function {}", matched_fun.0);
                    result_op = Some(matched_fun.1);
                    found_op = true;
                }
                else {

                    return Err("Too many operators in term".to_string())

                }
            }
            else {

                if let Ok(num) = next.parse::<f32>() {

                    result_children.push_back(Term::Constant(num));
                    
                }

            }
            println!("{}",next);
        }
        else 
        {

            return Err("Badly matched parens somewhere".to_string())
        
        }

    }

    if let Some(_) = result_op{

        let result_block = Block {

            operation: result_op.unwrap(),
            children: result_children,

        };
        println!("end of term, returning up {:?}", result_block);
        return Ok(result_block);

    }
    else {
        return Err("Didn't work :(".to_string())
    }

}

fn parse(code: &str) -> Result<Block, String>  {

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

    if let Ok(output) = process_term(&mut sliced) {

        println!("calls: {:?}", output);
        return Ok(output);

    }
    

    Err("Not Working Yet".to_string())

}

fn main() {
   // let input = "( + 3 (* 2 4 ) )";
    let input = "( + 13 ( - 4 5 ) ( sin 0.3 ) ( cos ( * 2 0.56 ) ) )";
    println!("parsing {}", input);
    let computable_blocks = parse(input);
    println!("Parsing code {}", input);
    let output = compute(computable_blocks.unwrap());

    println!("{:?}", output);

}

use shunting::{ ShuntingParser, MathContext };
use std::{ collections::HashMap, env, fs, string::String, vec::Vec };
use regex::Regex;

/**
 * Read a file and return the contents as a string
 */
fn readfile(filename: &String) -> String {
    
    let contents = fs::read_to_string(filename)
        .expect("Error reading file");
    
    contents
}

/**
 * Evaluate a string as a mathematical expression.
 */
fn eval(input: String) -> f64 {

    let expr = ShuntingParser::parse_str(
        input.as_str()
    ).unwrap();

    let result = MathContext::new().eval(&expr).unwrap();

    result
}

/**
 * Evaluate the error in a solution to an equation.
 */
fn f_err(equation: &String, unknown: &String, x: f64) -> f64 {
    
    let x = x.to_string();
    let x = x.as_str();

    let equation = equation.replace(unknown, x);
    let exprs:Vec<&str> = equation.split("=").collect();
    
    let result = ( 
        eval(exprs[0].to_string()) - eval(exprs[1].to_string()) 
    ).abs();

    result
}

/**
 * Solves an equation for a single unknown
 */
fn solve_eq_1d(equation: String, unknown: &String) -> f64 {

    let mut x:f64 = -1e20;
    let mut dx:f64 = (-2.0 * x) / 4.0;

    while dx.abs() > 1e-20 { // while dx is non-negligible
        while f_err(&equation, &unknown, x) > f_err(&equation, &unknown, x + dx) {  // while error is decreasing
            x += &dx;
        }
        x += &dx;   // Go forward one step
        dx *= -0.5; // Reverse direction and increase resolution
    }
    dx *= -2.0;     // Backsolve for previous dx
    x += dx;        // Backsolve for median of possible x values

    x
} 

/**
 * Get all unknown variables in an expression or equation
 */
fn get_unknowns(text: &String) -> Vec<String> {
    // println!("{}", text);

    let search = Regex::new(r"(?i)[a-z_]+").unwrap();

    let result: Vec<&str> = search
        .find_iter(text.as_str())
        .map(|var| var.as_str())
        .collect();    

    let mut all_unknowns: Vec<String> = Vec::new();

    for i in &result {
        if (result.contains(&i)) 
        && (all_unknowns.contains(&i.to_string()) == false) {
            all_unknowns.push(i.to_string());
        }
    }

    all_unknowns
}

struct EqnParser {
    equation: String,
    eqn_vars: Vec<String>,
    lhs_vars: Vec<String>,
    rhs_vars: Vec<String>
}
impl EqnParser {

    pub fn new(mut equation: String, known_variables: &HashMap<String, f64>) -> EqnParser {

        // Replace all the known variables in the equation with their values
        for variable in known_variables.keys() {
            equation = equation.replace(
                variable, 
                known_variables[variable]
                    .to_string()
                    .as_str() 
            );
        }

        let exprs: Vec<&str> = equation.split("=").collect();

        let lhs = exprs[0].to_string();
        let rhs = exprs[1].to_string();

        EqnParser {
            eqn_vars: get_unknowns(&equation),
            equation: equation,
            lhs_vars: get_unknowns(&lhs),
            rhs_vars: get_unknowns(&rhs)
        }
    }

    /**
     * Get all unknown variables in an expression or equation
     */
    pub fn get_unknowns(&self) -> Vec<String> {

        let search = Regex::new(r"(?i)[a-z_]+").unwrap();

        let result: Vec<&str> = search
            .find_iter(self.equation.as_str())
            .map(|var| var.as_str())
            .collect();

        let mut all_unknowns: Vec<String> = Vec::new();

        for i in &result {
            if (result.contains(&i)) 
            && (all_unknowns.contains(&i.to_string()) == false) {
                all_unknowns.push(i.to_string());
            }
        }

        all_unknowns
    }

    /**
     * get all unknown variables in the equation
     */
    #[allow(dead_code)]
    pub fn get_variables(&self) -> Vec<String> {

        let search = Regex::new("(?i)[a-z_]+").unwrap();

        let result: Vec<String> = search
            .find_iter(self.equation.as_str())
            .map(|var| var.as_str().to_string())
            .collect();

        result
    }

    /**
     * Returns a bool indicating whether the given equation can be solved
     */
    pub fn is_solvable(&self) -> bool {

        let eqn = self.eqn_vars.len();
        let lhs = self.lhs_vars.len();
        let rhs = self.rhs_vars.len();

        // iff there is 1 unknown in the equation AND 
        // the left AND right sides have at most 1 unknown,

        let _1_unknown = eqn == 1 
            && rhs <= 1
            && lhs <= 1;

        _1_unknown
    }
}

fn main() {

    // gather cli args and system of equations
    let args: Vec<String> = env::args().collect();
    let text = readfile( &args[1] );
    
    // initialize important variables
    let mut known_variables: HashMap<String, f64> = HashMap::new();
    let mut prev_knowns: usize = 0;
    let mut starting = true;

    // "if the number of found values is increasing" OR "the solver is doing the first iteration"
    while known_variables.len() > prev_knowns || starting {
        for line in text.split("\n") {
            if line.contains("=") {
                let parser = EqnParser::new(
                    line.to_string(),
                    &known_variables
                );
    
                if parser.is_solvable() {
    
                    let unknown = &parser.get_unknowns()[0];
    
                    known_variables.insert(
                        unknown.to_string(),
                        solve_eq_1d(parser.equation, unknown)
                    );  
                }
            }   
        }
        starting = false; // solver is beyond the first iteration
        prev_knowns = known_variables.len();
    }

    println!(" _____ ___  ____   ___  _\n| ____/ _ \\/ ___| / _ \\| |\n|  _|| | | \\___ \\| | | | |\n| |__| |_| |___) | |_| | |___ \n|_____\\__\\_\\____/ \\___/|_____|\n\n Solution: \n+=========+");
    for kv_pair in known_variables {
        println!(
            "{} = {}", 
            kv_pair.0,
            kv_pair.1
        )
    }
}
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Operand {
    Wire(String),
    Val(u16),
}

impl Operand {
    fn parse(s: &str) -> Operand {
        if let Ok(v) = s.parse() {
            Operand::Val(v)
        } else {
            Operand::Wire(s.to_string())
        }
    }
}

#[derive(Clone, Debug)]
enum Expr {
    Val(u16),
    Wire(String),
    And(Operand, Operand),
    Or(Operand, Operand),
    Lshift(Operand, u16),
    Rshift(Operand, u16),
    Not(Operand),
}

fn parse(input: &str) -> HashMap<String, Expr> {
    let mut exprs = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let expr_str = parts[0];
        let wire = parts[1].to_string();

        let expr = if expr_str.contains("AND") {
            let ops: Vec<&str> = expr_str.split(" AND ").collect();
            Expr::And(Operand::parse(ops[0]), Operand::parse(ops[1]))
        } else if expr_str.contains("OR") {
            let ops: Vec<&str> = expr_str.split(" OR ").collect();
            Expr::Or(Operand::parse(ops[0]), Operand::parse(ops[1]))
        } else if expr_str.contains("LSHIFT") {
            let ops: Vec<&str> = expr_str.split(" LSHIFT ").collect();
            Expr::Lshift(Operand::parse(ops[0]), ops[1].parse().unwrap())
        } else if expr_str.contains("RSHIFT") {
            let ops: Vec<&str> = expr_str.split(" RSHIFT ").collect();
            Expr::Rshift(Operand::parse(ops[0]), ops[1].parse().unwrap())
        } else if expr_str.starts_with("NOT ") {
            Expr::Not(Operand::parse(&expr_str[4..]))
        } else if let Ok(value) = expr_str.parse() {
            Expr::Val(value)
        } else {
            Expr::Wire(expr_str.to_string())
        };

        exprs.insert(wire, expr);
    }

    exprs
}

fn eval_operand(
    op: &Operand,
    exprs: &HashMap<String, Expr>,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    match op {
        Operand::Val(v) => *v,
        Operand::Wire(name) => simulate(exprs, name, cache),
    }
}

fn simulate(exprs: &HashMap<String, Expr>, wire: &str, cache: &mut HashMap<String, u16>) -> u16 {
    if let Some(&value) = cache.get(wire) {
        return value;
    }

    let value = match &exprs[wire] {
        Expr::Val(v) => *v,
        Expr::Wire(w) => simulate(exprs, w, cache),

        Expr::And(a, b) => eval_operand(a, exprs, cache) & eval_operand(b, exprs, cache),
        Expr::Or(a, b) => eval_operand(a, exprs, cache) | eval_operand(b, exprs, cache),

        Expr::Lshift(a, n) => eval_operand(a, exprs, cache) << n,
        Expr::Rshift(a, n) => eval_operand(a, exprs, cache) >> n,

        Expr::Not(a) => !eval_operand(a, exprs, cache),
    };

    cache.insert(wire.to_string(), value);
    value
}

pub fn solve(input: &str) -> (String, String) {
    let exprs = parse(input);

    let mut cache = HashMap::new();
    let part1 = simulate(&exprs, "a", &mut cache);

    let mut modified_exprs = exprs.clone();
    modified_exprs.insert("b".to_string(), Expr::Val(part1));

    let mut cache_part2 = HashMap::new();
    let part2 = simulate(&modified_exprs, "a", &mut cache_part2);

    (part1.to_string(), part2.to_string())
}

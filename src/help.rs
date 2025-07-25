use std::collections::HashMap;

pub fn get_help_categories() -> HashMap<&'static str, Vec<&'static str>>{
    let mut help = HashMap::new();

    help.insert("Arithmetic", vec!["+", "-", "*", "/", "pow", "abs", "sqrt", "round", "ceil","floor","sign", "float", "exp"]);
    help.insert("Functions", vec![
        "log", "log10", "log2", "ln", 
        "sin", "cos", "tan", "asin", "acos", "atan", "atan2", "hypot",
        "max", "min", "range", "map", "linspace", "sort", "length"
    ]);
    help.insert("Comparison", vec!["<", ">", "<=", ">=", "==", "!="]);
    help.insert("Boolean", vec!["and", "or", "not"]);
    help.insert("Constants", vec!["pi", "e", "inf", "nan"]);

    help

}

pub fn get_function_details(name: &str) -> Option<&'static str>{
    match name{
        "+" => Some("+: number + number -> number"),
        "-" => Some("-: number - number -> number"),
        "*" => Some("*: number * number -> number"),
        "/" => Some("/: number / number -> number"),

        "pow" => Some("pow(base: number, exponent: number) -> number"),
        "abs" => Some("abs(x: number) -> number"),
        "sqrt" => Some("sqrt(x: number) -> number"),
        "round" => Some("round(x: number) -> number"),
        "ceil" => Some("ceil(x: number) -> number"),
        "floor" => Some("floor(x: number) -> number"),
        "sign" => Some("sign(x: number) -> number (-1, 0, 1)"),
        "float" => Some("float(x: number) -> number"),
        "exp" => Some("exp(x: number) -> number (e^x)"),

         // Logarithmic and Trigonometric
        "log" => Some("log(base: number, value: number) -> number"),
        "log10" => Some("log10(x: number) -> number"),
        "log2" => Some("log2(x: number) -> number"),
        "ln" => Some("ln(x: number) -> number"),

        "sin" => Some("sin(x: radians) -> number"),
        "cos" => Some("cos(x: radians) -> number"),
        "tan" => Some("tan(x: radians) -> number"),
        "asin" => Some("asin(x: number) -> radians"),
        "acos" => Some("acos(x: number) -> radians"),
        "atan" => Some("atan(x: number) -> radians"),
        "atan2" => Some("atan2(y: number, x: number) -> radians"),
        "hypot" => Some("hypot(x: number, y: number) -> number"),

        // Utility functions
        "max" => Some("max(a: number, b: number, ...) or max([a, b, ...]) -> number"),
        "min" => Some("min(a: number, b: number, ...) or min([a, b, ...]) -> number"),
        "range" => Some("range(start: number, end: number) or range(end: number) -> [number]"),
        "map" => Some("map(f: function, list: [value]) -> [value]"),
        "linspace" => Some("linspace(start: number, end: number, count: number) -> [number]"),
        "sort" => Some("sort(list: [value]) -> [value]"),
        "length" => Some("length(list: [value]) -> number"),

        // Comparison
        "<" => Some("<: a < b -> bool"),
        ">" => Some(">: a > b -> bool"),
        "<=" => Some("<=: a <= b -> bool"),
        ">=" => Some(">=: a >= b -> bool"),
        "==" => Some("<: a == b -> bool"),
        "!=" => Some("<: a != b -> bool"),

        // Boolean
        "and" => Some("and(a: bool, b: bool) -> bool"),
        "or" => Some("or(a: bool, b: bool) -> bool"),
        "not" => Some("not(a: bool) -> bool"),

        // Constants
        "pi" => Some("Constant: π = 3.141592..."),
        "e" => Some("Constant: e = 2.71828..."),
        "inf" => Some("Constant: Positive infinity"),
        "nan" => Some("Constant: Not-a-Number"),

        // "and" | "&&" => Some("and(a: bool, b: bool) → bool"),
        // "or"  | "||" => Some("or(a: bool, b: bool) → bool"),
        // "not" | "!"  => Some("not(a: bool) → bool"),
        _ => None,

    }
}
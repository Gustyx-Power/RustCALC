use super::parser::{BinaryOp, Expr, PostfixOp, UnaryOp};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleMode {
    Deg,
    Rad,
}

impl AngleMode {
    pub fn label(&self) -> &'static str {
        match self {
            AngleMode::Deg => "DEG",
            AngleMode::Rad => "RAD",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            AngleMode::Deg => AngleMode::Rad,
            AngleMode::Rad => AngleMode::Deg,
        }
    }
}

pub struct EvalContext {
    pub angle_mode: AngleMode,
    pub ans: f64,
    pub memory: f64,
    pub custom_vars: HashMap<String, f64>,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self {
            angle_mode: AngleMode::Deg,
            ans: 0.0,
            memory: 0.0,
            custom_vars: HashMap::new(),
        }
    }
}

impl EvalContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn resolve_var(&self, name: &str) -> Option<f64> {
        match name.to_lowercase().as_str() {
            "pi" => Some(std::f64::consts::PI),
            "e" => Some(std::f64::consts::E),
            "phi" => Some(1.618_033_988_749_895),
            "tau" => Some(std::f64::consts::TAU),
            "ans" => Some(self.ans),
            "c" => Some(299_792_458.0),       // m/s
            "h" => Some(6.626_070_15e-34),    // J*s
            "g" => Some(9.80665),             // m/s^2
            "na" => Some(6.022_140_76e23),    // 1/mol
            "k" => Some(1.380_649e-23),       // J/K
            other => self.custom_vars.get(other).copied(),
        }
    }
}

pub fn evaluate(expr: &Expr, ctx: &EvalContext) -> Result<f64, String> {
    let raw = eval_internal(expr, ctx)?;
    Ok(clean_float(raw))
}

fn eval_internal(expr: &Expr, ctx: &EvalContext) -> Result<f64, String> {
    match expr {
        Expr::Number(val) => Ok(*val),
        Expr::Variable(name) => {
            ctx.resolve_var(name)
                .ok_or_else(|| format!("Unknown identifier '{}'", name))
        }
        Expr::UnaryOp { op, expr } => {
            let val = eval_internal(expr, ctx)?;
            match op {
                UnaryOp::Pos => Ok(val),
                UnaryOp::Neg => Ok(-val),
            }
        }
        Expr::BinaryOp { op, left, right } => {
            let l = eval_internal(left, ctx)?;
            let r = eval_internal(right, ctx)?;
            match op {
                BinaryOp::Add => Ok(l + r),
                BinaryOp::Sub => Ok(l - r),
                BinaryOp::Mul => Ok(l * r),
                BinaryOp::Div => {
                    if r.abs() < f64::EPSILON {
                        Err("Division by zero (undefined)".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                BinaryOp::Mod => {
                    if r.abs() < f64::EPSILON {
                        Err("Modulo by zero (undefined)".to_string())
                    } else {
                        Ok(l % r)
                    }
                }
                BinaryOp::Pow => {
                    let res = l.powf(r);
                    if res.is_nan() {
                        Err("Invalid domain for exponentiation".to_string())
                    } else {
                        Ok(res)
                    }
                }
            }
        }
        Expr::PostfixOp { op, expr } => {
            let val = eval_internal(expr, ctx)?;
            match op {
                PostfixOp::Percent => Ok(val / 100.0),
                PostfixOp::Factorial => compute_factorial(val),
            }
        }
        Expr::FunctionCall { name, args } => eval_function(name, args, ctx),
    }
}

fn eval_function(name: &str, args: &[Expr], ctx: &EvalContext) -> Result<f64, String> {
    let lower_name = name.to_lowercase();
    let evaluated_args: Vec<f64> = args
        .iter()
        .map(|a| eval_internal(a, ctx))
        .collect::<Result<Vec<f64>, String>>()?;

    let unary_arg = || -> Result<f64, String> {
        if evaluated_args.len() != 1 {
            Err(format!("Function '{}' expects exactly 1 argument, got {}", lower_name, evaluated_args.len()))
        } else {
            Ok(evaluated_args[0])
        }
    };

    match lower_name.as_str() {
        "sin" => {
            let x = unary_arg()?;
            let rad = match ctx.angle_mode {
                AngleMode::Deg => x.to_radians(),
                AngleMode::Rad => x,
            };
            Ok(clean_float(rad.sin()))
        }
        "cos" => {
            let x = unary_arg()?;
            let rad = match ctx.angle_mode {
                AngleMode::Deg => x.to_radians(),
                AngleMode::Rad => x,
            };
            Ok(clean_float(rad.cos()))
        }
        "tan" => {
            let x = unary_arg()?;
            let rad = match ctx.angle_mode {
                AngleMode::Deg => {
                    // Check undefined points: 90, 270, etc.
                    let rem = (x.abs() - 90.0) % 180.0;
                    if rem.abs() < 1e-9 {
                        return Err("tan is undefined at odd multiples of 90°".to_string());
                    }
                    x.to_radians()
                }
                AngleMode::Rad => {
                    let half_pi = std::f64::consts::FRAC_PI_2;
                    let rem = (x.abs() - half_pi) % std::f64::consts::PI;
                    if rem.abs() < 1e-9 {
                        return Err("tan is undefined at odd multiples of π/2".to_string());
                    }
                    x
                }
            };
            Ok(clean_float(rad.tan()))
        }
        "asin" => {
            let x = unary_arg()?;
            if !(-1.0..=1.0).contains(&x) {
                return Err(format!("Domain error: asin argument must be between -1 and 1, got {}", x));
            }
            let rad = x.asin();
            match ctx.angle_mode {
                AngleMode::Deg => Ok(clean_float(rad.to_degrees())),
                AngleMode::Rad => Ok(clean_float(rad)),
            }
        }
        "acos" => {
            let x = unary_arg()?;
            if !(-1.0..=1.0).contains(&x) {
                return Err(format!("Domain error: acos argument must be between -1 and 1, got {}", x));
            }
            let rad = x.acos();
            match ctx.angle_mode {
                AngleMode::Deg => Ok(clean_float(rad.to_degrees())),
                AngleMode::Rad => Ok(clean_float(rad)),
            }
        }
        "atan" => {
            let x = unary_arg()?;
            let rad = x.atan();
            match ctx.angle_mode {
                AngleMode::Deg => Ok(clean_float(rad.to_degrees())),
                AngleMode::Rad => Ok(clean_float(rad)),
            }
        }
        "atan2" => {
            if evaluated_args.len() != 2 {
                return Err(format!("atan2 expects 2 arguments (y, x), got {}", evaluated_args.len()));
            }
            let y = evaluated_args[0];
            let x = evaluated_args[1];
            let rad = y.atan2(x);
            match ctx.angle_mode {
                AngleMode::Deg => Ok(clean_float(rad.to_degrees())),
                AngleMode::Rad => Ok(clean_float(rad)),
            }
        }
        "sinh" => Ok(unary_arg()?.sinh()),
        "cosh" => Ok(unary_arg()?.cosh()),
        "tanh" => Ok(unary_arg()?.tanh()),
        "asinh" => Ok(unary_arg()?.asinh()),
        "acosh" => {
            let x = unary_arg()?;
            if x < 1.0 {
                return Err("Domain error: acosh argument must be >= 1".to_string());
            }
            Ok(x.acosh())
        }
        "atanh" => {
            let x = unary_arg()?;
            if x <= -1.0 || x >= 1.0 {
                return Err("Domain error: atanh argument must be between -1 and 1".to_string());
            }
            Ok(x.atanh())
        }
        "sqrt" => {
            let x = unary_arg()?;
            if x < 0.0 {
                return Err("Domain error: sqrt of negative number".to_string());
            }
            Ok(x.sqrt())
        }
        "cbrt" => Ok(unary_arg()?.cbrt()),
        "ln" => {
            let x = unary_arg()?;
            if x <= 0.0 {
                return Err("Domain error: ln requires argument > 0".to_string());
            }
            Ok(x.ln())
        }
        "log" | "log10" => {
            let x = unary_arg()?;
            if x <= 0.0 {
                return Err("Domain error: log requires argument > 0".to_string());
            }
            Ok(x.log10())
        }
        "log2" => {
            let x = unary_arg()?;
            if x <= 0.0 {
                return Err("Domain error: log2 requires argument > 0".to_string());
            }
            Ok(x.log2())
        }
        "exp" => Ok(unary_arg()?.exp()),
        "abs" => Ok(unary_arg()?.abs()),
        "floor" => Ok(unary_arg()?.floor()),
        "ceil" => Ok(unary_arg()?.ceil()),
        "round" => Ok(unary_arg()?.round()),
        "fact" | "factorial" => compute_factorial(unary_arg()?),
        other => Err(format!("Unknown function '{}'", other)),
    }
}

fn compute_factorial(val: f64) -> Result<f64, String> {
    if val < 0.0 {
        return Err("Factorial of negative number is undefined".to_string());
    }
    let rounded = val.round();
    if (val - rounded).abs() > 1e-9 {
        return Err("Factorial is only defined for non-negative integers".to_string());
    }
    let n = rounded as u64;
    if n > 170 {
        return Err("Factorial overflow (> 170!)".to_string());
    }
    let mut acc = 1.0f64;
    for i in 2..=n {
        acc *= i as f64;
    }
    Ok(acc)
}

pub fn clean_float(val: f64) -> f64 {
    if val.is_nan() || val.is_infinite() {
        return val;
    }
    if val.abs() < 1e-12 {
        return 0.0;
    }
    // Snap near integers (e.g. 30.000000000000004 -> 30.0)
    let round_val = val.round();
    if (val - round_val).abs() < 1e-10 {
        return round_val;
    }
    // Snap floating point jitter up to 10 decimal digits (e.g. 0.49999999999999994 -> 0.5)
    let factor = 1e10;
    let rounded = (val * factor).round() / factor;
    if (val - rounded).abs() < 1e-11 {
        return rounded;
    }
    val
}

pub fn format_result(val: f64) -> String {
    if val.is_nan() {
        return "NaN".to_string();
    }
    if val.is_infinite() {
        if val > 0.0 {
            return "∞".to_string();
        } else {
            return "-∞".to_string();
        }
    }

    let cleaned = clean_float(val);
    let abs = cleaned.abs();

    if abs > 0.0 && (abs >= 1e12 || abs < 1e-6) {
        format!("{:e}", cleaned)
    } else {
        // Format with up to 10 decimal digits and strip trailing zeros
        let mut s = format!("{:.10}", cleaned);
        if s.contains('.') {
            while s.ends_with('0') {
                s.pop();
            }
            if s.ends_with('.') {
                s.pop();
            }
        }
        format_with_commas(&s)
    }
}

pub fn format_with_commas(s: &str) -> String {
    let (is_neg, s_abs) = if let Some(stripped) = s.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, s)
    };

    let parts: Vec<&str> = s_abs.split('.').collect();
    let int_part = parts[0];
    let frac_part = parts.get(1);

    let mut formatted_int = String::new();
    let chars: Vec<char> = int_part.chars().collect();
    let len = chars.len();

    for (i, &ch) in chars.iter().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            formatted_int.push(',');
        }
        formatted_int.push(ch);
    }

    let mut res = if is_neg { format!("-{}", formatted_int) } else { formatted_int };
    if let Some(frac) = frac_part {
        res.push('.');
        res.push_str(frac);
    }
    res
}

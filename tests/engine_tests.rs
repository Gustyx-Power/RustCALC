use rustcalc::engine::{calculate, format_result, AngleMode, EvalContext};

#[test]
fn test_basic_arithmetic() {
    let ctx = EvalContext::default();
    assert_eq!(calculate("2 + 3", &ctx).unwrap(), 5.0);
    assert_eq!(calculate("10 - 4", &ctx).unwrap(), 6.0);
    assert_eq!(calculate("3 * 7", &ctx).unwrap(), 21.0);
    assert_eq!(calculate("20 / 4", &ctx).unwrap(), 5.0);
    assert_eq!(calculate("2 + 3 * 4", &ctx).unwrap(), 14.0);
    assert_eq!(calculate("(2 + 3) * 4", &ctx).unwrap(), 20.0);
    assert_eq!(calculate("1,000 + 2,500.5", &ctx).unwrap(), 3500.5);
}

#[test]
fn test_powers_and_roots() {
    let ctx = EvalContext::default();
    assert_eq!(calculate("2^3", &ctx).unwrap(), 8.0);
    // Right-associative: 2^(3^2) = 2^9 = 512
    assert_eq!(calculate("2^3^2", &ctx).unwrap(), 512.0);
    assert_eq!(calculate("sqrt(144)", &ctx).unwrap(), 12.0);
    assert_eq!(calculate("cbrt(27)", &ctx).unwrap(), 3.0);
}

#[test]
fn test_trigonometry_deg_and_rad() {
    let mut ctx_deg = EvalContext::default();
    ctx_deg.angle_mode = AngleMode::Deg;

    assert_eq!(calculate("sin(30)", &ctx_deg).unwrap(), 0.5);
    assert_eq!(calculate("cos(60)", &ctx_deg).unwrap(), 0.5);
    assert_eq!(calculate("tan(45)", &ctx_deg).unwrap(), 1.0);
    assert_eq!(calculate("asin(0.5)", &ctx_deg).unwrap(), 30.0);
    assert_eq!(calculate("acos(0.5)", &ctx_deg).unwrap(), 60.0);
    assert_eq!(calculate("atan(1)", &ctx_deg).unwrap(), 45.0);

    let mut ctx_rad = EvalContext::default();
    ctx_rad.angle_mode = AngleMode::Rad;
    assert_eq!(calculate("sin(pi / 6)", &ctx_rad).unwrap(), 0.5);
    assert_eq!(calculate("cos(pi / 3)", &ctx_rad).unwrap(), 0.5);
}

#[test]
fn test_factorial_and_percent() {
    let ctx = EvalContext::default();
    assert_eq!(calculate("5!", &ctx).unwrap(), 120.0);
    assert_eq!(calculate("0!", &ctx).unwrap(), 1.0);
    assert_eq!(calculate("3! + 4!", &ctx).unwrap(), 30.0);
    assert_eq!(calculate("50%", &ctx).unwrap(), 0.5);
    assert_eq!(calculate("200 * 15%", &ctx).unwrap(), 30.0);
}

#[test]
fn test_implicit_multiplication() {
    let ctx = EvalContext::default();
    assert_eq!(calculate("2pi", &ctx).unwrap(), 2.0 * std::f64::consts::PI);
    assert_eq!(calculate("3(4 + 2)", &ctx).unwrap(), 18.0);
    assert_eq!(calculate("(2 + 3)(4 + 1)", &ctx).unwrap(), 25.0);
}

#[test]
fn test_error_handling() {
    let ctx = EvalContext::default();
    assert!(calculate("1 / 0", &ctx).is_err());
    assert!(calculate("sqrt(-4)", &ctx).is_err());
    assert!(calculate("asin(2)", &ctx).is_err());
    assert!(calculate("((2 + 3)", &ctx).is_err());
}

#[test]
fn test_format_result() {
    assert_eq!(format_result(1234.5), "1,234.5");
    assert_eq!(format_result(0.0), "0");
    assert_eq!(format_result(120.0), "120");
    assert_eq!(format_result(1000000.0), "1,000,000");
    assert_eq!(format_result(1.0 / 0.0), "∞");
}

#[test]
fn test_scientific_advanced_formulas() {
    let mut ctx = EvalContext::default();
    ctx.angle_mode = AngleMode::Deg;

    // Trig identity: sin(30) + cos(60) = 1.0
    assert_eq!(calculate("sin(30) + cos(60)", &ctx).unwrap(), 1.0);

    // Golden ratio identity: phi^2 - phi = 1.0
    let phi_res = calculate("phi^2 - phi", &ctx).unwrap();
    assert!((phi_res - 1.0).abs() < 1e-10);

    // Speed of light in thousands of km/s
    assert_eq!(calculate("c / 1000", &ctx).unwrap(), 299792.458);

    // Hyperbolic identity: cosh(0) = 1, sinh(0) = 0
    assert_eq!(calculate("cosh(0) + sinh(0)", &ctx).unwrap(), 1.0);

    // Logarithm identities
    assert_eq!(calculate("ln(e^3)", &ctx).unwrap(), 3.0);
    assert_eq!(calculate("log(10000)", &ctx).unwrap(), 4.0);
    assert_eq!(calculate("log2(1024)", &ctx).unwrap(), 10.0);

    // Factorial permutations: 6! / (3! * 2!) = 720 / 12 = 60
    assert_eq!(calculate("6! / (3! * 2!)", &ctx).unwrap(), 60.0);

    // Percentage calculation: 250 + 20%
    assert_eq!(calculate("250 * 20%", &ctx).unwrap(), 50.0);
}

#[test]
fn test_paste_and_copy_state_transitions() {
    use rustcalc::app::CalculatorApp;
    use rustcalc::ui::keypad::KeyAction;

    let mut app = CalculatorApp::default();
    assert!(!app.has_copied);

    // Paste initial formula
    app.paste_text("sin(30) + cos(60)");
    assert_eq!(app.expression, "sin(30) + cos(60)");
    assert_eq!(app.result, "1");

    // Simulate evaluate
    app.execute_action(KeyAction::Evaluate);
    assert!(app.just_evaluated);
    assert_eq!(app.result, "1");

    // Copy result manually by setting state
    app.last_copied = app.result.clone();
    app.has_copied = true;
    assert!(app.has_copied);

    // Pasting an operator after evaluation appends to previous ans
    app.paste_text("+ 4");
    assert_eq!(app.expression, "1 + 4");
    assert_eq!(app.result, "5");

    // Clear resets copied state
    app.has_copied = true;
    app.execute_action(KeyAction::Clear);
    assert!(!app.has_copied);
    assert_eq!(app.expression, "");
    assert_eq!(app.result, "0");
}

#[test]
fn test_high_volume_stress_calculation() {
    let ctx = EvalContext::default();
    for i in 1..=500 {
        let expr = format!("{i} * 2 + sqrt(16) - 4");
        let val = calculate(&expr, &ctx).unwrap();
        assert_eq!(val, (i * 2) as f64);
    }
}



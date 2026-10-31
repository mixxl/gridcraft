use gridcraft_formula::{Expr, FormulaLocale, parse, parse_input, print, print_input};

fn input(src: &str) -> String {
    print(&parse_input(src, FormulaLocale::Es, |_| false).unwrap())
}

#[test]
fn spanish_aliases_and_argument_separators() {
    for (src, expected) in [
        ("=suma(A1:A10)", "SUM(A1:A10)"),
        ("PROMEDIO(A1:A10)", "AVERAGE(A1:A10)"),
        ("SI(A1>0;\"Sí\";\"No\")", "IF(A1>0,\"Sí\",\"No\")"),
        ("CONTAR(A1:A10)", "COUNT(A1:A10)"),
        ("SUMAR.SI(A1:A10;\">0\")", "SUMIF(A1:A10,\">0\")"),
        ("CONTAR.SI(A1:A10;\">0\")", "COUNTIF(A1:A10,\">0\")"),
        ("BUSCARV(A2;D:E;2;FALSO)", "VLOOKUP(A2,D:E,2,FALSE)"),
        ("HOY()", "TODAY()"),
        ("VNA(0,1;A1:A5)", "NPV(0.1,A1:A5)"),
        ("TIR(A1:A5;0,1)", "IRR(A1:A5,0.1)"),
        ("PAGO(0,08/12;10;10000)", "PMT(0.08/12,10,10000)"),
        ("VA(0,05;12;100)", "PV(0.05,12,100)"),
        ("VF(0,05;12;100)", "FV(0.05,12,100)"),
        ("=verdadero", "TRUE"),
        ("=falso", "FALSE"),
        ("VERDADERO()", "TRUE()"),
        ("FALSO()", "FALSE()"),
        ("SUMA(1;SUM(2;3);PROMEDIO(4;6))", "SUM(1,SUM(2,3),AVERAGE(4,6))"),
        ("SI(VERDADERO;;)", "IF(TRUE,,)"),
        ("LAMBDA(x;x+1)(2)", "LAMBDA(x,x+1)(2)"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
}

#[test]
fn spanish_input_preserves_reference_and_array_syntax() {
    for (src, expected) in [
        ("SUMA({1\\2;3\\4};5)", "SUM({1,2;3,4},5)"),
        ("{VERDADERO\\FALSO;TRUE\\FALSE}", "{TRUE,FALSE;TRUE,FALSE}"),
        ("SUM((A1:A3;C1:C3))", "SUM((A1:A3,C1:C3))"),
        ("\"SUMA;FALSO\"&\"a\"\"b\"", "\"SUMA;FALSO\"&\"a\"\"b\""),
        ("'SUMA;FALSO'!A1+SUMA!B2+SUMA!FALSO", "'SUMA;FALSO'!A1+SUMA!B2+SUMA!FALSO"),
        ("SUMA[CONTAR.SI]+FALSO[SI]+CONTAR", "SUMA[CONTAR.SI]+FALSO[SI]+CONTAR"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
    for bad in [r"SUMA({1;2\3})", "{SUMA(A1)}", "{FALSO;}"] {
        assert!(parse_input(bad, FormulaLocale::Es, |_| false).is_err(), "{bad}");
    }
    assert_eq!(print(&parse("SUMA(1,2)+FALSO").unwrap()), "SUMA(1,2)+FALSO");
    assert!(parse("SUM(1;2)").is_err());
    assert!(parse("{VERDADERO}").is_err());
    // Real nesting past the parser's limit is still refused.
    let nested = "(".repeat(300) + "1" + &")".repeat(300);
    assert!(parse_input(&nested, FormulaLocale::Es, |_| false).is_err());
    let deep = (0..300).fold(Expr::Number(1.0), |e, _| Expr::Paren(Box::new(e)));
    assert!(print_input(&deep, FormulaLocale::Es, |_| false).is_err());
}

#[test]
fn spanish_input_accepts_flat_chains_as_long_as_excel() {
    // A flat chain nests on its left without nesting in the text. Excel accepts 4,096
    // operands (8,192 characters); Spanish entry refused more than 256 (#916). On a 1 MiB
    // stack, as on the Windows main thread.
    std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            let long_chain = "FALSO+".repeat(1360) + "VERDADERO";
            let english = "FALSE+".repeat(1360) + "TRUE";
            assert_eq!(input(&long_chain), english);
            assert_eq!(print_input(&parse(&english).unwrap(), FormulaLocale::Es, |_| false).unwrap(), long_chain);
            let sums = vec!["SUMA(1;2)"; 800].join("+");
            assert_eq!(input(&sums), vec!["SUM(1,2)"; 800].join("+"));
        })
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn spanish_input_preserves_defined_and_local_names() {
    let known = |name: &str| ["SUMA", "FALSO"].iter().any(|n| n.eq_ignore_ascii_case(name));
    let expr = parse_input("SUMA(2;3)+FALSO+SI(VERDADERO;4;5)", FormulaLocale::Es, known).unwrap();
    assert_eq!(print(&expr), "SUMA(2,3)+FALSO+IF(TRUE,4,5)");
    for (src, expected) in [
        ("LET(FALSO;7;FALSO)", "LET(FALSO,7,FALSO)"),
        ("LET(falso;FALSO;SI(falso;1;2))", "LET(falso,FALSE,IF(falso,1,2))"),
        ("LET(SUMA;LAMBDA(x;x+1);SUMA(2))", "LET(SUMA,LAMBDA(x,x+1),SUMA(2))"),
        ("LET(falso;7;valor;FALSO;valor)", "LET(falso,7,valor,FALSO,valor)"),
        ("LAMBDA(FALSO;FALSO)(7)", "LAMBDA(FALSO,FALSO)(7)"),
        ("LAMBDA(SUMA;SUMA(2))(LAMBDA(x;x+1))", "LAMBDA(SUMA,SUMA(2))(LAMBDA(x,x+1))"),
        ("LET(FALSO;7;FALSO)+FALSO", "LET(FALSO,7,FALSO)+FALSE"),
        ("LET(FALSO;7;LAMBDA(x;FALSO+x)(1))", "LET(FALSO,7,LAMBDA(x,FALSO+x)(1))"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
}

#[test]
fn locale_controls_decimal_list_array_separators_and_display() {
    let es = FormulaLocale::Es;
    let en = FormulaLocale::En;
    let expr = parse_input("SI(VERDADERO;SUMA(1,5;2,25);0)", es, |_| false).unwrap();
    assert_eq!(print(&expr), "IF(TRUE,SUM(1.5,2.25),0)");
    assert_eq!(print_input(&expr, es, |_| false).unwrap(), "SI(VERDADERO;SUMA(1,5;2,25);0)");
    assert_eq!(print_input(&expr, en, |_| false).unwrap(), print(&expr));
    assert!(parse_input("SUM(1;2)", en, |_| false).is_err());
    assert_eq!(print(&parse_input("SUMA(1,2)+FALSO", en, |_| false).unwrap()), "SUMA(1,2)+FALSO");
    assert!(parse_input("SUMA(1.5;2)", es, |_| false).is_err());
    let expr = parse("SUM({1.5,2.25;3,4})+SUM((A1:A3,C1:C3))").unwrap();
    let shown = print_input(&expr, es, |_| false).unwrap();
    assert_eq!(print(&parse_input(&shown, es, |_| false).unwrap()), print(&expr));
    let expr = parse("LET(SUMA,LAMBDA(x,x+1),SUM(SUMA(2),FALSE))").unwrap();
    let shown = print_input(&expr, es, |n| n == "FALSO").unwrap();
    assert_eq!(shown, "LET(SUMA;LAMBDA(x;x+1);SUM(SUMA(2);FALSE))");
    assert_eq!(print(&parse_input(&shown, es, |n| n == "FALSO").unwrap()), print(&expr));
}

#[test]
fn spanish_display_round_trips_text_containing_separators() {
    let es = FormulaLocale::Es;
    let expr = parse(r#"IF(A1="a,b;c",CONCAT("1.5",",",";"),SUM(0.25,A1:A3 B2))"#).unwrap();
    let shown = print_input(&expr, es, |_| false).unwrap();
    assert_eq!(shown, r#"SI(A1="a,b;c";CONCAT("1.5";",";";");SUMA(0,25;A1:A3 B2))"#);
    assert_eq!(print(&parse_input(&shown, es, |_| false).unwrap()), print(&expr));
}

#[test]
fn local_names_match_aliases_in_any_case_like_the_evaluator() {
    // `ſi` (long s) is `SI` case-insensitively, so the call is the LAMBDA, not IF.
    assert_eq!(input("LET(ſi;LAMBDA(x;x*2);SI(3))"), "LET(ſi,LAMBDA(x,x*2),SI(3))");
    assert!(gridcraft_formula::same_name("ß", "ẞ"));
    assert!(gridcraft_formula::same_name("dług", "DŁUG"));
    assert!(gridcraft_formula::same_name("ſi", "SI"));
    assert!(gridcraft_formula::same_name("σοφός", "ΣΟΦΌΣ"));
    // Dotless ı is its own letter outside Turkic languages.
    assert!(!gridcraft_formula::same_name("ı", "i"));
    assert!(!gridcraft_formula::same_name("ı", "I"));
    assert!(!gridcraft_formula::same_name("a", "b"));
}

#[test]
fn spanish_names_cover_the_everyday_functions() {
    for (src, expected) in [
        ("BUSCARX(A1;B:B;C:C)", "XLOOKUP(A1,B:B,C:C)"),
        ("CONTAR.SI.CONJUNTO(A:A;\"x\";B:B;\">0\")", "COUNTIFS(A:A,\"x\",B:B,\">0\")"),
        ("SUMAR.SI.CONJUNTO(C:C;A:A;\"x\")", "SUMIFS(C:C,A:A,\"x\")"),
        ("SI.ERROR(INDICE(B:B;COINCIDIR(A1;A:A;0));\"\")", "IFERROR(INDEX(B:B,MATCH(A1,A:A,0)),\"\")"),
        ("REDONDEAR(A1*1,21;2)", "ROUND(A1*1.21,2)"),
        ("Y(A1>0;O(B1;NO(C1)))", "AND(A1>0,OR(B1,NOT(C1)))"),
        ("CONCATENAR(IZQUIERDA(A1;3);EXTRAE(A1;4;2);LARGO(A1))", "CONCATENATE(LEFT(A1,3),MID(A1,4,2),LEN(A1))"),
        ("año(HOY())+MES(AHORA())", "YEAR(TODAY())+MONTH(NOW())"),
        ("FRAC.AÑO(A1;B1)", "YEARFRAC(A1,B1)"),
        ("DIAS.LAB(A1;B1)", "NETWORKDAYS(A1,B1)"),
        ("M.C.D(12;18)", "GCD(12,18)"),
        ("DECIMAL(A1;2)+CONV.DECIMAL(\"FF\";16)", "FIXED(A1,2)+DECIMAL(\"FF\",16)"),
        ("ABS(-1)+MAX(1;2)", "ABS(-1)+MAX(1,2)"),
    ] {
        assert_eq!(input(src), expected, "{src}");
    }
    for (english, spanish) in
        [("=XLOOKUP(A1,B:B,C:C)", "BUSCARX"), ("=YEAR(A1)", "AÑO"), ("=FIXED(A1,2)", "DECIMAL"), ("=DECIMAL(\"FF\",16)", "CONV.DECIMAL")]
    {
        let shown = print_input(&parse(english).unwrap(), FormulaLocale::Es, |_| false).unwrap();
        assert!(shown.contains(&format!("{spanish}(")), "{english} => {shown}");
        assert_eq!(print(&parse_input(&shown, FormulaLocale::Es, |_| false).unwrap()), english.trim_start_matches('='), "{shown}");
    }
}

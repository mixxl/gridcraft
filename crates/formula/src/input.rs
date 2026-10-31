//! Localized aliases accepted at user-entry boundaries; file syntax stays English.

use crate::{Expr, ParseError, parser};

/// Formula-entry syntax selected by the interface language. Other interface languages
/// currently use the canonical English function table until a translation table is supplied.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormulaLocale {
    #[default]
    En,
    Es,
}

/// Excel's Spanish (es-ES) function names: canonical English, then Spanish. Both columns are
/// unique, so a name maps the same way in both directions; functions not listed keep their
/// English name. `FIXED` is `DECIMAL` in Spanish, and `DECIMAL` is `CONV.DECIMAL`.
const SPANISH: &[(&str, &str)] = &[
    ("ASIN", "ASENO"),
    ("SIN", "SENO"),
    ("CEILING", "MULTIPLO.SUPERIOR"),
    ("CEILING.MATH", "MULTIPLO.SUPERIOR.MAT"),
    ("FLOOR", "MULTIPLO.INFERIOR"),
    ("FLOOR.MATH", "MULTIPLO.INFERIOR.MAT"),
    ("COMBIN", "COMBINAT"),
    ("DEGREES", "GRADOS"),
    ("RADIANS", "RADIANES"),
    ("EVEN", "REDONDEA.PAR"),
    ("ODD", "REDONDEA.IMPAR"),
    ("GCD", "M.C.D"),
    ("LCM", "M.C.M"),
    ("INT", "ENTERO"),
    ("MOD", "RESIDUO"),
    ("MROUND", "REDOND.MULT"),
    ("POWER", "POTENCIA"),
    ("PRODUCT", "PRODUCTO"),
    ("QUOTIENT", "COCIENTE"),
    ("RAND", "ALEATORIO"),
    ("RANDBETWEEN", "ALEATORIO.ENTRE"),
    ("RANDARRAY", "MATRIZALEAT"),
    ("ROMAN", "NUMERO.ROMANO"),
    ("ARABIC", "NUMERO.ARABE"),
    ("ROUND", "REDONDEAR"),
    ("ROUNDDOWN", "REDONDEAR.MENOS"),
    ("ROUNDUP", "REDONDEAR.MAS"),
    ("SIGN", "SIGNO"),
    ("SQRT", "RAIZ"),
    ("SQRTPI", "RAIZ2PI"),
    ("SUBTOTAL", "SUBTOTALES"),
    ("SUM", "SUMA"),
    ("SUMIF", "SUMAR.SI"),
    ("SUMIFS", "SUMAR.SI.CONJUNTO"),
    ("SUMPRODUCT", "SUMAPRODUCTO"),
    ("SUMSQ", "SUMA.CUADRADOS"),
    ("TRUNC", "TRUNCAR"),
    ("AGGREGATE", "AGREGAR"),
    ("SEQUENCE", "SECUENCIA"),
    ("MINVERSE", "MINVERSA"),
    ("DECIMAL", "CONV.DECIMAL"),
    ("AVERAGE", "PROMEDIO"),
    ("AVERAGEA", "PROMEDIOA"),
    ("AVERAGEIF", "PROMEDIO.SI"),
    ("AVERAGEIFS", "PROMEDIO.SI.CONJUNTO"),
    ("COUNT", "CONTAR"),
    ("COUNTA", "CONTARA"),
    ("COUNTBLANK", "CONTAR.BLANCO"),
    ("COUNTIF", "CONTAR.SI"),
    ("COUNTIFS", "CONTAR.SI.CONJUNTO"),
    ("MAXIFS", "MAX.SI.CONJUNTO"),
    ("MINIFS", "MIN.SI.CONJUNTO"),
    ("MEDIAN", "MEDIANA"),
    ("MODE", "MODA"),
    ("MODE.SNGL", "MODA.UNO"),
    ("MODE.MULT", "MODA.VARIOS"),
    ("LARGE", "K.ESIMO.MAYOR"),
    ("SMALL", "K.ESIMO.MENOR"),
    ("RANK", "JERARQUIA"),
    ("RANK.EQ", "JERARQUIA.EQV"),
    ("RANK.AVG", "JERARQUIA.MEDIA"),
    ("STDEV", "DESVEST"),
    ("STDEV.S", "DESVEST.M"),
    ("STDEV.P", "DESVEST.P"),
    ("STDEVP", "DESVESTP"),
    ("STDEVA", "DESVESTA"),
    ("STDEVPA", "DESVESTPA"),
    ("PERCENTILE", "PERCENTIL"),
    ("PERCENTILE.INC", "PERCENTIL.INC"),
    ("PERCENTILE.EXC", "PERCENTIL.EXC"),
    ("QUARTILE", "CUARTIL"),
    ("QUARTILE.INC", "CUARTIL.INC"),
    ("QUARTILE.EXC", "CUARTIL.EXC"),
    ("CORREL", "COEF.DE.CORREL"),
    ("FORECAST", "PRONOSTICO"),
    ("FORECAST.LINEAR", "PRONOSTICO.LINEAL"),
    ("FORECAST.ETS", "PRONOSTICO.ETS"),
    ("TREND", "TENDENCIA"),
    ("GROWTH", "CRECIMIENTO"),
    ("SLOPE", "PENDIENTE"),
    ("INTERCEPT", "INTERSECCION.EJE"),
    ("RSQ", "COEFICIENTE.R2"),
    ("LINEST", "ESTIMACION.LINEAL"),
    ("LOGEST", "ESTIMACION.LOGARITMICA"),
    ("FREQUENCY", "FRECUENCIA"),
    ("GEOMEAN", "MEDIA.GEOM"),
    ("HARMEAN", "MEDIA.ARMO"),
    ("TRIMMEAN", "MEDIA.ACOTADA"),
    ("AVEDEV", "DESVPROM"),
    ("DEVSQ", "DESVIA2"),
    ("KURT", "CURTOSIS"),
    ("SKEW", "COEFICIENTE.ASIMETRIA"),
    ("STANDARDIZE", "NORMALIZACION"),
    ("NORM.DIST", "DISTR.NORM.N"),
    ("NORM.INV", "INV.NORM"),
    ("NORM.S.DIST", "DISTR.NORM.ESTAND.N"),
    ("NORM.S.INV", "INV.NORM.ESTAND"),
    ("NORMDIST", "DISTR.NORM"),
    ("NORMINV", "DISTR.NORM.INV"),
    ("NORMSDIST", "DISTR.NORM.ESTAND"),
    ("NORMSINV", "DISTR.NORM.ESTAND.INV"),
    ("T.TEST", "PRUEBA.T.N"),
    ("TTEST", "PRUEBA.T"),
    ("CONFIDENCE", "INTERVALO.CONFIANZA"),
    ("CONFIDENCE.NORM", "INTERVALO.CONFIANZA.NORM"),
    ("CONFIDENCE.T", "INTERVALO.CONFIANZA.T"),
    ("COVARIANCE.P", "COVARIANZA.P"),
    ("COVARIANCE.S", "COVARIANZA.M"),
    ("PERMUT", "PERMUTACIONES"),
    ("BINOM.DIST", "DISTR.BINOM.N"),
    ("EXPON.DIST", "DISTR.EXP.N"),
    ("AND", "Y"),
    ("OR", "O"),
    ("NOT", "NO"),
    ("XOR", "XO"),
    ("IF", "SI"),
    ("IFS", "SI.CONJUNTO"),
    ("IFERROR", "SI.ERROR"),
    ("IFNA", "SI.ND"),
    ("SWITCH", "CAMBIAR"),
    ("TRUE", "VERDADERO"),
    ("FALSE", "FALSO"),
    ("CONCATENATE", "CONCATENAR"),
    ("TEXTJOIN", "UNIRCADENAS"),
    ("LEFT", "IZQUIERDA"),
    ("RIGHT", "DERECHA"),
    ("MID", "EXTRAE"),
    ("LEN", "LARGO"),
    ("UPPER", "MAYUSC"),
    ("LOWER", "MINUSC"),
    ("PROPER", "NOMPROPIO"),
    ("TRIM", "ESPACIOS"),
    ("SUBSTITUTE", "SUSTITUIR"),
    ("REPLACE", "REEMPLAZAR"),
    ("FIND", "ENCONTRAR"),
    ("SEARCH", "HALLAR"),
    ("TEXT", "TEXTO"),
    ("VALUE", "VALOR"),
    ("REPT", "REPETIR"),
    ("CLEAN", "LIMPIAR"),
    ("CHAR", "CARACTER"),
    ("CODE", "CODIGO"),
    ("EXACT", "IGUAL"),
    ("DOLLAR", "MONEDA"),
    ("FIXED", "DECIMAL"),
    ("TEXTBEFORE", "TEXTOANTES"),
    ("TEXTAFTER", "TEXTODESPUES"),
    ("TEXTSPLIT", "DIVIDIRTEXTO"),
    ("NUMBERVALUE", "VALOR.NUMERO"),
    ("UNICHAR", "UNICAR"),
    ("HYPERLINK", "HIPERVINCULO"),
    ("FORMULATEXT", "FORMULATEXTO"),
    ("VALUETOTEXT", "VALORATEXTO"),
    ("ARRAYTOTEXT", "MATRIZATEXTO"),
    ("DATE", "FECHA"),
    ("DATEVALUE", "FECHANUMERO"),
    ("DAY", "DIA"),
    ("DAYS", "DIAS"),
    ("DAYS360", "DIAS360"),
    ("EDATE", "FECHA.MES"),
    ("EOMONTH", "FIN.MES"),
    ("HOUR", "HORA"),
    ("MINUTE", "MINUTO"),
    ("SECOND", "SEGUNDO"),
    ("MONTH", "MES"),
    ("YEAR", "AÑO"),
    ("NOW", "AHORA"),
    ("TODAY", "HOY"),
    ("TIME", "NSHORA"),
    ("TIMEVALUE", "HORANUMERO"),
    ("WEEKDAY", "DIASEM"),
    ("WEEKNUM", "NUM.DE.SEMANA"),
    ("ISOWEEKNUM", "ISO.NUM.DE.SEMANA"),
    ("NETWORKDAYS", "DIAS.LAB"),
    ("NETWORKDAYS.INTL", "DIAS.LAB.INTL"),
    ("WORKDAY", "DIA.LAB"),
    ("WORKDAY.INTL", "DIA.LAB.INTL"),
    ("YEARFRAC", "FRAC.AÑO"),
    ("DATEDIF", "SIFECHA"),
    ("VLOOKUP", "BUSCARV"),
    ("HLOOKUP", "BUSCARH"),
    ("LOOKUP", "BUSCAR"),
    ("XLOOKUP", "BUSCARX"),
    ("MATCH", "COINCIDIR"),
    ("XMATCH", "COINCIDIRX"),
    ("INDEX", "INDICE"),
    ("OFFSET", "DESREF"),
    ("INDIRECT", "INDIRECTO"),
    ("CHOOSE", "ELEGIR"),
    ("COLUMN", "COLUMNA"),
    ("COLUMNS", "COLUMNAS"),
    ("ROW", "FILA"),
    ("ROWS", "FILAS"),
    ("ADDRESS", "DIRECCION"),
    ("TRANSPOSE", "TRANSPONER"),
    ("FILTER", "FILTRAR"),
    ("SORT", "ORDENAR"),
    ("SORTBY", "ORDENARPOR"),
    ("UNIQUE", "UNICOS"),
    ("HSTACK", "APILARH"),
    ("VSTACK", "APILARV"),
    ("TAKE", "TOMAR"),
    ("DROP", "EXCLUIR"),
    ("CHOOSECOLS", "ELEGIRCOLS"),
    ("CHOOSEROWS", "ELEGIRFILAS"),
    ("TOCOL", "ENCOL"),
    ("TOROW", "ENFILA"),
    ("EXPAND", "EXPANDIR"),
    ("WRAPCOLS", "AJUSTARCOLS"),
    ("WRAPROWS", "AJUSTARFILAS"),
    ("ISBLANK", "ESBLANCO"),
    ("ISERROR", "ESERROR"),
    ("ISERR", "ESERR"),
    ("ISNA", "ESNOD"),
    ("ISNUMBER", "ESNUMERO"),
    ("ISTEXT", "ESTEXTO"),
    ("ISNONTEXT", "ESNOTEXTO"),
    ("ISLOGICAL", "ESLOGICO"),
    ("ISREF", "ESREF"),
    ("ISEVEN", "ES.PAR"),
    ("ISODD", "ES.IMPAR"),
    ("ISFORMULA", "ESFORMULA"),
    ("NA", "NOD"),
    ("TYPE", "TIPO"),
    ("ERROR.TYPE", "TIPO.DE.ERROR"),
    ("CELL", "CELDA"),
    ("SHEET", "HOJA"),
    ("SHEETS", "HOJAS"),
    ("NPV", "VNA"),
    ("IRR", "TIR"),
    ("PMT", "PAGO"),
    ("PV", "VA"),
    ("FV", "VF"),
    ("RATE", "TASA"),
    ("IPMT", "PAGOINT"),
    ("PPMT", "PAGOPRIN"),
    ("XNPV", "VNA.NO.PER"),
    ("XIRR", "TIR.NO.PER"),
    ("MIRR", "TIRM"),
    ("EFFECT", "INT.EFECTIVO"),
    ("NOMINAL", "TASA.NOMINAL"),
    ("CUMIPMT", "PAGO.INT.ENTRE"),
    ("CUMPRINC", "PAGO.PRINC.ENTRE"),
    ("CONVERT", "CONVERTIR"),
    ("DEC2BIN", "DEC.A.BIN"),
    ("DEC2HEX", "DEC.A.HEX"),
    ("BIN2DEC", "BIN.A.DEC"),
    ("HEX2DEC", "HEX.A.DEC"),
];

impl FormulaLocale {
    pub fn list_separator(self) -> char {
        if self == Self::Es { ';' } else { ',' }
    }
    pub fn decimal_separator(self) -> char {
        if self == Self::Es { ',' } else { '.' }
    }
    pub fn array_column_separator(self) -> char {
        if self == Self::Es { '\\' } else { ',' }
    }
    pub fn function_name(self, canonical: &str) -> &str {
        if self == Self::Es {
            SPANISH.iter().find(|(en, _)| crate::same_name(en, canonical)).map(|(_, es)| *es).unwrap_or(canonical)
        } else {
            canonical
        }
    }
    pub fn canonical_name(self, localized: &str) -> &str {
        if self == Self::Es {
            SPANISH.iter().find(|(_, es)| crate::same_name(es, localized)).map(|(en, _)| *en).unwrap_or(localized)
        } else {
            localized
        }
    }
}

/// Parses UI input using the selected function table and separators. Existing names and
/// LET/LAMBDA bindings take precedence over aliases. File and API syntax uses [`crate::parse`].
pub fn parse_input(src: &str, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<Expr, ParseError> {
    let mut expr = parser::parse_syntax(src, locale)?;
    if locale == FormulaLocale::Es {
        normalize(&mut expr, &known_name, &mut Vec::new(), 0, false)?;
    }
    Ok(expr)
}

/// Prints canonical expressions for editing. Name collisions retain the canonical spelling
/// so displaying then committing a formula never changes a defined-name reference.
pub fn print_input(expr: &Expr, locale: FormulaLocale, known_name: impl Fn(&str) -> bool) -> Result<String, ParseError> {
    // Check before cloning/printing. A flat operator chain such as A1+A2+… nests on its left
    // without nesting in the text: cloning, printing and `normalize` follow that left spine in
    // a loop, so it does not count (Excel accepts 4,096 operands, #916). Other nesting does.
    let mut pending = vec![(expr, 0usize)];
    while let Some((node, depth)) = pending.pop() {
        if depth > parser::MAX_DEPTH {
            return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
        }
        match node {
            Expr::Unary(_, child) | Expr::Paren(child) => pending.push((child, depth + 1)),
            Expr::Binary(_, left, right) => {
                pending.push((left, depth));
                pending.push((right, depth + 1));
            }
            Expr::Call(_, args) => pending.extend(args.iter().map(|e| (e, depth + 1))),
            Expr::Invoke(callee, args) => {
                pending.push((callee, depth + 1));
                pending.extend(args.iter().map(|e| (e, depth + 1)));
            }
            Expr::Array(rows) => pending.extend(rows.iter().flatten().map(|e| (e, depth + 1))),
            _ => {}
        }
    }
    if locale == FormulaLocale::En {
        return Ok(crate::print(expr));
    }
    let mut expr = expr.clone();
    normalize(&mut expr, &known_name, &mut Vec::new(), 0, true)?;
    let canonical = crate::print(&expr);
    let tokens = crate::lexer::tokenize(&canonical).map_err(|e| ParseError { msg: e.msg, pos: e.pos })?;
    let mut out = String::new();
    let mut array_depth = 0usize;
    for token in tokens {
        use crate::lexer::Tok;
        let text = canonical.get(token.start..token.end).unwrap_or("");
        match token.tok {
            Tok::LBrace => {
                array_depth += 1;
                out.push_str(text);
            }
            Tok::RBrace => {
                array_depth = array_depth.saturating_sub(1);
                out.push_str(text);
            }
            Tok::Number(_) => out.push_str(&text.replace('.', &locale.decimal_separator().to_string())),
            Tok::Comma => out.push(if array_depth > 0 { locale.array_column_separator() } else { locale.list_separator() }),
            _ => out.push_str(text),
        }
    }
    Ok(out)
}

pub(crate) fn boolean_alias(name: &str) -> Option<bool> {
    if name.eq_ignore_ascii_case("VERDADERO") {
        Some(true)
    } else if name.eq_ignore_ascii_case("FALSO") {
        Some(false)
    } else {
        None
    }
}

fn normalize(expr: &mut Expr, known_name: &impl Fn(&str) -> bool, bindings: &mut Vec<String>, depth: usize, display: bool) -> Result<(), ParseError> {
    if depth > parser::MAX_DEPTH {
        return Err(ParseError { msg: "formula is nested too deeply".into(), pos: 0 });
    }
    let bound = |name: &str, bindings: &[String]| bindings.iter().rev().any(|n| crate::same_name(n, name)) || known_name(name);
    match expr {
        Expr::Bool(value) if display => {
            let alias = if *value { "VERDADERO" } else { "FALSO" };
            if !bound(alias, bindings) {
                *expr = Expr::Name(alias.into());
            }
        }
        Expr::Name(name) if !display && !bound(name, bindings) => {
            if let Some(value) = boolean_alias(name) {
                *expr = Expr::Bool(value);
            }
        }
        Expr::Call(name, args) if name == "LET" && args.len() >= 3 && !args.len().is_multiple_of(2) => {
            let saved = bindings.len();
            if let Some((body, pairs)) = args.split_last_mut() {
                for pair in pairs.chunks_mut(2) {
                    if let [declaration, value] = pair {
                        // The new name is not in scope in its own value expression.
                        normalize(value, known_name, bindings, depth + 1, display)?;
                        if let Expr::Name(name) = declaration {
                            bindings.push(name.clone());
                        }
                    }
                }
                normalize(body, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) if name == "LAMBDA" => {
            let saved = bindings.len();
            if let Some((body, params)) = args.split_last_mut() {
                for param in params {
                    if let Expr::Name(name) = param {
                        bindings.push(name.clone());
                    }
                }
                normalize(body, known_name, bindings, depth + 1, display)?;
            }
            bindings.truncate(saved);
        }
        Expr::Call(name, args) => {
            let alias = if display { FormulaLocale::Es.function_name(name) } else { FormulaLocale::Es.canonical_name(name) };
            if !bound(name, bindings) && (!display || !bound(alias, bindings)) {
                *name = alias.to_string();
            }
            for arg in args {
                normalize(arg, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Invoke(callee, args) => {
            normalize(callee, known_name, bindings, depth + 1, display)?;
            for arg in args {
                normalize(arg, known_name, bindings, depth + 1, display)?;
            }
        }
        Expr::Unary(_, child) | Expr::Paren(child) => normalize(child, known_name, bindings, depth + 1, display)?,
        Expr::Binary(..) => {
            // Follow the left spine of a flat chain (A1+A2+…) in a loop: it is not nesting.
            // Each call leaves `bindings` as it found them, so the order does not matter.
            let mut node = expr;
            while let Expr::Binary(_, left, right) = node {
                normalize(right, known_name, bindings, depth + 1, display)?;
                node = left;
            }
            normalize(node, known_name, bindings, depth, display)?;
        }
        Expr::Array(rows) if display => {
            for row in rows {
                for child in row {
                    normalize(child, known_name, bindings, depth + 1, display)?;
                }
            }
        }
        // Array elements have already been parsed as literals. Sheet/structured references
        // contain user-defined labels and must not be translated.
        _ => {}
    }
    Ok(())
}

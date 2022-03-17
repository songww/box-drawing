use std::io::Write;

use num::ToPrimitive;
use pyo3::prelude::*;
use rustpython_parser::{ast, error, parser};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=boxDrawing.py");
    println!("cargo:rerun-if-changed=boxDrawingRecipes.py");

    let mut output = String::from("match c {\n");

    Python::with_gil(|py| {
        let module = PyModule::from_code(
            py,
            include_str!("../../boxDrawingRecipes.py"),
            "boxDrawingRecipes.py",
            "boxDrawingRecipes",
        )
        .unwrap();
        let recipes = module.getattr("recipes").unwrap();
        for recipe in recipes.iter().unwrap() {
            let k = recipe.unwrap();
            let (name, code) = (k.get_item(0).unwrap(), k.get_item(1).unwrap());

            let _: u32 = u32::from_str_radix(&code.to_string(), 16).unwrap();
            let name = name.to_string();

            let commands = recipes.get_item(k).unwrap();
            let commands = commands
                .iter()
                .unwrap()
                .map(|cmd| {
                    Ok(cmd?
                        .to_string()
                        .replace("boxPen,", "")
                        .replace("(boxPen)", "()")
                        .replace("box(", "box_("))
                })
                .collect::<Result<Vec<_>, PyErr>>()
                .unwrap();

            let commands: Result<Vec<ast::Expression>, error::ParseError> = commands
                .iter()
                .map(|c| parser::parse_expression(c))
                .collect();

            output.push_str(&format!(
                "0x{code} => Recipe {{c: 0x{code}, name: \"{name}\", commands: Box::new([ ", // {cmds} ]) }},",
                code = code.to_string(),
                name = &name,
            ));
            output.push('\n');
            for command in commands.unwrap().iter() {
                let mut s = String::new();
                let mrequired = build_command(&mut s, command);

                let m = if mrequired { "m" } else { "_" };
                output.push_str(&format!(
                    "Box::new(|{m}: &Metrics<F>| {{ {closure} }}),\n",
                    m = m,
                    closure = &s
                ));
            }
            output.push_str("]) },\n");
        }
        output.push_str("}\n");
        println!("cargo:warning= {}", output);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("src/recipes.rs.in")
            .unwrap();
        f.write_all(output.as_bytes()).unwrap();
    });
}

fn build_command<'a>(s: &'a mut String, command: &ast::Located<ast::ExpressionType>) -> bool {
    let mut mrequired = false;
    match command.node {
        ast::ExpressionType::Call {
            ref function,
            ref args,
            ref keywords,
        } => {
            // println!("cargo:warning= 0x{} => {}({:?}) {:?}", code, function.name(), args, command.node );
            let mut name;
            match function.node {
                ast::ExpressionType::Identifier { name: ref n } => {
                    // print!("cargo:warning= 0x{} => {}(", code, name );
                    name = n.clone();
                }
                _ => {
                    // println!("cargo:warning= 0x{} => unknown({}) {:?}", code, name, function.node );
                    unreachable!()
                }
            }
            name.get_mut(0..1).map(|s| {
                s.make_ascii_uppercase();
                &*s
            });

            s.push_str(&format!("let builder = {}Builder::default();\n", name));

            for (idx, arg) in args.iter().enumerate() {
                // s.push_str(&format!("let attr = {}::position({});\n", name, idx));
                s.push_str(&format!("builder.set_{}(", idx));
                match arg.node {
                    ast::ExpressionType::Identifier { ref name } => {
                        mrequired = true;
                        s.push_str(normalized_const(&name));
                    }
                    ast::ExpressionType::String {
                        value: ast::StringGroup::Constant { ref value },
                    } => {
                        let v = match value.as_str() {
                            "right" => "Side::TopRight",
                            "left" => "Side::BottomLeft",
                            "top" => "Side::TopLeft",
                            "bottom" => "Side::BottomRight",
                            "TL" => "Side::TopLeft",
                            "TR" => "Side::TopRight",
                            "BL" => "Side::BottomLeft",
                            "BR" => "Side::BottomRight",
                            "bottomUp" => "Direction::BottomUp",
                            "topDown" => "Direction::topDown",
                            "25" => "Shade::TwentyFive",
                            "50" => "Shade::Fifty",
                            "75" => "Shade::SeventyFive",
                            _ => {
                                unreachable!("unmatched const string: '{}' <-> {:?}", value, arg);
                            }
                        };
                        s.push_str(v);
                    }
                    ast::ExpressionType::Tuple { ref elements } => {
                        if take_tuple(s, &elements) {
                            mrequired = true;
                        }
                    }
                    _ => {
                        unreachable!("cargo:warning= => {}( {:?}", name, arg.node);
                    }
                }
                s.push_str(".into());\n");
            }
            for keyword in keywords.iter() {
                let kwname = keyword.name.as_ref().unwrap();
                let attr = normalized_keyword(&kwname);
                s.push_str("builder.");
                s.push_str(attr);
                s.push_str("(");
                if take_expr(s, &keyword.value) {
                    mrequired = true;
                }
                s.push_str(".into());");
                s.push('\n');
            }
        }
        _ => {
            // println!("cargo:warning= 0x{} => ({}) {:?}", code, name, command.node );
            unreachable!()
        }
    }
    s.push_str("builder.build().unwrap().into()\n");

    mrequired
}

fn take_expr<'a>(s: &'a mut String, expr: &ast::Located<ast::ExpressionType>) -> bool {
    let mut mrequired = false;
    match &expr.node {
        ast::ExpressionType::Binop { a, op, b } => {
            if take_expr(s, &a) {
                mrequired = true;
            }
            match op {
                ast::Operator::Add => {
                    s.push('+');
                }
                ast::Operator::Sub => s.push('-'),
                ast::Operator::Mult => s.push('*'),
                ast::Operator::Div => s.push('/'),

                _ => {
                    unreachable!("op : {:?}", op);
                }
            }
            if take_expr(s, &b) {
                mrequired = true;
            }
        }
        ast::ExpressionType::Unop { op, a } => {
            match op {
                ast::UnaryOperator::Neg => {
                    s.push('-');
                }
                _ => unreachable!("Unop {:?}", op),
            };
            if take_expr(s, &a) {
                mrequired = true;
            }
        }
        ast::ExpressionType::Identifier { ref name } => {
            s.push_str(normalized_const(&name));
            mrequired = true;
        }
        ast::ExpressionType::Number { value: number } => {
            take_number(s, number);
        }
        ast::ExpressionType::Tuple { ref elements } => {
            if take_tuple(s, &elements) {
                mrequired = true;
            }
        }
        ast::ExpressionType::Subscript { a, b } => {
            match a.node {
                ast::ExpressionType::Identifier { ref name } => {
                    s.push_str(normalized_const(&name));
                    mrequired = true;
                }
                _ => unreachable!(),
            };
            match b.node {
                ast::ExpressionType::Number { ref value } => match value {
                    ast::Number::Integer { value } => {
                        let v = value.to_isize().unwrap();
                        match v {
                            0 => {
                                s.push_str(".x");
                            }
                            1 => {
                                s.push_str(".y");
                            }
                            _ => {
                                unreachable!("Subscript out of range: {:?}", b.node)
                            }
                        };
                    }
                    _ => {
                        unreachable!("Subscript invalid number type: {:?}", b.node)
                    }
                },
                _ => {
                    unreachable!("Subscript invalid type: {:?}", b.node)
                }
            }
        }
        _ => unreachable!("{:?}", expr),
    };
    // println!("cargo:warning= take-expr: {}", value);
    mrequired
}

fn take_tuple<'a>(s: &'a mut String, elements: &[ast::Located<ast::ExpressionType>]) -> bool {
    assert_eq!(elements.len(), 2);

    let mut mrequired = false;

    s.push_str("Point::from((");
    if take_expr(s, &elements[0]) {
        mrequired = true;
    }
    s.push_str(", ");
    if take_expr(s, &elements[1]) {
        mrequired = true;
    }
    s.push_str("))");

    mrequired
}

fn take_number<'a>(s: &'a mut String, number: &ast::Number) -> &'a mut String {
    s.push_str("F::from(");
    match &number {
        ast::Number::Integer { value: integer } => {
            s.push_str(&integer.to_string());
        }
        ast::Number::Float { value: float } => {
            s.push_str(&float.to_string());
        }
        ast::Number::Complex { real: _, imag: _ } => {
            unreachable!()
        }
    }
    s.push_str("f64).unwrap()");
    s
}

fn normalized_keyword(name: &str) -> &'static str {
    match name {
        "buttB" => "butt_bot",
        "buttT" => "butt_top",
        "buttL" => "butt_left",
        "buttR" => "butt_right",
        "start" => "start",
        "end" => "end",
        "step" => "step",
        "stroke" => "stroke",
        _ => unreachable!("keyword name=>{}", name),
    }
}

fn normalized_const(name: &str) -> &'static str {
    match name {
        "FAT" => "m.fat",
        "BUTT" => "m.butt",
        "WIDTH" => "m.width",
        "HEIGHT" => "m.height",
        "MEDIAN" => "m.median",
        "RADIUS" => "m.radius",
        "STROKE" => "m.stroke",
        "EM_HEIGHT" => "m.em_height",
        "BLOCK_TOP" => "m.block_top",
        "FAT_STROKE" => "m.fat_stroke",
        "BLOCK_HEIGHT" => "m.block_height",
        "BLOCK_ORIGIN" => "m.block_origin",
        _ => unreachable!("constant value {}", name),
    }
}

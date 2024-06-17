use clearscreen::clear;
use colored::Colorize;
use regex::Regex;
use std::io::{stdout, Write};
use std::{env::args, fs::File, io::Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cds = String::new();
    let mut fns: Vec<CODE> = Vec::new();

    let inf: Vec<String> = args().collect();
    if inf.len() > 2 {
        eprintln!("{}", "Can run only 1 file at a time!".blink().bold().red());
        return Err("Too many input files".into());
    } else if inf.len() == 1 {
        eprintln!("{}", "Need at least 1 file to run!".blink().bold().red());
        return Err("No input file provided".into());
    }

    let inf = inf.get(1).unwrap();
    let mut inf_f = File::open(inf)?;

    inf_f.read_to_string(&mut cds)?;
    println!("File content loaded successfully");

    let cdpieces = cds.lines();
    let mut curfn = String::new();
    let mut isfunc = false;
    let mut isbraces = false;

    for nlcdp in cdpieces {
        let trimmed_line = nlcdp.trim();

        if trimmed_line.starts_with("ON ") {
            let fn_name = trimmed_line.strip_prefix("ON ").unwrap().trim();
            let fn_name = fn_name.trim_matches(|c| c == '(' || c == ')' || c == '{' || c == '}');
            let newfn = CODE {
                fnnm: fn_name.to_string(),
                code: Vec::new(),
            };
            fns.push(newfn);
            curfn = fn_name.to_string();
            isfunc = true;
        } else if trimmed_line == "{" && isfunc {
            isbraces = true;
        }
        else if trimmed_line == "}" && isbraces {
            isbraces = false;
            isfunc = false;
        } else if isfunc {
            if let Some(func) = fns.iter_mut().find(|f| f.fnnm == curfn) {
                if trimmed_line != "}" {
                    func.code.push(trimmed_line.to_string());
                }
            }
        }
    }

    println!("Finished processing file. Functions found:");
    for i in &fns {
        println!("Function: {} with {} lines of code", i.fnnm, i.code.len());
    }
    clear().unwrap();
    for main in fns.clone() {
        if main.fnnm == "main" {
            CODE::run(main, &fns.clone());
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct CODE {
    fnnm: String,
    code: Vec<String>,
}

impl Run for CODE {}

trait Run {
    fn run(code: CODE, fns: &[CODE]) {
        let cds = code.code;
        let echonlregex = Regex::new(r#"echonl\("([^"]+)"\);"#).unwrap();

        let echolregex = Regex::new(r#"echol\("([^"]+)"\);"#).unwrap();

        for cd in cds {
            if cd.contains("echonl") {
                if let Some(cap) = echonlregex.captures(&cd) {
                    let mut x = 1;
                    while x < cap.len() {
                        let txt = cap.get(x).unwrap();
                        println!("{}", txt.as_str().trim());
                        x += 1;
                    }
                    //stdout().flush().unwrap();
                    println!("");
                }
            } else if cd.contains("echol") {
                if let Some(cap) = echolregex.captures(&cd) {
                    let mut x = 1;
                    while x < cap.len() {
                        let txt = cap.get(x).unwrap();
                        print!("{}", txt.as_str().trim());
                        x += 1;
                    }
                    println!("");
                }
                else if cd == "out.flush();"{
                    stdout().flush().unwrap();
                }
            } else {
                for fnn in fns {
                    if cd.contains(&fnn.fnnm) {
                        CODE::run(fnn.clone(), fns);
                    }
                }
            }
        }
    }
}

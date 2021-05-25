extern crate rikai;

use rikai::parse;
use rikai::writer;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use structopt::StructOpt;
use tera::Tera;

#[derive(Serialize, Deserialize)]
struct Settings {
    template: Option<String>,
    comment_head: String,
    comment_tail: String,
    separate_line: String,
}

#[derive(StructOpt)]
struct Cl {
    jsonfile: String,
}

fn read_json(args: &Cl) -> Settings {
    let path = Path::new(&args.jsonfile);
    if !path.exists() {
        panic!("Jsonファイルが存在しません");
    }
    let mut json_text = String::new();
    File::open(&args.jsonfile)
        .unwrap()
        .read_to_string(&mut json_text)
        .unwrap();
    serde_json::from_str(&json_text).unwrap()
}

fn main() -> io::Result<()> {
    let args = Cl::from_args();
    let problem_elements = parse::from_io();
    let json: Settings = read_json(&args);

    let file_dir = Path::new(&args.jsonfile).parent().unwrap();

    let current_path = std::env::current_dir().unwrap();
    let current_path = current_path.to_str().unwrap();
    let current_path = Path::new(current_path).join(file_dir);

    let writer = writer::Writer {
        comment_head: json.comment_head,
        comment_tail: json.comment_tail,
        separate_line: json.separate_line,
    };

    if let Some(filename) = json.template {
        let filepath = current_path.join(&filename);
        if !filepath.exists() {
            panic!("ファイルが存在しません: {}", filepath.to_str().unwrap());
        }
        let mut template = String::new();
        File::open(filepath)
            .unwrap()
            .read_to_string(&mut template)
            .unwrap();
        let mut context = tera::Context::new();

        for problem in problem_elements {
            let problem_is = problem.is();
            let writer_lines = writer.lines(problem).concat();
            match problem_is {
                parse::ParagrahKind::Problem => context.insert("problem", &writer_lines),
                parse::ParagrahKind::Limit => context.insert("limit", &writer_lines),
                parse::ParagrahKind::Input => context.insert("input", &writer_lines),
                parse::ParagrahKind::Output => context.insert("output", &writer_lines),
                _ => context.insert("example", &writer_lines),
            }
        }

        match Tera::one_off(&template, &context, false) {
            Ok(t) => print!("{}", t),
            Err(e) => {
                println!("テンプレートファイルを書き出している最中にエラーが発生しました");
                println!("{}", e);
                ::std::process::exit(1);
            }
        }
    } else {
        for line in problem_elements.into_iter().map(|x| writer.lines(x)) {
            println!("{}", line.concat());
        }
    }
    Ok(())
}

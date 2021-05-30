extern crate rikai;

use rikai::entry;
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
    template: String,
    has_blank_line: Option<bool>,
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
        opt_blank: json.has_blank_line,
    };

    let filepath = current_path.join(&json.template);
    if !filepath.exists() {
        panic!("ファイルが存在しません: {}", filepath.to_str().unwrap());
    }
    let mut template = String::new();
    File::open(filepath)
        .unwrap()
        .read_to_string(&mut template)
        .unwrap();

    let mut context = tera::Context::new();
    let mut example_vec = <Vec<entry::Example>>::new();

    for problem in problem_elements {
        let writer_lines = writer.lines(&problem).concat();
        match problem.is() {
            parse::ParagrahKind::Problem => context.insert("problem", &writer_lines),
            parse::ParagrahKind::Limit => context.insert("limit", &writer_lines),
            parse::ParagrahKind::Input => context.insert("input", &writer.to_input(&problem)),
            parse::ParagrahKind::Output => context.insert("output", &writer_lines),
            _ => example_vec.push(entry::Example::new(writer_lines, problem.pre())),
        }
    }

    let timetext = chrono::Local::now()
        .format("%Y年%m月%d日 %H時%M分%S秒 %Z")
        .to_string();

    context.insert("examples", &example_vec);
    context.insert("time", &timetext);

    match Tera::one_off(&template, &context, false) {
        Ok(t) => print!("{}", t),
        Err(e) => {
            println!("テンプレートファイルを書き出している最中にエラーが発生しました");
            println!("{}", e);
            ::std::process::exit(1);
        }
    }
    Ok(())
}

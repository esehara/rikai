use scraper;

#[derive(Debug)]
pub enum ParagrahKind {
    Problem,
    Limit,
    Input,
    Output,
    Example,
}

fn latex_fix(s: &String) -> String {
    s.replace("\\leq", "<=")
        .replace("\\dots", "...")
        .replace("\\ldots", "...")
}

#[derive(Debug)]
pub struct Paragraph {
    pub title: String,
    raw_text: Vec<String>,
    pub pre: String,
}

impl Paragraph {
    pub fn pre(&self) -> Vec<String> {
        self.pre
            .split("\n")
            .filter(|x| x != &"")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }
    pub fn text(&self) -> String {
        let raw_text = &self.raw_text;
        raw_text
            .into_iter()
            .map(|x| latex_fix(x))
            .collect::<Vec<String>>()
            .concat()
    }

    pub fn is(&self) -> ParagrahKind {
        match &self.title[..] {
            | "問題文" => ParagrahKind::Problem,
            | "制約" => ParagrahKind::Limit,
            | "入力" => ParagrahKind::Input,
            | "出力" => ParagrahKind::Output,
            | _ => ParagrahKind::Example
            }
    }

    pub fn lines(&self, blank: bool) -> Vec<String> {
        let line_strings = self.text();
        let line_strings = line_strings.split("\n").map(|x| String::from(x));

        if blank {
            line_strings.collect::<Vec<String>>()
        } else {
            line_strings
                .filter(|x| x != &"".to_string())
                .collect::<Vec<String>>()
        }
    }
}

pub fn from_io() -> Vec<Paragraph> {
    problem_elements(read_io())
        .into_iter()
        .map(|x| to_paragraph(x))
        .collect()
}

fn read_io() -> String {
    use std::io::Read;
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    stdin.read_to_string(&mut buf).unwrap();
    buf
}

fn problem_elements(buf: String) -> Vec<String> {
    let documents = scraper::Html::parse_document(&buf);
    let problem_root_selector = scraper::Selector::parse("span.lang-ja").unwrap();
    let problem_elements_selector = scraper::Selector::parse("div.part").unwrap();
    let problem_root_element = documents.select(&problem_root_selector).next().unwrap();
    problem_root_element
        .select(&problem_elements_selector)
        .map(|x| x.inner_html())
        .collect()
}

pub fn to_paragraph(raw_html: String) -> Paragraph {
    let fragments = scraper::Html::parse_fragment(&raw_html);
    let selector = scraper::Selector::parse("section").unwrap();
    let preselector = scraper::Selector::parse("pre").unwrap();
    let section = fragments.select(&selector).next().unwrap();

    let text = section
        .text()
        .filter(|x| x != &"")
        .map(|x| String::from(x))
        .collect::<Vec<_>>();

    let pre = if let Some(pre) = section.select(&preselector).next() {
        pre.text()
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
            .concat()
    } else {
        String::new()
    };

    Paragraph {
        title: String::from(&text[1]),
        raw_text: (text[2..]).to_vec(),
        pre: pre,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn make_paragraph() {
        let sample_html = String::from(r#"<section>\n<h3>問題文</h3><p>1を出力せよ</p></section>"#);
        let p = to_paragraph(sample_html);
        assert_eq!("問題文", p.title);
        assert_eq!("1を出力せよ", p.text());
    }
}

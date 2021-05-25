use scraper;

#[derive(Debug)]
pub enum ParagrahKind {
    Problem,
    Limit,
    Input,
    Output,
    Example,
}

#[derive(Debug)]
pub struct Paragraph {
    pub title: String,
    raw_text: Vec<String>,
}

impl Paragraph {
    pub fn text(&self) -> String {
        self.raw_text.concat()
    }

    pub fn is(&self) -> ParagrahKind {
        if self.title == "問題文".to_string() {
            ParagrahKind::Problem
        } else if self.title == "制約".to_string() {
            ParagrahKind::Limit
        } else if self.title == "入力".to_string() {
            ParagrahKind::Input
        } else if self.title == "出力".to_string() {
            ParagrahKind::Output
        } else {
            ParagrahKind::Example
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.text()
            .split("\n")
            .map(|x| String::from(x))
            .collect::<Vec<String>>()
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
    let text = fragments
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .filter(|x| x != &"")
        .map(|x| String::from(x))
        .collect::<Vec<_>>();
    Paragraph {
        title: String::from(&text[1]),
        raw_text: (text[2..]).to_vec(),
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

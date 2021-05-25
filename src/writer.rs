use crate::parse;

#[derive(Debug)]
pub struct Writer {
    pub comment_head: String,
    pub comment_tail: String,
    pub separate_line: String,
}

impl Writer {
    pub fn lines(&self, paragrah: parse::Paragraph, blank: bool) -> Vec<String> {
        let mut title_vec = self.title_with_separate_line(&paragrah);
        let mut lines_vec = self.lines_with_comment_syntax(&paragrah, blank);
        title_vec.append(&mut lines_vec);
        title_vec
    }

    fn line_with_comment_syntax(&self, line: &String) -> String {
        format!("{} {} {}\n", self.comment_head, line, self.comment_tail)
    }

    fn add_comment_syntax_to_lines(&self, lines: &Vec<String>) -> Vec<String> {
        lines
            .into_iter()
            .map(|x| self.line_with_comment_syntax(&x))
            .collect::<Vec<String>>()
    }

    pub fn title_with_separate_line(&self, paragrah: &parse::Paragraph) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(self.separate_line.clone());
        lines.push(paragrah.title.clone());
        lines.push(self.separate_line.clone());
        let lines = self.add_comment_syntax_to_lines(&lines);
        lines
    }

    pub fn lines_with_comment_syntax(
        &self,
        paragrah: &parse::Paragraph,
        blank: bool,
    ) -> Vec<String> {
        self.add_comment_syntax_to_lines(&paragrah.lines(blank))
    }
}

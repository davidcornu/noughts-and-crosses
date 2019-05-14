use unicode_segmentation::UnicodeSegmentation;
use std::io;

#[derive(Default)]
pub struct AsciiBox<'a> {
    lines: Vec<(usize, &'a str)>,
    width: usize,
}

impl<'a> AsciiBox<'a> {
    pub fn add_line(&mut self, line: &'a str) {
        let line_width = grapheme_count(line);
        if line_width > self.width {
            self.width = line_width
        }
        self.lines.push((line_width, line));
    }
    pub fn print(&self, screen: &mut impl io::Write, heading: &str) -> Result<(), io::Error> {
        let heading_width = grapheme_count(heading);
        let final_width = if heading_width > self.width {
            heading_width
        } else {
            self.width
        };

        write!(
            screen,
            "┌─{}{}─┐\r\n",
            heading,
            "─".repeat(final_width - heading_width)
        )?;

        for (line_width, line) in self.lines.iter() {
            write!(
                screen,
                "│ {}{} │\r\n",
                line,
                " ".repeat(final_width - line_width)
            )?;
        }

        write!(screen, "└─{}─┘\r\n", "─".repeat(final_width))?;

        Ok(())
    }
}

fn grapheme_count(input: &str) -> usize {
    UnicodeSegmentation::graphemes(input, true).count()
}

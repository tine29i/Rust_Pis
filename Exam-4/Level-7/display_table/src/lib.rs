use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.headers.is_empty() && self.body.is_empty() {
            return Ok(());
        }

        let mut col_widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();

        for row in &self.body {
            for (i, cell) in row.iter().enumerate() {
                if cell.len() > col_widths[i] {
                    col_widths[i] = cell.len();
                }
            }
        }

        let print_row = |row: &[String], col_widths: &[usize]| -> String {
            let mut result = String::new();
            result.push('|');
            for (i, cell) in row.iter().enumerate() {
                let width = col_widths[i];
                let pad_left = (width - cell.len()) / 2;
                let pad_right = width - cell.len() - pad_left;
                result.push_str(&format!(" {}{}{} |", " ".repeat(pad_left), cell, " ".repeat(pad_right)));
            }
            result
        };

        let header = print_row(&self.headers, &col_widths);
        let separator: String = col_widths.iter()
            .map(|&width| format!("|{}+", "-".repeat(width + 2)))
            .collect::<Vec<_>>().join("");
        let separator = format!("{}|", separator.trim_end_matches('+'));

        writeln!(f, "{}", header)?;
        writeln!(f, "{}", separator)?;

        for row in &self.body {
            writeln!(f, "{}", print_row(row, &col_widths))?;
        }

        Ok(())
    }
}
use std::fmt;
use serde::Serialize;
use crate::config;

#[derive(Debug, Serialize)]
pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<f32>>,
}

impl Table {
    pub fn new(headers: Vec<String>, rows: Vec<Vec<f32>>) -> Self {
        Self { headers, rows }
    }

    pub fn headers(&self) -> impl Iterator<Item = &String> {
        self.headers.iter()
    }

    pub fn rows(&self) -> impl Iterator<Item = &Vec<f32>> {
        self.rows.iter()
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_headers: Vec<String> = self
            .headers()
            .map(|h| shorten_name(h, config::MAX_HEADER_WIDTH))
            .collect();

        let mut col_widths: Vec<usize> = display_headers.iter().map(|h| h.len()).collect();

        for row in &self.rows {
            for (i, val) in row.iter().enumerate() {
                let len = format!("{:.2}", val).len();
                if len > col_widths[i] {
                    col_widths[i] = len;
                }
            }
        }

        for (i, header) in display_headers.iter().enumerate() {
            write!(f, "{:>width$}  ", header, width = col_widths[i])?;
        }
        writeln!(f)?;

        for width in &col_widths {
            write!(f, "{:-<width$}--", "", width = *width)?;
        }
        writeln!(f)?;

        for row in &self.rows {
            for (i, value) in row.iter().enumerate() {
                if i == 0 {
                    write!(f, "{:>width$} ", *value as u32, width=col_widths[0])?;
                } else {
                    write!(f, "{:>width$.2} ", value, width=col_widths[i])?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn shorten_name(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else if max > 3 {
        format!("{}...", &s[..max - 3])
    } else {
        s[..max].to_string()
    }
}

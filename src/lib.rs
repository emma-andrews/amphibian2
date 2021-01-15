mod line;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[no_mangle]
pub struct TextEditor {
    lines: Vec<line::Line>, // Holds all lines
    mode: String, // Language mode, i.e. Java or Python
    cur_line: u32, // Current line the editor is on
    is_newline: bool, // If the next inserted line is a new line
}

#[no_mangle]
impl TextEditor {
    fn get_line(&self, index: u32) -> &String {
        // Get the line at the specific index
        match self.lines.get(index as usize) {
            Some(x) => x.get_text(),
            None => panic!("Index out of bounds"), // panic for now
        }
    }

    fn get_mode(&self) -> &String {
        &self.mode
    }

    fn insert_line(&mut self, text: String, index: u32) {
        // Inserts line at specified index
        self.lines.insert(index as usize, line::Line::new(text, self.cur_line))
    }

    fn handle_text(&mut self, text: String) {
        // Handles inputted text
        if self.is_newline {
            // If a newline, then create a new line in the vector
            self.lines.push(line::Line::new(text, self.cur_line));
            self.is_newline = false;
        }
        else {
            // Otherwise add the text to the current line's text
            self.lines[(self.cur_line - 1) as usize].append_text(&text);
        }
    }

    fn handle_backspace(&mut self) {
        // Remove the last character on the current line
        // TODO - update to handle middle of line deletions
        if self.lines.len() != 0 {
            self.lines[(self.cur_line - 1) as usize].remove_char();
        }
    }

    fn handle_delete(&mut self) {
        // TODO - Get cursor location, then find char associated with that
    }
}

#[wasm_bindgen]
#[no_mangle]
impl TextEditor {
    pub fn new(mode: String) -> TextEditor {
        // Create an empty Text Editor
        TextEditor {
            lines: Vec::new(),
            mode: mode,
            cur_line: 1,
            is_newline: true
        }
    }

    pub fn new_from_string(text: String, mode: String) -> TextEditor {
        // Create a Text Editor from a string
        let mut te = TextEditor {
            lines: Vec::new(),
            mode: mode,
            cur_line: 1,
            is_newline: true
        };

        let mut prev_i = 0;
        for (i, c) in text.chars().enumerate() {
            if c == '\n' {
                // Upon encountering a newline, push the previous contents to the vector
                te.append_line(text[prev_i..i].to_string());
                prev_i = i;
            }
        }

        te
    }

    pub fn render(&self) -> String {
        // Get the HTML of all the lines in the vector
        let mut s = String::from("");
        for x in &self.lines {
            s.push_str(&x.get_html());
        }
        s
    }

    pub fn append_line(&mut self, text: String) {
        // Adds a line to the end
        self.lines.push(line::Line::new(text, self.cur_line));
        self.cur_line += 1;
    }

    pub fn get_last_line(&self) -> String {
        self.lines.last().clone().unwrap().get_text().to_string()
    }

    pub fn get_input(&mut self, input_type: String, text: String) {
        // Handle input event based on type of input
        match input_type.as_str() {
            "insertText" => self.handle_text(text), // Handle text input event
            "insertLineBreak" => {self.cur_line += 1; self.is_newline = true}, // Handle newline event
            "deleteContentBackward" => self.handle_backspace(), // Handle backspace event
            "deleteContentForward" => self.handle_delete(), // Handle delete event
            _ => (), // Do nothing on unrecognized event
        };
    }
}
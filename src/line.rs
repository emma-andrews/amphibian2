#[no_mangle]
pub struct Line {
    text: String,
    line_num: u32,
}

#[no_mangle]
impl Line {
    pub fn new(text: String, line_num: u32) -> Line {
        Line {
            text: text,
            line_num: line_num
        }
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn append_text(&mut self, text: &String) {
        self.text += text
    }

    pub fn remove_char(&mut self) {
        // Remove the last character from the line, only if there are characters to remove
        if self.text.chars().count() != 0 {
            self.text.pop();
        }
    }

    pub fn get_html(&self) -> String {
        // Create div containing a span with struct's text
        let mut span = String::from("<div class=\"line\" style=\"height: 14px; top: ");

        // Calculate the value of the top px value, each line has height 18px
        let top: u32 = (self.line_num - 1) * 18;
        
        span.push_str(&top.to_string());
        span.push_str("px;\"><span class=\"line\">");
        span.push_str(&self.text);
        span.push_str("</span></div>");
        span
    }
}
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Colors {
    is_color_supported: bool,
}

#[wasm_bindgen]
impl Colors {
    #[wasm_bindgen(constructor)]
    pub fn new(is_color_supported: bool) -> Self {
        Colors {
            is_color_supported: is_color_supported,
        }
    }

    #[wasm_bindgen]
    pub fn reset(&self, input: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[0m{}\x1b[0m", input)
        } else {
            input.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bold(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[1m{}\x1b[22m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn dim(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[2m{}\x1b[22m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn italic(&self, input: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[3m{}\x1b[23m", input)
        } else {
            input.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn underline(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[4m{}\x1b[24m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn inverse(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[7m{}\x1b[27m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn hidden(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[8m{}\x1b[28m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn strikethrough(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[9m{}\x1b[29m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn black(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[30m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn red(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[31m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn green(&self, input: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[32m{}\x1b[39m", input)
        } else {
            input.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn yellow(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[33m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn blue(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[34m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn magenta(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[35m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn cyan(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[36m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn white(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[37m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn gray(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[90m{}\x1b[39m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_black(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[40m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_red(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[41m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_green(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[42m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_yellow(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[43m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_blue(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[44m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_magenta(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[45m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_cyan(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[46m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_white(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[47m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_gray(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[100m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }

    #[wasm_bindgen]
    pub fn bg_reset(&self, text: &str) -> String {
        if self.is_color_supported {
            format!("\x1b[49m{}\x1b[49m", text)
        } else {
            text.to_string()
        }
    }
}

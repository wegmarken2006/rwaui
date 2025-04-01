use crate::{c_log, DomCfg, Elem};
const SIZE: i32 = 4;

#[cfg(feature = "bulma")]
impl DomCfg {
    pub fn button(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "button");
        e.element.set_inner_html(text);
        e.element.set_attribute("type", "button").unwrap();
        let class = format!("button is-primary is-size-{}", SIZE);
        e.element.set_attribute("class", &class).unwrap();
        e
    }
}

#[cfg(feature = "bulma")]
impl Elem {
    pub fn set_elem_size(&mut self) {
        c_log("SET SIZE 1");
        let mut c_cur = String::new();
        let mut c_new = String::new();
        c_cur = match self.element.get_attribute("class") {
            Some(c) => c,
            None => "".to_string(),
        };
        c_log("SET SIZE 2");
        if c_cur.len() > 0 {
            c_new = format!("{} is-size-{}", c_cur, SIZE);
        } else {
            c_new = format!("is-size-{}", SIZE);
        }

        c_log(&c_new);

        self.element.set_attribute("class", &c_new).unwrap();
    }
}

#[cfg(feature = "pico")]
impl DomCfg {
    pub fn button(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "button");
        e.element.set_inner_html(text);
        e.element.set_attribute("type", "button").unwrap();
        e.element.set_attribute("class", "primary").unwrap();
        e
    }
}

#[cfg(feature = "pico")]
impl Elem {
    pub fn set_elem_size(&mut self) {}
}

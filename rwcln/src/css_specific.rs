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

    pub fn tab(&mut self, id: &str) -> Elem {
        let mut div1 = self.create_element("", "div");
        let class = format!("tabs is-size-{}", SIZE);
        div1.element.set_attribute("class", &class).unwrap();
        let ul1 = self.create_element("", "ul");
        div1.append(&ul1);
        div1.child_1 = Some(ul1.element);
        div1
    }
    /*
        div1 := dom.newElem("", "div")
     class := fmt.Sprintf("tabs %s", SIZE)
     div1.jsValue.Call("setAttribute", "class", class)
     ul1 := dom.newElem("", "ul")
     div1.Append(ul1)
     div1.child1 = ul1.jsValue

     return div1
    } */

    pub fn tab_content(&mut self, tab: &mut Elem, id: &str, title: &str) -> Elem {
        let li = self.create_element("", "li");
        let li_clone = li.clone();
        let a = self.create_element("", "a");
        a.element.set_inner_html(title);
        li.append(&a);
        //li.element.set_inner_html(&format!("<a>{title}</a>"));

        let mut div2 = self.create_element(id, "div");
        div2.element.set_attribute("class", "tabcontent").unwrap();
        div2.child_1 = Some(li.element);
        div2.parent = Some(Box::new(tab.clone()));
        let mut div2_clone = div2.clone();
        let oc_bt = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(move || {
            div2_clone.enable_this_tab();
        });

        li_clone.on_click(oc_bt);
        div2.enable_this_tab_if_first();

        let div2_clone = div2.clone();
        //self.tabs.push(div2_clone);

        let mut tab_clone = tab.clone();
        let dom_share = tab_clone.dom;
        sharegc!(dom_share, dom);
        dom.tabs.push(div2_clone);
        shares!((dom.clone()), dom_share);
        tab_clone.dom = dom_share;

        tab_clone
            .child_1
            .unwrap()
            .append_child(&li_clone.element)
            .unwrap();

        div2
    }
}
/*
func (dom *Dom) Tabcontent(tab Elem, id string, title string) Elem {
    li := dom.newElem("", "li")
    a := dom.newElem("", "a")
    //bt.jsValue.Call("setAttribute", "class", "tablinks outline secondary")
    li.Append(a)
    a.SetInnerText(title)
    div2 := dom.newElem(id, "div")
    div2.jsValue.Call("setAttribute", "class", "tabcontent")
    div2.child1 = li.jsValue
    li.OnClick(func() {
        div2.enableThisTab()
    })
    div2.enableThisTabIfFirst()
    //save Id
    dom.tabs = append(dom.tabs, div2)
    tab.child1.Call("appendChild", li.jsValue)

    return div2
}

*/

#[cfg(feature = "bulma")]
impl Elem {
    pub fn enable_this_tab(&mut self) {
        let self_clone = self.clone();
        let parent = self_clone.parent.unwrap(); // .
        let dom_share = parent.dom;
        sharegc!(dom_share, dom);
        let tabs = dom.tabs;

        self.element
            .set_attribute("style", "display: block;")
            .unwrap();
        self_clone
            .child_1
            .unwrap()
            .set_attribute("class", "is-active")
            .unwrap();

        for tab in tabs {
            if tab.element.id() != self.element.id() {
                tab.element
                    .set_attribute("style", "display: none;")
                    .unwrap();
                tab.child_1.unwrap().set_attribute("class", "").unwrap();
            }
        }
    }

    pub fn enable_this_tab_if_first(&mut self) {
        let self_clone = self.clone();
        let parent = self_clone.parent.unwrap();
        let dom_share = parent.dom;
        sharegc!(dom_share, dom);
        let tabs = dom.tabs;

        if tabs.len() == 0 {
            self.element
                .set_attribute("style", "display: block;")
                .unwrap();
            self_clone
                .child_1
                .unwrap()
                .set_attribute("class", "is-active")
                .unwrap();
        } else {
            self.element
                .set_attribute("style", "display: none;")
                .unwrap();
            self_clone
                .child_1
                .unwrap()
                .set_attribute("class", "")
                .unwrap();
        }
    }

    pub fn set_elem_size(&mut self) {
        let mut c_cur = String::new();
        let mut c_new = String::new();
        c_cur = match self.element.get_attribute("class") {
            Some(c) => c,
            None => "".to_string(),
        };
        if c_cur.len() > 0 {
            c_new = format!("{} is-size-{}", c_cur, SIZE);
        } else {
            c_new = format!("is-size-{}", SIZE);
        }

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

    pub fn tab(&mut self, id: &str) -> Elem {
        let e = self.create_element(id, "div");
        e.element.set_attribute("class", "tab").unwrap();
        e
    }

    pub fn tab_content(&mut self, tab: &mut Elem, id: &str, title: &str) -> Elem {
        let bt = self.create_element("", "button");
        let bt_clone = bt.clone();
        bt.element
            .set_attribute("class", "tablinks secondary")
            .unwrap();
        bt.element.set_inner_html(title);
        tab.append(&bt);

        let mut div2 = self.create_element(id, "div");
        div2.element.set_attribute("class", "tabcontent").unwrap();
        div2.child_1 = Some(bt.element);
        div2.parent = Some(Box::new(tab.clone()));
        let mut div2_clone = div2.clone();
        let oc_bt = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(move || {
            div2_clone.enable_this_tab();
        });

        bt_clone.on_click(oc_bt);
        div2.enable_this_tab_if_first();

        let div2_clone = div2.clone();
        //self.tabs.push(div2_clone);

        let mut tab_clone = tab.clone();
        let dom_share = tab_clone.dom;
        sharegc!(dom_share, dom);
        dom.tabs.push(div2_clone);
        shares!((dom.clone()), dom_share);
        tab_clone.dom = dom_share;

        div2
    }
}

#[cfg(feature = "pico")]
impl Elem {
    pub fn enable_this_tab(&mut self) {
        let self_clone = self.clone();
        let parent = self_clone.parent.unwrap(); // .
        let dom_share = parent.dom;
        sharegc!(dom_share, dom);
        let tabs = dom.tabs;

        self.element
            .set_attribute("style", "display: block;")
            .unwrap();
        self_clone
            .child_1
            .unwrap()
            .set_attribute("class", "tablinks active")
            .unwrap();

        for tab in tabs {
            if tab.element.id() != self.element.id() {
                tab.element
                    .set_attribute("style", "display: none;")
                    .unwrap();
                tab.child_1
                    .unwrap()
                    .set_attribute("class", "tablinks secondary")
                    .unwrap();
            }
        }
    }

    pub fn enable_this_tab_if_first(&mut self) {
        let self_clone = self.clone();
        let parent = self_clone.parent.unwrap();
        let dom_share = parent.dom;
        sharegc!(dom_share, dom);
        let tabs = dom.tabs;

        if tabs.len() == 0 {
            self.element
                .set_attribute("style", "display: block;")
                .unwrap();
            self_clone
                .child_1
                .unwrap()
                .set_attribute("class", "tablinks active")
                .unwrap();
        } else {
            self.element
                .set_attribute("style", "display: none;")
                .unwrap();
            self_clone
                .child_1
                .unwrap()
                .set_attribute("class", "tablinks secondary")
                .unwrap();
        }
    }

    pub fn set_elem_size(&mut self) {}
}

//// wasm-pack build --target web
//// wasm-pack build --debug --target web

#[macro_use]
extern crate mutils;

mod structs;

use structs::*;

use serde_yml;

use wasm_bindgen::prelude::*;
use web_sys::{
    js_sys::{self, JsString},
    window, Document, Element, HtmlInputElement, HtmlSelectElement, MessageEvent,
    WebSocket, Window,
};

#[derive(Clone)]
struct DomCfg {
    wind: Window,
    doc: Document,
    id_count: i32,
    tabs: vt!(Elem),
}

#[derive(Clone)]
struct Elem {
    element: Element,
    style: String,
    child_1: Option<Element>,
    parent: Option<Box<Elem>>,
    ws: Option<WebSocket>,
    dom: sharet!(DomCfg),
}

impl Elem {
    fn append(&self, to_append: &Elem) {
        self.element.append_child(&to_append.element).unwrap();
    }

    fn set_background_color(&mut self, color: &str) {
        self.style = format!("{} background-color: {};", &self.style, color);
        self.element.set_attribute("style", &self.style).unwrap();
    }

    fn set_color(&mut self, color: &str) {
        self.style = format!("{} color: {};", &self.style, color);
        self.element.set_attribute("style", &self.style).unwrap();
    }

    fn on_click(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn on_change(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn on_submit(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("submit", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn on_keypress(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn on_input(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("input", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn on_blur(&self, closure: Closure<dyn FnMut()>) {
        self.element
            .add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();
    }

    fn set_multi_cols(&mut self, cols_nr: i32) {
        self.style = format!("{} float: {};;", self.style, "left");
        self.style = format!("{} width: {}{};", self.style, 100 / cols_nr, '%');
        self.element.set_attribute("style", &self.style).unwrap();
    }

    fn value(&self) -> String {
        let self_clone = self.clone();
        match self_clone.child_1 {
            Some(child) => return child.dyn_into::<HtmlInputElement>().unwrap().value(),
            None => {
                return self_clone
                    .element
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value()
            }
        };
    }

    fn select_value(&self) -> String {
        let self_clone = self.clone();
        return self_clone
            .element
            .dyn_into::<HtmlSelectElement>()
            .unwrap()
            .value();
    }

    fn add_websocket(&mut self) {
        let id = self.element.get_attribute("id").unwrap();
        //let window = web_sys::window().expect("no global `window` exists");
        //let location = window.location();
        //let host = location.hostname().unwrap();
        //let port = location.port().unwrap();
        //let addr = format!(r#"ws://{}:{}/{}"#, host, port, id);

        let id_num_s = id.split_once("id_").unwrap().1;
        let id_num = id_num_s.parse::<u8>().unwrap();
        let addr = format!(r#"ws://127.0.0.1:61{:0>3}"#, id_num);

        let ws = web_sys::WebSocket::new(&addr).unwrap();
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        c_log(&ws.url()); //TODO
        self.ws = Some(ws);
    }

    fn ws_read_conf(&mut self) {
        let this = self.clone();
        let ws: WebSocket = self.ws.clone().unwrap();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                c_log(&format!("message event, received arraybuffer: {:?}", abuf));
                let array = js_sys::Uint8Array::new(&abuf);
                /*
                let len = array.byte_length() as usize;

                c_log(&format!(
                    "Arraybuffer received {}bytes: {:?}",
                    len,
                    array.to_vec()
                ));
                */
                let yaml = array.to_vec();

                let gui_descr: vt!(GuiDescr) = match serde_yml::from_slice(&yaml) {
                    Ok(gd) => gd,
                    Err(err) => {
                        c_log(&format!("GuiDescr Err {:?}", err));
                        vec![]
                    }
                };
                sharegc!((this.dom), dom);
                let body = dom.doc.body().unwrap();

                let mut tab_valid = false;
                let tb = dom.tab("");

                for tabs in gui_descr {
                    let mut tb_clone = tb.clone();
                    let tbc = dom.tab_content(&mut tb_clone, "", "");

                    if tabs.tab.id != None && !tab_valid {
                        body.append_child(&tb.element).unwrap();
                        tab_valid = true;
                    }
                    let tbc_clone = tbc.clone();
                    if tabs.tab.id != None {
                        tbc_clone.element.set_id(&tabs.tab.id.unwrap());
                        tbc_clone
                            .child_1
                            .unwrap()
                            .set_inner_html(&tabs.tab.text.unwrap());
                    }

                    let rows = tabs.tab.rows;
                    for row in rows {
                        vi!(elems, Elem);
                        for grid in row.gridrow {
                            if grid.h2 != None {
                                let h2_conf = grid.h2.unwrap();
                                let mut h2 = dom.header2(&h2_conf.id, &h2_conf.text);
                                if h2_conf.mutable {
                                    h2.add_websocket();
                                    h2.ws_read();
                                }
                                elems.push(h2);
                            }

                            if grid.textarea != None {
                                let ta_conf = grid.textarea.unwrap();
                                let mut ta =
                                    dom.textarea(&ta_conf.id, ta_conf.lines, &ta_conf.text);
                                ta.add_websocket();
                                ta.ws_read();
                                elems.push(ta);
                            }

                            if grid.label != None {
                                let lb_conf = grid.label.unwrap();
                                let mut lb = dom.label(&lb_conf.id, &lb_conf.text);
                                if lb_conf.mutable {
                                    lb.add_websocket();
                                    lb.ws_read();
                                }
                                elems.push(lb);
                            }
                            if grid.p != None {
                                let p_conf = grid.p.unwrap();
                                let mut text = "".to_string();
                                if p_conf.text != None {
                                    text = p_conf.text.unwrap();
                                }
                                let mut id = "".to_string();
                                if p_conf.id != None {
                                    id = p_conf.id.unwrap();
                                }
                                let p = dom.paragraph(&id, &text);
                                elems.push(p);
                            }

                            if grid.dropdown != None {
                                let dd_conf = grid.dropdown.unwrap();
                                let mut dd = dom.dropdown(
                                    &dd_conf.id,
                                    dd_conf.items,
                                    dd_conf.defaultind as usize,
                                );
                                dd.add_websocket();
                                let mut dd_clone = dd.clone();
                                let oc_dd = Closure::<dyn FnMut()>::new(move || {
                                    let value = dd_clone.select_value();
                                    dd_clone.ws_send(value);
                                });
                                dd.on_change(oc_dd);
                                elems.push(dd);
                            }

                            if grid.button != None {
                                let bt_conf = grid.button.unwrap();
                                let mut bt = dom.button(&bt_conf.id, &bt_conf.text);
                                bt.add_websocket();
                                let mut bt_clone = bt.clone();
                                let oc_bt = Closure::<dyn FnMut()>::new(move || {
                                    bt_clone.ws_send("pressed".to_string());
                                });
                                bt.on_click(oc_bt);
                                elems.push(bt);
                            }

                            if grid.input != None {
                                let ip_conf = grid.input.unwrap();
                                let mut text = "".to_string();
                                if ip_conf.text != None {
                                    text = ip_conf.text.unwrap();
                                }
                                let mut ip = dom.input(&ip_conf.id, &text);
                                ip.add_websocket();
                                let mut ip_clone = ip.clone();
                                let oc_ip = Closure::<dyn FnMut()>::new(move || {
                                    //c_log("input");
                                    let value = ip_clone.value();
                                    ip_clone.ws_send(value);
                                });
                                ip.on_change(oc_ip);
                                //ip.on_blur(oc_ip);
                                elems.push(ip);
                            }

                            if grid.slider != None {
                                let sl_conf = grid.slider.unwrap();
                                let mut sl = dom.slider(
                                    &sl_conf.id,
                                    sl_conf.minmaxini[0],
                                    sl_conf.minmaxini[1],
                                    sl_conf.minmaxini[2],
                                );
                                sl.add_websocket();
                                let mut sl_clone = sl.clone();
                                let oc_sl = Closure::<dyn FnMut()>::new(move || {
                                    let value = sl_clone.value();
                                    sl_clone.ws_send(value);
                                });
                                sl.on_change(oc_sl);

                                elems.push(sl);
                            }
                        }
                        let gd = dom.gridrow(elems);
                        if tab_valid {
                            tbc.append(&gd);
                            body.append_child(&tbc.element).unwrap();
                        } else {
                            body.append_child(&gd.element).unwrap();
                        }
                    }
                }
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                c_log2(&txt);
                //console_log!("message event, received Text: {:?}", txt);
            } else {
                //console_log!("message event, received Unknown: {:?}", e.data());
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            c_log(&format!("read_conf error event: {:?}", e));
        });
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        /*
        let cloned_ws = ws.clone();
        let onopen_callback = Closure::<dyn FnMut()>::new(move || {
            c_log("socket opened");
            c_log(&format!("stat {}", cloned_ws.ready_state()));
            match cloned_ws.send_with_str("ping") {
                Ok(_) => c_log("message successfully sent"),
                Err(err) => c_log("error sending message: {:?}"), //console_log!("error sending message: {:?}", err),
            }
            // send off binary message
            match cloned_ws.send_with_u8_array(&[0, 1, 2, 3]) {
                Ok(_) => c_log("binary message successfully sent"),
                Err(err) => c_log("error sending message: {:?}"), //console_log!("error sending message: {:?}", err),
            }
        });
        ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();
        */
    }

    fn ws_read(&mut self) {
        let this = self.clone();
        let ws: WebSocket = self.ws.clone().unwrap();
        let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                c_log(&format!("message event, received arraybuffer: {:?}", abuf));
                let array = js_sys::Uint8Array::new(&abuf);
                let yaml = array.to_vec();
                let rx_msg: RxTxMessage = match serde_yml::from_slice(&yaml) {
                    Ok(rm) => rm,
                    Err(err) => {
                        c_log(&format!("RxMessage Err {:?}", err));
                        RxTxMessage::default()
                    }
                };
                if rx_msg.text.len() > 0 {
                    this.element.set_inner_html(&rx_msg.text);
                }
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                c_log2(&txt);
                //console_log!("message event, received Text: {:?}", txt);
            } else {
                //console_log!("message event, received Unknown: {:?}", e.data());
            }
        });
        ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let this2 = self.clone();
        let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
            c_log(&format!(
                "{} ws_read error event: {:?}",
                this2.element.id(),
                e
            ));
        });
        ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();
    }

    fn is_ws_open(&mut self) -> bool {
        let ws: WebSocket = self.ws.clone().unwrap();
        let status = ws.ready_state();
        if status != 1 {
            return false;
        } else {
            // WebSocket.OPEN
            return true;
        }
    }

    fn ws_send(&mut self, message: String) {
        let ws: WebSocket = self.ws.clone().unwrap();
        ws.send_with_u8_array(message.as_bytes()).unwrap();
    }

    fn enable_this_tab(&mut self) {
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

    fn enable_this_tab_if_first(&mut self) {
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
}

impl DomCfg {
    fn create_element(&mut self, id: &str, name: &str) -> Elem {
        let mut id_set = id.to_string();
        if id.len() == 0 {
            id_set = self.get_new_id();
        }
        let element = self
            .doc
            .create_element(name)
            .expect(&format!("create {} error", name));
        element.set_id(&id_set);
        let this = self.clone();
        shareable!(this, dom_share);
        Elem {
            element: element,
            style: "".to_string(),
            child_1: None,
            parent: None,
            ws: None,
            dom: dom_share,
        }
    }

    fn gridrow(&mut self, children: vt!(Elem)) -> Elem {
        let e = self.create_element("", "p");
        e.element.set_attribute("class", "grid").unwrap();
        for child in children {
            e.append(&child);
        }
        e
    }

    fn title(&mut self, id: &str, title: &str) -> Elem {
        let e = self.create_element(id, "title");
        e.element.set_inner_html(title);
        e
    }

    fn row(&mut self, id: &str) -> Elem {
        let e = self.create_element(id, "div");
        e.element.set_attribute("class", "row").unwrap();
        e
    }

    fn col(&mut self, id: &str) -> Elem {
        let e = self.create_element(id, "div");
        e.element.set_attribute("class", "column").unwrap();
        e
    }

    fn input(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "input");
        e.element.set_attribute("type", "text").unwrap();
        if text.len() > 0 {
            e.element.set_attribute("placeholder", text).unwrap();
        }
        e
    }

    fn textarea(&mut self, id: &str, lines: i32, text: &str) -> Elem {
        let mut e = self.create_element(id, "textarea");
        e.element.set_inner_html(text);
        e.element.set_attribute("type", "text").unwrap();
        e.element
            .set_attribute("rows", &format!("{}", lines))
            .unwrap();
        e.set_background_color("black");
        e.set_color("white");
        e
    }

    fn form(&mut self, id: &str) -> Elem {
        let mut f = self.create_element(id, "form");
        let e = self.create_element(id, "input");
        let b = self.create_element(id, "button");
        b.element.set_attribute("type", "submit").unwrap();
        b.element.set_inner_html("Submit");
        e.element.set_attribute("type", "text").unwrap();
        let e_clone = e.element.clone();
        f.child_1 = Some(e_clone);
        f.append(&e);
        f.append(&b);
        f
    }

    fn button(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "button");
        e.element.set_inner_html(text);
        e.element.set_attribute("type", "button").unwrap();
        e.element.set_attribute("class", "primary").unwrap();
        e
    }

    fn label(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "label");
        e.element.set_inner_html(text);
        e
    }

    fn paragraph(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "p");
        e.element.set_inner_html(text);
        e
    }

    fn header2(&mut self, id: &str, text: &str) -> Elem {
        let e = self.create_element(id, "h2");
        e.element.set_inner_html(text);
        e
    }

    fn dropdown(&mut self, id: &str, choices: vt!(String), defaultind: usize) -> Elem {
        let sel = self.create_element(id, "select");
        let op1 = self.create_element(id, "option");
        op1.element.set_attribute("value", "").unwrap();
        if choices.len() > 0 && defaultind < choices.len() {
            sel.element
                .set_attribute("aria-label", &choices[defaultind])
                .unwrap();
            op1.element.set_inner_html(&choices[defaultind]);
        }
        sel.element.append_child(&op1.element);
        for_enum!(ind, choice, choices, {
            if ind == defaultind {
                continue;
            } else {
                let op = self.create_element("", "option");
                op.element.set_inner_html(&choice);
                sel.element.append_child(&op.element);
            }
        });

        sel
    }

    fn tab(&mut self, id: &str) -> Elem {
        let e = self.create_element(id, "div");
        e.element.set_attribute("class", "tab").unwrap();
        e
    }

    fn slider(&mut self, id: &str, min: i32, max: i32, value: i32) -> Elem {
        let e = self.create_element(id, "input");
        e.element.set_attribute("type", "range").unwrap();
        e.element.set_attribute("min", &format!("{}", min)).unwrap();
        e.element.set_attribute("max", &format!("{}", max)).unwrap();
        e.element
            .set_attribute("value", &format!("{}", value))
            .unwrap();
        e
    }

    fn tab_content(&mut self, tab: &mut Elem, id: &str, title: &str) -> Elem {
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
        let oc_bt = Closure::<dyn FnMut()>::new(move || {
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

    fn get_new_id(&mut self) -> String {
        let id_str = format!("id_{}", self.id_count);
        self.id_count = self.id_count + 1;
        id_str
    }
}

fn c_log(text: &str) {
    web_sys::console::log_1(&text.into());
}

fn c_log2(text: &JsString) {
    web_sys::console::log_1(text);
}

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut dom = DomCfg {
        wind: window,
        doc: document,
        id_count: 300,
        tabs: vec![],
    };

    let mut title = dom.title("id_0", "title");
    body.append_child(&title.element).unwrap();

    title.add_websocket();
    title.ws_read_conf();

    //send to ackonwledge websocket ready
    /*
    for _ind in 0..10 {
        if title.is_ws_open() {
            c_log("6"); //prova
            title.ws_send("title".into());
            break;
        } else {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
    */

    /*
    // Manufacture the element we're gonna append
    let p1 = dom.paragraph("id_1", "Hello from Rust!");

    let h2 = dom.header2("id_2", "Hello from Rust!");
    h2.set_background_color("green");

    let r1 = dom.row("id_3");

    let mut c1 = dom.col("id_4");
    c1.set_multi_cols(3);

    let c2 = dom.col("id_5");

    let b1 = dom.button("id_1ì6", "press", "yellow");

    let mut i1 = dom.input("id_7");
    let i1_clone = i1.clone();

    let mut i1_string = String::new();
    let ic1 = Closure::<dyn FnMut()>::new(move || {
        /* */
        let value = i1_clone.value();

        let vl = value.clone();
        c_log(&vl);
        /* */
    });

    //i1.on_input(ic1);
    i1.on_keypress(ic1);

    let f1 = dom.form("id_8");
    let f1_clone = f1.clone();
    let fc1 = Closure::<dyn FnMut()>::new(move || {
        let value = f1_clone.value();

        let vl = value.clone();
        c_log(&vl);
        /* */
    });
    f1.on_submit(fc1);

    // Add click event listener to the button
    let h2_clone = h2.clone();
    let bc1 = Closure::<dyn FnMut()>::new(move || {
        h2_clone.set_background_color("blue");
    });

    b1.on_click(bc1);

    c1.append(&h2);
    c1.append(&p1);
    c2.append(&b1);
    c2.append(&i1);
    c2.append(&f1);
    r1.append(&c1);
    r1.append(&c2);

    body.append_child(&r1.element)?;
    */

    Ok(())
}

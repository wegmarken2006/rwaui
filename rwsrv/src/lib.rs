#[macro_use]
extern crate mutils;

mod structs;
pub use structs::PlotConf;
use structs::*;

use crossbeam_channel::Sender;
use serde::Serialize;
use std::io::{self, BufRead, BufReader, Write};
use std::{fs::File, io::Read};
use tungstenite::handshake::derive_accept_key;
use tungstenite::Message;
//use webview_app::application::Application;

#[derive(Clone)]
pub struct WsElem {
    tx: Option<sharet!(Sender<vt!(u8)>)>,
    pub id: String,
}

impl Default for WsElem {
    fn default() -> WsElem {
        WsElem {
            id: "".to_string(),
            tx: None,
        }
    }
}
impl WsElem {
    fn attach(&mut self) {
        uchani!(tx1, rx1, vt!(u8));
        shareable!(tx1, stx1);
        self.tx = Some(stx1);

        let id_num_s = self.id.split_once("id_").unwrap().1;
        let id_num = id_num_s.parse::<u8>().unwrap();
        let addr = format!(r#"127.0.0.1:61{:0>3}"#, id_num);
        let ws_listener = std::net::TcpListener::bind(addr.clone())
            .expect(&format!("error binding to {}", &addr));
        let addr = ws_listener.local_addr().unwrap().clone();

        //println!("attach URL {}", addr);

        let this = self.clone();
        std::thread::spawn(move || {
            for stream in ws_listener.incoming() {
                let stream = stream.unwrap();

                let mut websocket = tungstenite::accept(stream).unwrap();

                loop {
                    chan_rx!(rx1, msg, {
                        websocket.send(Message::Binary(msg)).unwrap();
                        let msg = websocket.read().unwrap_or_else(|_| {
                            std::process::exit(0);
                        });
                        //println!("WS received from {}: {}", &addr, &msg.to_string());
                    });
                }
            }
        });
    }

    fn attach_tx(&mut self) {
        uchani!(tx1, rx1, vt!(u8));
        shareable!(tx1, stx1);
        self.tx = Some(stx1);

        let id_num_s = self.id.split_once("id_").unwrap().1;
        let id_num = id_num_s.parse::<u8>().unwrap();
        let addr = format!(r#"127.0.0.1:61{:0>3}"#, id_num);
        let ws_listener = std::net::TcpListener::bind(addr.clone())
            .expect(&format!("error binding to {}", &addr));
        let addr = ws_listener.local_addr().unwrap().clone();

        //println!("attach_tx URL {}", addr);

        std::thread::spawn(move || {
            for stream in ws_listener.incoming() {
                let stream = stream.unwrap();

                let mut websocket = tungstenite::accept(stream).unwrap();

                loop {
                    chan_rx!(rx1, msg, {
                        websocket.send(Message::Binary(msg)).unwrap();
                    });
                }
            }
        });
    }

    pub fn callback(&mut self, mut f: funmt!(String)) {
        let id_num_s = self.id.split_once("id_").unwrap().1;
        let id_num = id_num_s.parse::<u8>().unwrap();
        let addr = format!(r#"127.0.0.1:62{:0>3}"#, id_num);

        let ws_listener = std::net::TcpListener::bind(addr.clone())
            .expect(&format!("error binding to {}", &addr));

        //let addr = ws_listener.local_addr().unwrap().clone();
        //println!("Callback URL {}", addr);

        std::thread::spawn(move || {
            for stream in ws_listener.incoming() {
                let stream = stream.unwrap();
                let mut websocket = tungstenite::accept(stream).unwrap();
                loop {
                    let msg = websocket.read().unwrap_or_else(|_| {
                        std::process::exit(0);
                    });
                    //println!("WS received from {}: {}", &addr, &msg.to_string());
                    if msg.len() > 0 {
                        f(msg.to_string());
                    }
                }
            }
        });
    }

    fn send(&mut self, message: vt!(u8)) {
        let stx = self.tx.clone().unwrap();
        sharegc!(stx, tx1);
        //tx.send(message.to_string()).unwrap();
        chan_tx!(tx1, message);
    }

    fn send_message(&mut self, tx_msg: RxTxMessage) {
        let mut buffer = Vec::new();
        let mut ser = serde_yml::Serializer::new(&mut buffer);
        tx_msg.serialize(&mut ser).unwrap();

        let stx = self.tx.clone().unwrap();
        sharegc!(stx, tx1);
        //tx.send(message.to_string()).unwrap();
        chan_tx!(tx1, buffer);
    }

    pub fn set_inner_text(&mut self, text: &str) {
        let mut tx_msg = RxTxMessage::default();
        tx_msg.text = text.to_string();
        self.send_message(tx_msg);
    }

    pub fn set_background_color(&mut self, color: &str) {
        let mut tx_msg = RxTxMessage::default();
        tx_msg.backgroundcolor = color.to_string();
        self.send_message(tx_msg);
    }

    pub fn set_color(&mut self, color: &str) {
        let mut tx_msg = RxTxMessage::default();
        tx_msg.color = color.to_string();
        self.send_message(tx_msg);
    }

    pub fn set_list(&mut self, list: vt!(String)) {
        let mut tx_msg = RxTxMessage::default();
        tx_msg.list = list;
        self.send_message(tx_msg);
    }

    pub fn draw_plot(&mut self, plot_conf: PlotConf) {
        let mut tx_msg = RxTxMessage::default();
        tx_msg.plot_conf = Some(plot_conf);
        self.send_message(tx_msg);
    }
}

pub fn init(yaml_name: &str) -> (String, funrt!(&str, WsElem)) {
    let mut f = File::open(yaml_name).expect(&format!("File {} not found", yaml_name));

    vi!(buf, u8);
    f.read_to_end(&mut buf).unwrap();

    let yaml = buf.to_vec();

    let mut title = WsElem::default();
    title.id = "id_0".to_string();
    title.attach();
    title.send(buf);

    let gui_descr: vt!(GuiDescr) = match serde_yml::from_slice(&yaml) {
        Ok(gd) => gd,
        Err(err) => {
            println!("GuiDescr Err {:?}", err);
            vec![]
        }
    };

    hmi!(helems, String, WsElem);
    for tabs in gui_descr {
        let rows = tabs.tab.rows;
        for row in rows {
            for grid in row.gridrow {
                if grid.h2 != None {
                    let conf = grid.h2.unwrap();
                    let id = conf.id;
                    let mut h2 = WsElem::default();
                    h2.id = id.clone();
                    if conf.mutable {
                        h2.attach_tx();
                    }
                    hms!(helems, id, h2);
                }

                if grid.textarea != None {
                    let conf = grid.textarea.unwrap();
                    let id = conf.id;
                    let mut ta = WsElem::default();
                    ta.id = id.clone();
                    ta.attach_tx();
                    hms!(helems, id, ta);
                }

                if grid.label != None {
                    let conf = grid.label.unwrap();
                    let id = conf.id;
                    let mut lb = WsElem::default();
                    lb.id = id.clone();
                    if conf.mutable {
                        lb.attach_tx();
                    }
                    hms!(helems, id, lb);
                }

                if grid.dropdown != None {
                    let conf = grid.dropdown.unwrap();
                    let id = conf.id;
                    let mut dd = WsElem::default();
                    dd.id = id.clone();
                    dd.attach(); //write and read
                    hms!(helems, id, dd);
                }

                if grid.button != None {
                    let conf = grid.button.unwrap();
                    let id = conf.id;
                    let mut bt = WsElem::default();
                    bt.id = id.clone();
                    hms!(helems, id, bt);
                }

                if grid.input != None {
                    let conf = grid.input.unwrap();
                    let id = conf.id;
                    let mut ip = WsElem::default();
                    ip.id = id.clone();
                    hms!(helems, id, ip);
                }

                if grid.date != None {
                    let conf = grid.date.unwrap();
                    let id = conf.id;
                    let mut dt = WsElem::default();
                    dt.id = id.clone();
                    hms!(helems, id, dt);
                }

                if grid.slider != None {
                    let conf = grid.slider.unwrap();
                    let id = conf.id;
                    let mut sl = WsElem::default();
                    sl.id = id.clone();
                    hms!(helems, id, sl);
                }

                if grid.plot != None {
                    let conf = grid.plot.unwrap();
                    let id = conf.id;
                    let mut plt = WsElem::default();
                    plt.id = id.clone();
                    plt.attach_tx();
                    hms!(helems, id, plt);
                }
            }
        }
    }

    let addr = start_server("pkg".to_string());

    /*
    fn on_activate(app: &Application) -> WebView {
        let webview = WebView::builder(app).title("Title 2").url(addr).build();
        webview
    }

    std::thread::spawn(move || {
        Application::new("prova").on_activate(on_activate).run();
    });
    */

    return (addr, move |id| helems[id].clone());
}

fn start_server(rel_path: String) -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("tcp server error");
    //let listener = std::net::TcpListener::bind("127.0.0.1:9001").expect("tcp server error");
    let addr = listener.local_addr().unwrap().clone();

    //let fh_name = "static/index.html";
    let fh_name = format!("{}{}", rel_path, "/index.html");
    let content =
        std::fs::read_to_string(fh_name.clone()).expect(&format!("Cannot read {}", &fh_name));

    std::thread::spawn(move || {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let buf_reader = BufReader::new(&mut stream);

            let http_request: Vec<_> = buf_reader
                .lines()
                .map(|result| result.unwrap())
                .take_while(|line| !line.is_empty())
                .collect();

            let ok_response = "HTTP/1.1 200 OK\r\n";
            let js_type = "Content-Type: text/javascript\r\n";
            let wasm_type = "Content-Type: application/wasm\r\n";

            let strs: Vec<&str> = http_request[0].split_whitespace().collect();
            println!("{:?}", strs);
            if strs[0] == "GET" {
                let head = &format!("/{}/", rel_path);
                if strs[1].starts_with(head) {
                    let path = strs[1][1..].to_string(); //cut initial /

                    let mut static_content = Vec::new();

                    let mut file = File::open(&path).expect(&format!("Unable to open {}", path));
                    file.read_to_end(&mut static_content)
                        .expect("Unable to read");

                    let mut response: String = String::new();
                    if path.ends_with(".js") {
                        response = format!("{ok_response}{js_type}\r\n");
                    } else if path.ends_with(".wasm") {
                        response = format!("{ok_response}{wasm_type}\r\n");
                    } else {
                        response = format!("{ok_response}\r\n");
                    }

                    stream.write_all(response.as_bytes()).unwrap();
                    stream.write_all(&static_content).unwrap();

                    continue;
                } else if strs[1].contains("/id_") {
                    //println!("{:?}", http_request);

                    let key_header = "Sec-WebSocket-Key: ";

                    // Given the request we find the line starting the the key_header and then find the
                    // key sent from the client.
                    let request_key = http_request
                        .iter()
                        .find(|line| line.starts_with(key_header))
                        .map(|line| line[key_header.len()..].trim())
                        .unwrap();

                    let header_key = derive_accept_key(request_key.as_bytes());
                    let response = format!(
                        "HTTP/1.1 101 Switching Protocols\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\nUpgrade: websocket\r\n\r\n",
                        header_key
                    );

                    stream.write_all(response.as_bytes()).unwrap();
                    //println!("RESP\r\n{response}");

                    continue;
                }
            }

            let response = format!("{}\r\n{}", ok_response, content);

            stream.write_all(response.as_bytes()).unwrap();
        }
    });

    let addr_str = format!("http://{:?}", addr);

    return addr_str;
}

pub fn wait_key_from_console() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    println!("Press:\n q<Enter> to exit");

    loop {
        let mut text = String::new();
        reader.read_line(&mut text).unwrap();

        match text.trim() {
            "q" | "Q" => std::process::exit(0),
            _ => continue,
        }
    }
}

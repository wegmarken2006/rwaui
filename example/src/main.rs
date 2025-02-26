use rwsrv::*;

fn main() {
    let (addr, get_elem) = init("config_tabs.yaml");

    let args =  std::env::args();
    let mut wv = false;
    if args.len() > 0 {
        for arg in args {
            if arg == "wv" {
                wv = true;
            }
        }
    }

    let h2 = get_elem("id_7");
    let mut bt2 = get_elem("id_2");
    let mut h2_clone = h2.clone();
    bt2.callback(move |_| {
        h2_clone.set_inner_text("new header");
        h2_clone.set_background_color("red");
        h2_clone.set_color("white");
    });

    let mut dt1 = get_elem("id_30");
    dt1.callback(move |val| println!("dt1 val {}", val));

    let mut bt21 = get_elem("id_21");
    bt21.callback(move |val| println!("bt21 val {}", val));

    let mut dd = get_elem("id_1");
    dd.callback(move |val| println!("dd val {}", val));

    let mut sl = get_elem("id_4");
    let lb1 = get_elem("id_6");
    let mut lb1_clone = lb1.clone();
    sl.callback(move |val| {
        println!("sl val {}", val);
        lb1_clone.set_inner_text(&val);
    });

    let mut ip1 = get_elem("id_3");
    ip1.callback(move |val| println!("ip1 val {}", val));


    if wv {
        web_view::builder()
        .title("Test")
        .content(web_view::Content::Url(&addr))
        .size(1200, 1000)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
    } else {
        println!("Serving on {}", &addr);
        wait_key_from_console();
    }

}

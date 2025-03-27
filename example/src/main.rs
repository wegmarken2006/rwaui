use rwsrv::*;

fn main() {
    let (addr, get_elem) = init("config_tabs.yaml");

    let args = std::env::args();
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
    let list = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    dd.set_list(list);
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

    let mut plt1 = get_elem("id_47");
    let mut plt2 = get_elem("id_48");
    let mut plt3 = get_elem("id_49");
    let mut plt4 = get_elem("id_50");

    let layout = PlotLayout {
        title: "test".to_string(),
        width: 400,
        height: 400,
    };

    let y1 = vec![1.0, 2.0, 4.0, 8.0, 16.0];
    let y2 = vec![2.0, 4.0, 8.0, 16.0, 32.0];
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let mut y: Vec<Vec<f64>> = vec![];
    let names = vec!["uno".to_string(), "due".to_string()];
    y.push(y1.clone());
    y.push(y2.clone());
    let xs2 = vec![
        "aa".to_string(),
        "bb".to_string(),
        "cc".to_string(),
        "dd".to_string(),
        "ee".to_string(),
    ];

    plt1.draw_plot_scatter(x.clone(), y.clone(), names.clone(), layout.clone());

    plt2.draw_plot_bars(xs2, y.clone(), names.clone(), layout.clone());

    plt3.draw_plot_lines(x, y.clone(), names.clone(), layout.clone());

    plt4.draw_plot_box(y, names, layout);

    //
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

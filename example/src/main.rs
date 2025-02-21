use rwsrv::*;

fn main() {
    let get_elem = init("config_tabs.yaml");

    let h2 = get_elem("id_7");
    let mut bt = get_elem("id_2");
    let mut h2_clone = h2.clone();
    bt.callback(move |_| h2_clone.set_inner_text("new header"));

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

    wait_key_from_console();
}

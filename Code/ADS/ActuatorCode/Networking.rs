// WiFi networking functions for actuator control
// Note: these reference global state from the main actuator code

fn wifi_setup(ssid: &str, pass: &str) {
    wifi_begin(ssid, pass);
    let ip = wifi_local_ip();  //get the IP
    let mut vip = wifi_local_ip();  //get the IP
    println!("{}", ip);

    while ip == vip {
        wifi_begin(ssid, pass);
        vip = wifi_local_ip();  //get the IP
        println!("{}", vip);
        if vip != ip {
            break;
        }
    }

    server_begin();
}

fn wifi_loop(
    paused: &mut bool,
    a_val: &mut i32,
    b_val: &mut i32,
    st: &mut i32,
    count: &mut i32,
    count_r: &mut i32,
    cycle_test: &mut bool,
) {
    while let Some((data, client)) = server_accept() {
        command(data, client, paused, a_val, b_val, st, count, count_r, cycle_test);
    }
}

fn command(
    data: char,
    mut client: WifiClient,
    paused: &mut bool,
    a_val: &mut i32,
    b_val: &mut i32,
    st: &mut i32,
    count: &mut i32,
    count_r: &mut i32,
    cycle_test: &mut bool,
) {
    if data == 'p' {
        *paused = !*paused;
    } else if data == 's' {
        //prepare to receive 3 value updates: A B and t
        *a_val = get_int(&mut client);
        print!("aVal set to: ");
        println!("{}", a_val);

        *b_val = get_int(&mut client);
        print!("bVal set to: ");
        println!("{}", b_val);

        *st = get_int(&mut client);
        print!("delay set to: ");
        *st = *st * 1000;
        println!("{}", st);
    } else if data == 'r' {
        *count = 0;  //set counter to 0
        *count_r = 0;
    } else if data == 'l' {
        *cycle_test = !*cycle_test;
    } else if data == 'a' {
        //set aVal to incoming integer
        *a_val = get_int(&mut client);
        print!("aVal set to: ");
        println!("{}", a_val);
        *cycle_test = false;
    } else if data == 'b' {
        //set bVal to incoming integer
        *b_val = get_int(&mut client);
        print!("bVal set to: ");
        println!("{}", b_val);
        *cycle_test = false;
    }
}

fn get_int(client: &mut WifiClient) -> i32 {
    let mut temp: i32 = -1;
    while temp == -1 {
        temp = client.read();
    }
    temp
}

fn send_update(count: i32) {
    if let Some(mut client) = server_available() {
        if client.connected() {
            client.write_byte(b'c');
            client.write_int(count);
        }
    }
}

struct WifiClient;
impl WifiClient {
    fn read(&mut self) -> i32 { -1 }
    fn connected(&self) -> bool { false }
    fn write_byte(&mut self, _b: u8) {}
    fn write_int(&mut self, _val: i32) {}
}
fn wifi_begin(_ssid: &str, _pass: &str) {}
fn wifi_local_ip() -> String { String::from("0.0.0.0") }
fn server_begin() {}
fn server_accept() -> Option<(char, WifiClient)> { None }
fn server_available() -> Option<WifiClient> { None }

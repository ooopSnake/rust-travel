#[test]
fn test() {
    let html_str = reqwest::blocking::get("https://ipinfo.io/ip")
        .unwrap()
        .text()
        .unwrap();
    println!("ipinfo: {}", html_str);
}


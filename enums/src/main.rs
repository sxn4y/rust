enum IpAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

impl IpAddr {
    fn display(&self) {
        match self {
            IpAddr::V4(a, b, c, d) => {
                println!("IPv4 Address: {}.{}.{}.{}", a, b, c, d);
            }
            IpAddr::V6(address) => {
                println!("IPv6 Address: {}", address);
            }
        }
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    home.display();
    loopback.display();
}
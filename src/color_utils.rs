pub struct Colors {
    pub HEADER: &'static str,
    pub OKBLUE: &'static str,
    pub OKGREEN: &'static str,
    pub WARNING: &'static str,
    pub FAIL: &'static str,
    pub ENDC: &'static str,
    pub BOLD: &'static str,
    pub UNDERLINE: &'static str,
}

impl Colors {
    pub fn new() -> Colors {
        Colors {
            HEADER: "\u{001b}[95m",
            OKBLUE: "\u{001b}[94m",
            OKGREEN: "\u{001b}[92m",
            WARNING: "\u{001b}[93m",
            FAIL: "\u{001b}[91m",
            ENDC: "\u{001b}[0m",
            BOLD: "\u{001b}[1m",
            UNDERLINE: "\u{001b}[4m",
        }
    }
}

pub fn bprint(s: &str, m: &str) {
    let b = Colors::new();
    println!("{}{}{}", b.BOLD, b.OKBLUE, s);
    println!("{}", m);
    println!("{}", b.ENDC);
}

pub fn gprint(s: &str, m: &str) {
    let b = Colors::new();
    println!("{}{}{}", b.BOLD, b.OKGREEN, s);
    println!("{}", m);
    println!("{}", b.ENDC);
}

pub fn rprint(s: &str) {
    let b = Colors::new();
    println!("{}{}{}", b.BOLD, b.FAIL, s);
    println!("{}", b.ENDC);
}

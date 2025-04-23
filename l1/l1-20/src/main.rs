#![allow(clippy::upper_case_acronyms)]

/// USD $ — US Dollar
#[derive(Debug)]
struct USD(f64);

impl USD {
    pub fn new(v: f64) -> Self {
        USD(v)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn to_krw(&self) -> KRW {
        let rate = 1_335.065_3;
        KRW::new(self.0 * rate)
    }

    pub fn to_jpy(&self) -> JPY {
        let rate = 143.86853;
        JPY::new(self.0 * rate)
    }

    pub fn to_cny(&self) -> CNY {
        let rate = 7.0537183;
        CNY::new(self.0 * rate)
    }
}

/// KRW ₩ — South Korean Won
#[derive(Debug)]
struct KRW(f64);

impl KRW {
    pub fn new(v: f64) -> Self {
        KRW(v)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn to_usd(&self) -> USD {
        let rate = 0.00074902642;
        USD::new(self.0 * rate)
    }

    pub fn to_jpy(&self) -> JPY {
        let rate = 0.10776616;
        JPY::new(self.0 * rate)
    }

    pub fn to_cny(&self) -> CNY {
        let rate = 0.0052834214;
        CNY::new(self.0 * rate)
    }
}

/// JPY ¥ — Japanese yen
#[derive(Debug)]
struct JPY(f64);

impl JPY {
    pub fn new(v: f64) -> Self {
        JPY(v)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn to_usd(&self) -> USD {
        let rate = 0.0069504789;
        USD::new(self.0 * rate)
    }

    pub fn to_krw(&self) -> KRW {
        let rate = 9.2793508;
        KRW::new(self.0 * rate)
    }

    pub fn to_cny(&self) -> CNY {
        let rate = 0.04902672;
        CNY::new(self.0 * rate)
    }
}

/// CNY ¥ — Chinese Yuan Renminbi
#[derive(Debug)]
struct CNY(f64);

impl CNY {
    pub fn new(v: f64) -> Self {
        CNY(v)
    }

    pub fn get(&self) -> f64 {
        self.0
    }

    pub fn to_usd(&self) -> USD {
        let rate = 0.1417692;
        USD::new(self.0 * rate)
    }

    pub fn to_krw(&self) -> KRW {
        let rate = 189.27129;
        KRW::new(self.0 * rate)
    }

    pub fn to_jpy(&self) -> JPY {
        let rate = 20.397041;
        JPY::new(self.0 * rate)
    }
}

fn main() {
    let usd = USD::new(5.0);
    let krw = KRW::new(5.0);
    let jpy = JPY::new(5.0);
    let cny = CNY::new(5.0);

    println!("$");
    println!("{}", usd.get());
    println!("{}", usd.to_krw().get());
    println!("{}", usd.to_cny().get());
    println!("{}", usd.to_jpy().get());

    println!("₩");
    println!("{}", krw.get());
    println!("{}", krw.to_usd().get());
    println!("{}", krw.to_cny().get());
    println!("{}", krw.to_jpy().get());

    println!("¥");
    println!("{}", jpy.get());
    println!("{}", jpy.to_usd().get());
    println!("{}", jpy.to_cny().get());
    println!("{}", jpy.to_krw().get());

    println!("¥");
    println!("{}", cny.get());
    println!("{}", cny.to_usd().get());
    println!("{}", cny.to_krw().get());
    println!("{}", cny.to_jpy().get());

    println!("¥ -> ₩ -> ¥");
    println!("{}", cny.to_krw().to_jpy().get());
}

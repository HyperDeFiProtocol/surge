use std::fs;
use regex::Regex;

mod subscribe;
mod source;

pub struct Proxy {
    name: String,
    text: String,
}

struct Proxies {
    name: String,
    list: Vec<Proxy>,
}

impl Proxy {
    pub fn new(name: &str, text: &str) -> Self {
        let re = Regex::new(r"(\s+)").unwrap();

        Proxy {
            name: re.replace_all(name.trim(), " ").into(),
            text: re.replace_all(text.trim(), " ").into(),
        }
    }
}

impl Proxies {
    pub fn new(name: &str) -> Self {
        Proxies {
            name: String::from(name),
            list: Vec::<Proxy>::new(),
        }
    }

    pub fn export_list(&self) -> String {
        let mut vec: Vec<String> = Vec::<String>::new();
        for proxy in &self.list {
            vec.push(String::from(&proxy.text));
        }

        vec.join("\n")
    }

    pub fn export_test(&self) -> String {
        let mut vec: Vec<String> = Vec::<String>::new();

        vec.push(String::from(&self.name) + &String::from(" = url-test"));

        for proxy in &self.list {
            vec.push(String::from(&proxy.name));
        }

        vec.push(String::from("url = http://cp.cloudflare.com/generate_204"));
        vec.push(String::from("interval = 3600"));
        vec.push(String::from("tolerance = 100"));

        vec.join(", ")
    }

    pub fn ready(&self) -> bool {
        self.list.len() > 0
    }
}

fn key(s: &str) -> &str {
    let vec: Vec<&str> = s.split("=").collect();

    vec[0].trim()
}


fn proxy_group_select(hint: &str, vec: Vec<&Proxies>) -> String {
    let mut list: Vec<String> = Vec::<String>::new();

    list.push(String::from(hint));

    for proxies in &vec {
        if proxies.ready() {
            list.push(String::from(&proxies.name));
        }
    }

    list.join(", ")
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start...");

    let config_general: String = source::load("0.general.conf");
    let config_replica: String = source::load("1.replica.conf");
    let config_rule: String = source::load("2.rule.conf");
    let config_host: String = source::load("3.host.conf");
    let config_url_rewrite: String = source::load("4.url-rewrite.conf");
    let config_mitm: String = source::load("5.mitm.conf");
    let config_script: String = source::load("6.script.conf");

    let mut proxies: Vec<Proxy> = Vec::<Proxy>::new();
    for s in subscribe::get_nodes() {
        proxies.push(Proxy::new(key(&s), &s));
    }

    if proxies.len() == 0 {
        println!("No proxies loaded...");
        std::process::exit(0);
    }

    let mut hyprid_connect:Proxies = Proxies::new("??????? HC");
    let mut hkg_iepl:Proxies = Proxies::new("???????? ?????? IEPL");
    let mut hkg:Proxies = Proxies::new("???????? ??????");
    let mut sgp:Proxies = Proxies::new("???????? ?????????");
    let mut twn:Proxies = Proxies::new("???????? ??????");
    let mut jpn:Proxies = Proxies::new("???????? ??????");
    let mut kor:Proxies = Proxies::new("???????? ??????");
    let mut ind:Proxies = Proxies::new("???????? ??????");
    let mut usa:Proxies = Proxies::new("???????? ??????");
    let mut can:Proxies = Proxies::new("???????? ?????????");
    let mut aus:Proxies = Proxies::new("???????? ????????????");
    let mut phl:Proxies = Proxies::new("???????? ?????????");
    let mut gbr:Proxies = Proxies::new("???????? ??????");
    let mut deu:Proxies = Proxies::new("???????? ??????");
    let mut bra:Proxies = Proxies::new("???????? ??????");
    let mut otr:Proxies = Proxies::new("??????");

    for proxy in proxies {
        if proxy.name.contains("??????") && proxy.name.contains("IEPL") {
            hkg_iepl.list.push(proxy);
        } else if proxy.name.contains("HC") || proxy.name.contains("HA") {
            hyprid_connect.list.push(proxy);
        } else if proxy.name.contains("??????") {
            hkg.list.push(proxy);
        } else if proxy.name.contains("?????????") {
            sgp.list.push(proxy);
        } else if proxy.name.contains("??????") {
            twn.list.push(proxy);
        } else if proxy.name.contains("??????") {
            jpn.list.push(proxy);
        } else if proxy.name.contains("??????") {
            kor.list.push(proxy);
        } else if proxy.name.contains("??????") {
            ind.list.push(proxy);
        } else if proxy.name.contains("??????") {
            usa.list.push(proxy);
        } else if proxy.name.contains("?????????") {
            can.list.push(proxy);
        } else if proxy.name.contains("????????????") {
            aus.list.push(proxy);
        } else if proxy.name.contains("?????????") {
            phl.list.push(proxy);
        } else if proxy.name.contains("??????") {
            gbr.list.push(proxy);
        } else if proxy.name.contains("??????") {
            deu.list.push(proxy);
        } else if proxy.name.contains("??????") {
            bra.list.push(proxy);
        } else {
            otr.list.push(proxy);
        }
    }


    let mut vec_proxy: Vec<String> = Vec::<String>::new();
    let mut vec_proxy_group_url_test: Vec<String> = Vec::<String>::new();

    let vec_proxies: Vec<&Proxies> = vec![
        &hkg_iepl,
        &hyprid_connect,
        &hkg,
        &sgp,
        &twn,
        &jpn,
        &kor,
        &ind,
        &usa,
        &can,
        &aus,
        &phl,
        &gbr,
        &deu,
        &bra,
        &otr
    ];

    for proxies in &vec_proxies {
        if proxies.ready() {
            vec_proxy.push(proxies.export_list());
            vec_proxy_group_url_test.push(proxies.export_test());
        }
    }

    //
    // configs text
    //
    let mut configs: Vec<String> = Vec::<String>::new();

    // [General]
    configs.push(config_general);

    // [Replica]
    configs.push(config_replica);

    // [Proxy]
    configs.push(String::from("[Proxy]"));
    configs.push(String::from("Direct = direct"));
    configs.push(String::from("Reject = reject"));
    configs.push(String::from("\n"));
    configs.push(vec_proxy.join("\n"));
    configs.push(String::from("\n"));

    // [Proxy Group]
    configs.push(String::from("[Proxy Group]"));

    // [Proxy Group] :: url-test
    configs.push(vec_proxy_group_url_test.join("\n"));
    configs.push(String::from("\n"));

    // [Proxy Group] :: select
    configs.push(proxy_group_select("Proxy = select", vec_proxies.clone()));
    configs.push(String::from("Domestic = select, Direct, Proxy"));
    configs.push(proxy_group_select("Others = select, Proxy, Direct, Domestic", vec_proxies.clone()));
    configs.push(proxy_group_select("Apple = select, Direct, Proxy", vec_proxies.clone()));
    configs.push(proxy_group_select("Scholar = select, Direct, Proxy", vec_proxies.clone()));
    configs.push(proxy_group_select("AsianTV = select, Direct, Domestic, Proxy", vec_proxies.clone()));
    configs.push(proxy_group_select("GlobalTV = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("Netflix = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("Spotify = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("YouTube = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("Disney = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("Telegram = select, Proxy, Direct", vec_proxies.clone()));
    configs.push(proxy_group_select("Steam = select, Proxy, Direct, Domestic", vec_proxies.clone()));
    configs.push(proxy_group_select("Speedtest = select, Proxy, Direct, Domestic", vec_proxies.clone()));
    configs.push(proxy_group_select("PayPal = select, Proxy, Direct, Domestic", vec_proxies.clone()));
    configs.push(proxy_group_select("Microsoft = select, Domestic, Proxy, Direct", vec_proxies.clone()));
    configs.push(String::from("\n"));

    // [Rule]
    configs.push(config_rule);

    // [Host]
    configs.push(config_host);

    // [URL Rewrite]
    configs.push(config_url_rewrite);

    // [MITM]
    configs.push(config_mitm);

    // [Script]
    configs.push(config_script);

    // remove duplicated blank lines
    let re = Regex::new(r"\n{3,}").unwrap();
    let output: String = re.replace_all(&configs.join("\n"), "\n\n").into();

    // output
    print!("Write to `./output/auto.conf`... ");
    match fs::write("./output/auto.conf", &output) {
        Err(_err) => {
            println!("Failed!\n\n");
            println!("{}", _err);
            std::process::exit(0);
        },
        Ok(_) => {
            println!("Success!");
        }
    }

    Ok(())
}

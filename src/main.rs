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

    let mut hyprid_connect:Proxies = Proxies::new("🛣️ HC");
    let mut hkg_iepl:Proxies = Proxies::new("🇭🇰 香港 IEPL");
    let mut hkg:Proxies = Proxies::new("🇭🇰 香港");
    let mut sgp:Proxies = Proxies::new("🇸🇬 新加坡");
    let mut twn:Proxies = Proxies::new("🇨🇳 台湾");
    let mut jpn:Proxies = Proxies::new("🇯🇵 日本");
    let mut kor:Proxies = Proxies::new("🇰🇷 韩国");
    let mut ind:Proxies = Proxies::new("🇮🇳 印度");
    let mut usa:Proxies = Proxies::new("🇺🇸 美国");
    let mut can:Proxies = Proxies::new("🇨🇦 加拿大");
    let mut aus:Proxies = Proxies::new("🇦🇺 澳大利亚");
    let mut phl:Proxies = Proxies::new("🇵🇭 菲律宾");
    let mut gbr:Proxies = Proxies::new("🇬🇧 英国");
    let mut deu:Proxies = Proxies::new("🇩🇪 德国");
    let mut bra:Proxies = Proxies::new("🇧🇷 巴西");
    let mut otr:Proxies = Proxies::new("其它");

    for proxy in proxies {
        if proxy.name.contains("香港") && proxy.name.contains("IEPL") {
            hkg_iepl.list.push(proxy);
        } else if proxy.name.contains("HC") || proxy.name.contains("HA") {
            hyprid_connect.list.push(proxy);
        } else if proxy.name.contains("香港") {
            hkg.list.push(proxy);
        } else if proxy.name.contains("新加坡") {
            sgp.list.push(proxy);
        } else if proxy.name.contains("台湾") {
            twn.list.push(proxy);
        } else if proxy.name.contains("日本") {
            jpn.list.push(proxy);
        } else if proxy.name.contains("韩国") {
            kor.list.push(proxy);
        } else if proxy.name.contains("印度") {
            ind.list.push(proxy);
        } else if proxy.name.contains("美国") {
            usa.list.push(proxy);
        } else if proxy.name.contains("加拿大") {
            can.list.push(proxy);
        } else if proxy.name.contains("澳大利亚") {
            aus.list.push(proxy);
        } else if proxy.name.contains("菲律宾") {
            phl.list.push(proxy);
        } else if proxy.name.contains("英国") {
            gbr.list.push(proxy);
        } else if proxy.name.contains("德国") {
            deu.list.push(proxy);
        } else if proxy.name.contains("巴西") {
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
    fs::write("./output/auto.conf", &output)?;
    println!("{}", output);

    Ok(())
}

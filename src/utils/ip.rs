use actix_web::HttpRequest;

pub async fn header2ip(req: &HttpRequest) -> String {
    if let Some(cf_connecting_ip) = req.headers().get("cf-connecting-ip") {
        if let Ok(ip) = cf_connecting_ip.to_str() {
            return ip.to_string();
        }
    }
    if let Some(cf_pseudo_ipv4) = req.headers().get("cf-pseudo-ipv4") {
        if let Ok(ip) = cf_pseudo_ipv4.to_str() {
            return ip.to_string();
        }
    }
    req.peer_addr().unwrap().to_string()
}

pub async fn tor_check(req: &HttpRequest) -> bool {
    if let Some(cf_ipcountry) = req.headers().get("cf-ipcountry") {
        if let Ok(country) = cf_ipcountry.to_str() {
            return country == "T1";
        }
    }

    false
}

use std::collections::HashMap;

pub fn parse_params_to_str(map: &HashMap<&str, &str>) -> anyhow::Result<String> {
    let mut url = String::from("?");
    for (k, v) in map {
        url.push_str(k);
        url.push_str("=");
        url.push_str(v);
        url.push_str("&");
    }
    let len = url.len();
    url.remove(len-1);

    Ok(url)
}
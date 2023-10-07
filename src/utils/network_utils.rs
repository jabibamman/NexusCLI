use curl::easy::Easy;

pub fn execute_curl_request(url: &str, method: &str, proxy: Option<&str>) -> Result<(), curl::Error> {
    let mut easy = Easy::new();
    easy.url(url)?;
    easy.custom_request(method)?;

    if let Some(proxy) = proxy {
        easy.proxy(proxy)?;
    }

    match easy.perform() {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}

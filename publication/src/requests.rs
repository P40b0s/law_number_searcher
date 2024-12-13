// use hyper::{body::Bytes, header::{self, HeaderName, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CONNECTION, HOST, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT}, Request, Uri};
// use utilites::http::{to_body, BoxBody};
// ///Хост прописываем отдельно, так как он из ури берется без порта
// pub fn empty_get_request(uri: Uri) -> Request<BoxBody>
// {
//     let host = [uri.host().unwrap(), uri.port().unwrap().as_str()].concat();
//     Request::builder()
//     .method("GET")
//     .uri(&uri)
//     .header(HOST, "publication.pravo.gov.ru")
//     .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0")
//     .header(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
//     .header(ACCEPT_ENCODING, "gzip, deflate")
//     .header(ACCEPT_LANGUAGE, "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3")
//     //.header(CONNECTION, "keep-alive")
//     .header(REFERER, ["http:://publication.pravo.gov.ru", uri.path()].concat())
//     .header(UPGRADE_INSECURE_REQUESTS, 1)
//     .header("Priority", "u=1")
//     .body(to_body(Bytes::new()))
//     .unwrap()
// }

// pub fn standart_headers() -> Vec<(HeaderName, &'static str)>
// {
//     let mut headers: Vec<(HeaderName, &'static str)> = Vec::with_capacity(5);
//     headers.push((USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:127.0) Gecko/20100101 Firefox/127.0"));
//     headers.push((ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"));
//     headers.push((ACCEPT_ENCODING, "gzip, deflate"));
//     headers.push((ACCEPT_LANGUAGE, "ru-RU,ru;q=0.8,en-US;q=0.5,en;q=0.3"));
//     //headers.push((CONNECTION, "keep-alive"));
//     headers
// }
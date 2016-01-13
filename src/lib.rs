extern crate ease;
extern crate hyper;

use ease::{Url, Request, Error};
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};

pub struct HazelcastRestClient {
    ip_address: &'static str,
    port: &'static str,
}

impl HazelcastRestClient {
    pub fn new(ip_address: &'static str, port: &'static str) -> HazelcastRestClient {
        HazelcastRestClient {
            ip_address: ip_address,
            port: port,
        }
    }

    pub fn queue_offer<T: ToString>(self: &Self,
                                    queue_name: &str,
                                    value: T)
                                    -> Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url)
            .header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])))
            .body(value.to_string())
            .post()
            .and_then(|x| Ok(x.body))
    }

    pub fn queue_delete(self: &Self, queue_name: &str, timeout: i32) -> Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name,
                                 timeout.to_string());
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).delete().and_then(|x| Ok(x.body))
    }

    pub fn queue_size(self: &Self, queue_name: &str) -> Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/size",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).get().and_then(|x| Ok(x.body))
    }

    pub fn map_put<T: ToString>(self: &Self,
                                map_name: &str,
                                key_name: &str,
                                value: T)
                                -> Result<String, Error> {

        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url)
            .header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])))
            .body(value.to_string())
            .post()
            .and_then(|x| Ok(x.body))
    }

    pub fn map_get(self: &Self, map_name: &str, key_name: &str) -> Result<String, Error> {

        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url)
            .header(ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])))
            .get()
            .and_then(|x| Ok(x.body))
    }

    pub fn map_remove(self: &Self, map_name: &str, key_name: &str) -> Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).delete().and_then(|x| Ok(x.body))
    }

    pub fn map_remove_all(self: &Self, map_name: &str) -> Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).delete().and_then(|x| Ok(x.body))
    }
}
#[test]
fn it_works() {
    let client = HazelcastRestClient::new("192.168.1.23", "5701");
    match client.queue_offer::<String>("orhan", "3".to_owned()) {
        Ok(resp) => println!("{:?}", resp),
        Err(err) => println!("{:?}", err),
    }
}

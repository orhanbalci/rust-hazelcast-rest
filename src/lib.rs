extern crate hyper;
use hyper::*;
use hyper::header::{Headers,Connection};
use std::io::Read;
use std::result::Result as StdResult;

pub struct HazelcastRestClient {
    ip_address: &'static str,
    port: &'static str,
    http_client: Client,
}

impl HazelcastRestClient {
    pub fn new(ip_address: &'static str, port: &'static str) -> HazelcastRestClient {
        HazelcastRestClient {
            ip_address: ip_address,
            port: port,
            http_client: Client::new(),
        }
    }

    pub fn queue_offer<T: ToString>(self: &Self,
                                    queue_name: &str,
                                    value: T)
                                    -> std::result::Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        self.http_client.post(&url_string).header(Connection::keep_alive()).body(&value.to_string()).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn queue_delete(self: &Self,
                        queue_name: &str,
                        timeout: i32)
                        -> std::result::Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name,
                                 timeout.to_string());
        self.http_client.delete(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn queue_size(self: &Self, queue_name: &str) -> std::result::Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/size",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        self.http_client.get(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn map_put<T: ToString>(self: &Self,
                                map_name: &str,
                                key_name: &str,
                                value: T)
                                -> std::result::Result<String, Error> {

        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        self.http_client.post(&url_string).body(&value.to_string()).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn map_get(self: &Self,
                   map_name: &str,
                   key_name: &str)
                   -> std::result::Result<String, Error> {

        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        self.http_client.get(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn map_remove(self: &Self,
                      map_name: &str,
                      key_name: &str)
                      -> std::result::Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name,
                                 key_name);
        self.http_client.delete(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }

    pub fn map_remove_all(self: &Self, map_name: &str) -> std::result::Result<String, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/maps/{}",
                                 self.ip_address,
                                 self.port,
                                 map_name);
        self.http_client.delete(&url_string).send().and_then(|mut x| {
            let mut content = String::new();
            x.read_to_string(&mut content);
            StdResult::Ok(content)
        })
    }
}
#[test]
fn it_works() {
    let client = HazelcastRestClient::new("10.0.2.15", "5701");
    client.queue_offer::<String>("orhan", "3".to_owned());
    client.queue_offer::<String>("orhan", "4".to_owned());
    assert_eq!(12, client.queue_size("orhan").unwrap().parse::<i32>().unwrap());
    assert_eq!(3, client.queue_delete("orhan",10).unwrap().parse::<i32>().unwrap());
}

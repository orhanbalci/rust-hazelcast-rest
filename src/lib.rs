extern crate ease;

use ease::{Url, Request, Response, Error};

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
    // http://node IP address:port/hazelcast/rest/queues/queueName
    pub fn queue_offer<T: ToString>(self: &Self,
                                    queue_name: &str,
                                    value: T)
                                    -> Result<Response, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).body(value.to_string()).post()
    }

    pub fn queue_delete(self: &Self, queue_name: &str, timeout: i32) -> Result<Response, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/{}",
                                 self.ip_address,
                                 self.port,
                                 queue_name,
                                 timeout.to_string());
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).delete()
    }

    pub fn queue_size(self: &Self, queue_name: &str) -> Result<Response, Error> {
        let url_string = format!("http://{}:{}/hazelcast/rest/queues/{}/size",
                                 self.ip_address,
                                 self.port,
                                 queue_name);
        let url = Url::parse(&url_string).unwrap();
        Request::new(url).get()
    }
}
#[test]
fn it_works() {
    let client = HazelcastRestClient::new("192.168.1.23", "5701");
    match client.queue_offer::<String>("orhan", "balci".to_owned()) {
        Ok(resp) => println!("{:?}",resp),
        Err(err) => println!("{:?}",err)
    }
}

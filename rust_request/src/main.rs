extern crate requests;
extern crate reqwest;
extern crate hyper;
extern crate time;
extern crate chrono;
// extern crate hazelcast_rest;
use std::io::Read;
use std::collections::HashMap;
// use hazelcast_rest::HazelcastRestClient;
use hyper::Client;

use requests::ToJson;
use chrono::prelude::*;


fn main() {
    //  let response = requests::get("https://iot-rust.firebaseio.com/1637Orrington/dryer1/status.json").unwrap();
    // //  assert_eq!(response.url(), "https://iot-rust.firebaseio.com/1637Orrington/laundry1/status.json");
    //  assert_eq!(response.reason(), "OK");
    //  assert_eq!(response.status_code(), requests::StatusCode::Ok);
    //  let data = response.json().unwrap();
    //  println!("{:?}", data["startTime"].to_string());

    // let mut response = reqwest::get("https://iot-rust.firebaseio.com/1637Orrington/dryer1/status.json").expect("Failed to send request");
    // // println!("{}", response.status());
    // // for header in response.headers().iter() {
    // //     println!("{}: {}", header.name(), header.value_string());
    // // }
    let mut buf = String::new();
    // response.read_to_string(&mut buf).expect("Failed to read response");
    // println!("{}", buf);

    //
    // let mut map = HashMap::new();
    // map.insert("status","in use");
    // map.insert("startTime","7pm");
    // map.insert("remainingTime","30min");
    // // let params = [("foo", "bar"), ("baz", "quux")];
    //
    // let client = reqwest::Client::new().unwrap();
    // // let res = client.post("https://iot-rust.firebaseio.com/1637Orrington/dryer1.json").json(&map).send().expect("Failed to send request");
    // let mut response = client.request(reqwest::Method::Put, "https://iot-rust.firebaseio.com/1637Orrington/dryer1.json")
    // .json(&map)
    // .send()
    // .expect("Failed to send request");

    // // get local time
    // let mut local: DateTime<Local> = Local::now();
    // println!("{:?}", local);

    get_status("dryer1",&mut buf);

}

// get status(in use or available) of the given machine, store the status in parameter buf
fn get_status(machine: &str, buf: &mut String){
    println!("url: {}", ("https://iot-rust.firebaseio.com/1637Orrington/".to_string()+machine+"/status.json").as_str());
    let mut response = reqwest::get(("https://iot-rust.firebaseio.com/1637Orrington/".to_string()+machine+"/status.json").as_str()).expect("Failed to send request");
    response.read_to_string(buf).expect("Failed to read response");
    println!("{}", buf);

}

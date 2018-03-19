extern crate nickel;
extern crate serde_yaml;

use std::collections::BTreeMap;
use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};

fn main() {
    let mut server = Nickel::new();
    server.get("**", handle);
    server.listen("127.0.0.1:8080").unwrap();
}

fn handle<'mw>(_req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let mut map = BTreeMap::new();
    for var in std::env::vars(){
        map.insert(var.0, var.1);
    }

    let s = serde_yaml::to_string(&map).unwrap();
    res.send(s)
}

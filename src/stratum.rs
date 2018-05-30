use serde_json;
use std::io::Write;
use std::net::ToSocketAddrs;
use tokio::net::TcpStream;
use url::Url;

pub struct Stratum {
}

#[derive(Serialize)]
pub struct Message {
  id: i32,
  params: Params,
}

#[derive(Serialize)]
pub enum Params {
  Login(Login),
}

#[derive(Serialize)]
pub struct Login {
  user: String,
  pass: String,
  agent: String,
}

impl Stratum {
  pub fn new<A: ToSocketAddrs>(addr: A) -> Self {

    let mut addrs = addr.to_socket_addrs().unwrap();

    let connect = TcpStream::connect(&addrs.next().unwrap());

    Stratum {}
  }
}

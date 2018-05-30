extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate url;
#[macro_use]
extern crate serde_derive;
extern crate mithril;

pub mod stratum;

fn main() {
    let stratum = stratum::Stratum::new("xmr.coinfoundry.org:3032");
}

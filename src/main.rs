extern crate cdrs;

use cdrs::client::CDRS;
use cdrs::authenticators::PasswordAuthenticator;
use cdrs::compression;
fn main() {
    println!("Hello, world!");
    let user = "dart".to_string();
    let pass = "dart".to_string();
    let authenticator = PasswordAuthenticator::new(user, pass);
    let addr = "0.0.0.0".to_string();
    // pass authenticator into CDRS' constructor
    let mut client = CDRS::new(addr, authenticator).unwrap();

    // without compression
    let response_frame = client.start(compression::Compression::Snappy).unwrap();
    // let select_query = String::from("SELECT * FROM cycling.person;");

    // match client.query(select_query) {
    //     Ok(res) => {
    //         println!("Result frame: {:?},\nparsed body: {:?}",
    //                  res,
    //                  res.get_body())
    //     }
    //     Err(err) => println!("Error: {:?}", err),
    // }


}

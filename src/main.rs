extern crate web3;

use web3::futures::Future;

fn main() {
    let (_eloop, transport) = web3::transports::Http::new("http://localhost:8545").unwrap();

    let web3 = web3::Web3::new(transport);
    let accounts = web3.eth().accounts().wait().unwrap();

    let balance = web3.eth().balance(accounts[1], None).wait().unwrap();

    println!("Balance: {}", balance);
}
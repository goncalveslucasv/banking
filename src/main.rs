#![allow(dead_code)]
pub mod domain;

use domain::Account;

fn main(){
    let account = Account::create();

    println!("{}", account.balance())

}





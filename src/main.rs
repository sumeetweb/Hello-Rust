use crate::step1::BalanceModule;
mod step1;

fn main() {
    // let mut balances:BalanceModule = step1::BalanceModule {
    //     balance : HashMap::new(),
    // };

    // balances.balance.insert(1, 100);
    
    // println!("{:?}", balances.balance.get(&1).unwrap());

    let mut balances = BalanceModule::new();
    balances.balance.insert(1, 100);
    println!("{:?}", balances.balance.get(&1).unwrap());
}

#[test]
pub fn test_step1(){
    let mut balances = BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
}
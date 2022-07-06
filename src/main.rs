mod step1;
mod step2;
mod step3;

fn main() {
    // let mut balances:BalanceModule = step1::BalanceModule {
    //     balance : HashMap::new(),
    // };

    // balances.balance.insert(1, 100);
    
    // println!("{:?}", balances.balance.get(&1).unwrap());

    // let mut balances = step1::BalanceModule::new();
    // balances.balance.insert(1, 100);
    // println!("{:?}", balances.balance.get(&1).unwrap());
}

#[test]
pub fn test_step1(){
    let mut balances = step1::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
}

#[test]
pub fn test_step2(){
    let mut balances = step2::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
}

#[test]
pub fn test_step3(){

    type AccountId = u32;
    type Balance = u32;

    let mut balances = step3::BalanceModule::new();
    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.get_balance(1) == 100);
    assert!(balances.get_balance(2) == 200);

    balances.transfer(1, 2, 50);
    assert!(balances.get_balance(2) == 250);
}
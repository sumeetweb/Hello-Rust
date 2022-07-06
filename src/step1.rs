use std::collections::HashMap;

pub struct BalanceModule {
    pub balance : HashMap<u32, u32>,
}

impl BalanceModule {
    pub fn new() -> Self {
        Self {
            balance : HashMap::new(),
        }
    }

    pub fn set_balance(&mut self, who:u32, amount:u32) {
        self.balance.insert(who, amount);
    }

    pub fn get_balance(&self, who:u32) -> u32 {
        *self.balance.get(&who).unwrap_or(&0)
    }
    
    // Lifetime in this scope, pass as error value.
    pub fn transfer(&mut self, from: u32, to:u32, amount:u32) -> Result<(), &'static str> {
        let from_balance:&u32 = self.balance.get(&from).ok_or("From does not have enough funds")?;
        let to_balance:&u32 = self.balance.get(&to).ok_or("To balance doesnot exist")?;

        let new_from_balance:u32 = from_balance.checked_sub(amount).ok_or("User DNE")?;
        let new_to_balance:u32 = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balance.insert(from, new_from_balance);
        self.balance.insert(to, new_to_balance);

        Ok(())
    }
}
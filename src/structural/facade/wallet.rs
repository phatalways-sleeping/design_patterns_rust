pub struct Wallet {
    balance: usize,
}

impl Default for Wallet {
    fn default() -> Self {
        Self { balance: 0 }
    }
}

impl Wallet {
    pub fn credit_balance(&mut self, amount: usize) {
        self.balance.checked_add(amount).expect("Failed to add");
    }

    pub fn debit_balance(&mut self, amount: usize) {
        self.balance
            .checked_sub(amount)
            .expect("Balance is not sufficient");
    }
}

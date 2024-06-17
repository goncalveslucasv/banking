use crate::domain::balance::Balance;
use crate::domain::movement::Movement;

#[derive(Clone)]
pub struct Account {
    balance: Balance,
    movements: Vec<Movement>,
}

impl Account {
    pub fn create() -> Self {
        return Account{
            balance: Balance::create(),
            movements: Vec::new(),
        }
    }

    pub fn perform_movement(&self, movement: Movement) -> Result<Self, String> {
        let new_balance = self.balance.increase(movement.obtain_amount());
        if new_balance.amount() < 0.0 {
            return Err(String::from("Some error"));
        }
        let new_movements= {
            let mut movements = self.movements.clone();
            movements.push(movement);
            movements
        };

        Ok(Self { balance: new_balance, movements: new_movements,..self.clone()})
    }

    pub fn balance(self) -> f32 {
        self.balance.amount()
    }
}

#[cfg(test)]
mod tests {
    use super::Account;
    use crate::domain::movement::Movement;

    #[test]
    fn create_an_account_with_zero_balance(){
        let account = Account::create();

        let balance = account.balance();

        assert_eq!(0f32, balance);
    }

    #[test]
    fn create_a_credit_movement(){
        let account = Account::create();
        let movement = Movement::create(50f32);

        let refreshed_account = account.perform_movement(movement).unwrap();

        assert_eq!(refreshed_account.balance(), 50f32);
    }

    #[test]
    fn create_a_debit_movement(){
        let account = Account::create();
        let initial_balance = Movement::create(50f32);
        let refreshed_account = account.perform_movement(initial_balance).unwrap();

        let credit_movement = Movement::create(-10f32);
        let refreshed_account = refreshed_account.perform_movement(credit_movement).unwrap();

        assert_eq!(refreshed_account.balance(), 40f32);
    }

    #[test]
    fn throw_an_error_when_there_is_no_balance(){
        let account = Account::create();

        let credit_movement = Movement::create(-10f32);
        let refreshed_account = account.perform_movement(credit_movement);

        assert!(refreshed_account.is_err())
    }
}


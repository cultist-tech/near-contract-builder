#[macro_export]
macro_rules! impl_owner_feature {
    ($contract: ident, $instance: ident, $assert_owner:ident) => {
        use $crate::owner::{ContractOwner};

        #[near_bindgen]
        impl ContractOwner for $contract {
          fn get_owner(&self) -> AccountId {
            self.$instance.get_owner()
          }

          fn set_owner(&mut self, account_id: AccountId) -> AccountId {
            self.$assert_owner();
            self.$instance.set_owner(account_id)
          }
        }
    };
}

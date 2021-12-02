use solana_sdk::account::{Account, AccountSharedData};

pub struct PythUtil {}

impl PythUtil {
    pub fn update_slot(
        _str: &str,
        account_data: AccountSharedData,
        slot: u64,
    ) -> AccountSharedData {
        let mut account = Account::from(account_data);
        let new_slot = &slot.to_le_bytes()[..];

        account.data[40] = new_slot[0];
        account.data[41] = new_slot[1];
        account.data[42] = new_slot[2];
        account.data[43] = new_slot[3];
        account.data[44] = new_slot[4];
        account.data[45] = new_slot[5];
        account.data[46] = new_slot[6];
        account.data[47] = new_slot[7];

        AccountSharedData::from(account)
    }
}

use quick_protobuf::serialize_into_vec;
//use solana_program::{account_info::AccountInfo};
use solana_program::pubkey::Pubkey;
use solana_sdk::account::{Account, AccountSharedData};
use std::str::FromStr;
use switchboard_protos::protos::{
    aggregator_state::{AggregatorState, RoundResult},
    switchboard_account_types::SwitchboardAccountType,
};
pub struct SwitchboardUtil {}

impl SwitchboardUtil {
    pub fn update_slot(str: &str, account_data: AccountSharedData, slot: u64) -> AccountSharedData {
        let account_key = Pubkey::from_str(str).unwrap();
        let mut account = Account::from(account_data);
        let owner = Pubkey::from_str(&account.owner.to_string()).unwrap();
        let account_info = solana_program::account_info::AccountInfo::new(
            &account_key,
            false,
            true,
            &mut account.lamports,
            &mut account.data,
            &owner,
            account.executable,
            account.rent_epoch,
        );
        let state = switchboard_program::get_aggregator(&account_info.clone()).unwrap();

        let result = state.current_round_result.unwrap();
        let new_state = AggregatorState {
            version: state.version.clone(),
            configs: state.configs.clone(),
            fulfillment_manager_pubkey: state.fulfillment_manager_pubkey.clone(),
            job_definition_pubkeys: state.job_definition_pubkeys.clone(),
            agreement: state.agreement.clone(),
            current_round_result: Some(RoundResult {
                num_success: result.num_success,
                num_error: result.num_error,
                result: result.result,
                round_open_slot: Some(slot),
                round_open_timestamp: result.round_open_timestamp,
                min_response: result.min_response,
                max_response: result.max_response,
                medians: result.medians,
            }),
            last_round_result: state.last_round_result.clone(),
            parse_optimized_result_address: state.parse_optimized_result_address.clone(),
            bundle_auth_addresses: state.bundle_auth_addresses.clone(),
        };

        let mut data = serialize_into_vec(&new_state).unwrap();
        data.insert(0, SwitchboardAccountType::TYPE_AGGREGATOR as u8);

        let mut new_account = Account::new_data(account.lamports, &data, &account.owner).unwrap();
        new_account.data = data;

        AccountSharedData::from(new_account)
    }
}

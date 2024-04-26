use scale::Encode as _;
#[allow(dead_code)]
pub const CODE_HASH: [u8; 32] = [
    199u8, 212u8, 119u8, 36u8, 237u8, 68u8, 63u8, 201u8, 105u8, 219u8, 34u8, 167u8, 29u8, 234u8,
    219u8, 244u8, 122u8, 163u8, 35u8, 173u8, 17u8, 240u8, 155u8, 153u8, 100u8, 203u8, 208u8, 62u8,
    171u8, 176u8, 32u8, 190u8,
];
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub enum PSP22Error {
    Custom(String),
    InsufficientBalance(),
    InsufficientAllowance(),
    ZeroRecipientAddress(),
    ZeroSenderAddress(),
    SafeTransferCheckFailed(String),
}
#[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
pub enum NoChainExtension {}
pub mod event {
    #[allow(dead_code, clippy::large_enum_variant)]
    #[derive(Debug, Clone, PartialEq, Eq, scale :: Encode, scale :: Decode)]
    pub enum Event {
        Approval {
            owner: ink_primitives::AccountId,
            spender: ink_primitives::AccountId,
            amount: u128,
        },
        Transfer {
            from: Option<ink_primitives::AccountId>,
            to: Option<ink_primitives::AccountId>,
            value: u128,
        },
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Instance {
    account_id: ink_primitives::AccountId,
}
impl From<ink_primitives::AccountId> for Instance {
    fn from(account_id: ink_primitives::AccountId) -> Self {
        Self { account_id }
    }
}
impl From<Instance> for ink_primitives::AccountId {
    fn from(instance: Instance) -> Self {
        instance.account_id
    }
}
impl ink_wrapper_types::EventSource for Instance {
    type Event = event::Event;
}
pub trait PSP22Metadata {
    fn token_name(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<Option<String>, ink_wrapper_types::InkLangError>>;
    fn token_symbol(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<Option<String>, ink_wrapper_types::InkLangError>>;
    fn token_decimals(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<u8, ink_wrapper_types::InkLangError>>;
}
impl PSP22Metadata for Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    fn token_name(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<Option<String>, ink_wrapper_types::InkLangError>> {
        let data = vec![61u8, 38u8, 27u8, 212u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn token_symbol(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<Option<String>, ink_wrapper_types::InkLangError>> {
        let data = vec![52u8, 32u8, 91u8, 229u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn token_decimals(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<u8, ink_wrapper_types::InkLangError>> {
        let data = vec![114u8, 113u8, 183u8, 130u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
}
pub trait PSP22 {
    fn total_supply(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>>;
    fn balance_of(
        &self,
        owner: ink_primitives::AccountId,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>>;
    fn allowance(
        &self,
        owner: ink_primitives::AccountId,
        spender: ink_primitives::AccountId,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>>;
    fn transfer(
        &self,
        to: ink_primitives::AccountId,
        value: u128,
        _data: Vec<u8>,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>;
    fn transfer_from(
        &self,
        from: ink_primitives::AccountId,
        to: ink_primitives::AccountId,
        value: u128,
        _data: Vec<u8>,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>;
    fn approve(
        &self,
        spender: ink_primitives::AccountId,
        value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>;
    fn increase_allowance(
        &self,
        spender: ink_primitives::AccountId,
        delta_value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>;
    fn decrease_allowance(
        &self,
        spender: ink_primitives::AccountId,
        delta_value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>;
}
impl PSP22 for Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    fn total_supply(
        &self,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>> {
        let data = vec![22u8, 45u8, 248u8, 194u8];
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn balance_of(
        &self,
        owner: ink_primitives::AccountId,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>> {
        let data = {
            let mut data = vec![101u8, 104u8, 56u8, 47u8];
            owner.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn allowance(
        &self,
        owner: ink_primitives::AccountId,
        spender: ink_primitives::AccountId,
    ) -> ink_wrapper_types::ReadCall<Result<u128, ink_wrapper_types::InkLangError>> {
        let data = {
            let mut data = vec![77u8, 71u8, 217u8, 33u8];
            owner.encode_to(&mut data);
            spender.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ReadCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn transfer(
        &self,
        to: ink_primitives::AccountId,
        value: u128,
        _data: Vec<u8>,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![219u8, 32u8, 249u8, 245u8];
            to.encode_to(&mut data);
            value.encode_to(&mut data);
            _data.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn transfer_from(
        &self,
        from: ink_primitives::AccountId,
        to: ink_primitives::AccountId,
        value: u128,
        _data: Vec<u8>,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![84u8, 179u8, 199u8, 110u8];
            from.encode_to(&mut data);
            to.encode_to(&mut data);
            value.encode_to(&mut data);
            _data.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn approve(
        &self,
        spender: ink_primitives::AccountId,
        value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![178u8, 15u8, 27u8, 189u8];
            spender.encode_to(&mut data);
            value.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn increase_allowance(
        &self,
        spender: ink_primitives::AccountId,
        delta_value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![150u8, 214u8, 181u8, 122u8];
            spender.encode_to(&mut data);
            delta_value.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
    #[allow(dead_code, clippy::too_many_arguments)]
    fn decrease_allowance(
        &self,
        spender: ink_primitives::AccountId,
        delta_value: u128,
    ) -> ink_wrapper_types::ExecCall<Result<Result<(), PSP22Error>, ink_wrapper_types::InkLangError>>
    {
        let data = {
            let mut data = vec![254u8, 203u8, 87u8, 213u8];
            spender.encode_to(&mut data);
            delta_value.encode_to(&mut data);
            data
        };
        ink_wrapper_types::ExecCall::new(self.account_id, data)
    }
}
#[allow(dead_code)]
pub fn upload() -> ink_wrapper_types::UploadCall {
    let wasm = include_bytes!("../../../artifacts/psp22.wasm");
    ink_wrapper_types::UploadCall::new(wasm.to_vec(), CODE_HASH)
}
impl Instance {
    #[allow(dead_code, clippy::too_many_arguments)]
    pub fn new(
        supply: u128,
        name: Option<String>,
        symbol: Option<String>,
        decimals: u8,
    ) -> ink_wrapper_types::InstantiateCall<Self> {
        let data = {
            let mut data = vec![155u8, 174u8, 157u8, 94u8];
            supply.encode_to(&mut data);
            name.encode_to(&mut data);
            symbol.encode_to(&mut data);
            decimals.encode_to(&mut data);
            data
        };
        ink_wrapper_types::InstantiateCall::new(CODE_HASH, data)
    }
}

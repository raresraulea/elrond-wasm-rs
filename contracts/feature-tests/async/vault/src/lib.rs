#![no_std]

elrond_wasm::imports!();

/// General test contract.
/// Used especially for investigating async calls and contract interaction in general.
#[elrond_wasm_derive::contract]
pub trait Vault {
	#[init]
	fn init(&self) {}

	#[endpoint]
	fn echo_arguments(
		&self,
		#[var_args] args: VarArgs<BoxedBytes>,
	) -> SCResult<MultiResultVec<BoxedBytes>> {
		Ok(args.into_vec().into())
	}

	#[payable("*")]
	#[endpoint]
	fn accept_funds(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) {
		self.accept_funds_event(&token, &payment);
	}

	#[payable("*")]
	#[endpoint]
	fn reject_funds(
		&self,
		#[payment_token] token: TokenIdentifier,
		#[payment] payment: Self::BigUint,
	) -> SCResult<()> {
		self.reject_funds_event(&token, &payment);
		sc_error!("reject_funds")
	}

	#[endpoint]
	fn retrieve_funds(
		&self,
		token: TokenIdentifier,
		amount: Self::BigUint,
		#[var_args] return_message: OptionalArg<BoxedBytes>,
	) {
		self.retrieve_funds_event(&token, &amount);

		let data = match &return_message {
			OptionalArg::Some(data) => data.as_slice(),
			OptionalArg::None => &[],
		};
		self.send()
			.direct_via_async_call(&self.blockchain().get_caller(), &token, &amount, data);
	}

	#[event("accept_funds")]
	fn accept_funds_event(
		&self,
		#[indexed] token: &TokenIdentifier,
		#[indexed] payment: &Self::BigUint,
	);

	#[event("reject_funds")]
	fn reject_funds_event(
		&self,
		#[indexed] token: &TokenIdentifier,
		#[indexed] payment: &Self::BigUint,
	);

	#[event("retrieve_funds")]
	fn retrieve_funds_event(
		&self,
		#[indexed] token: &TokenIdentifier,
		#[indexed] amount: &Self::BigUint,
	);
}

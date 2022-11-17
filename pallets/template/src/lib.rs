#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type StringLimit: Get<u32>;
	}

	#[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub(super) type ProjectStore<T: Config> =
		StorageValue<_, Project<T::AccountId, BoundedVec<u8, T::StringLimit>>, OptionQuery>;

	// - genesis state config: https://docs.substrate.io/reference/how-to-guides/basics/configure-genesis-state/
	// - basic example pallet: https://github.com/paritytech/substrate/blob/master/frame/examples/basic/src/lib.rs
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub account_map: Vec<T::AccountId>,
	}

	// - PR remove default for accountid: https://github.com/paritytech/substrate/pull/10403
	// - stackoverflow: https://substrate.stackexchange.com/questions/4691/the-trait-stddefaultdefault-is-not-implemented-for-accountid?noredirect=1&lq=1
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { account_map: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let err_msg = "string to long";

			ProjectStore::<T>::put(Project {
				id: "projectID01".as_bytes().to_vec().try_into().expect(err_msg),
				boq_id: "to_be_added".as_bytes().to_vec().try_into().expect(err_msg),
				title: "Elbphilharmonie Hamburg".as_bytes().to_vec().try_into().expect(err_msg),
				location: "Platz d. Deutschen Einheit 4, 20457 Hamburg, DE"
					.as_bytes()
					.to_vec()
					.try_into()
					.expect(err_msg),
				construction_manager: self.account_map[0].clone(),
			});
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn do_something(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

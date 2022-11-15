#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
mod types;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
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

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn do_something(_origin: OriginFor<T>) -> DispatchResult {
			Ok(())
		}
	}
}

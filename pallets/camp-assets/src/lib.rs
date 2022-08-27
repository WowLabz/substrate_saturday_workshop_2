#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		inherent::Vec,
		pallet_prelude::{ValueQuery, *},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::ops::AddAssign;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn get_asset_id)]
	pub type AssetId<T: Config> = StorageValue<_, T::AssetId, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokenCreated { name: Vec<u8>, owner: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_token(
			origin: OriginFor<T>,
			name: Vec<u8>,
			symbol: Vec<u8>,
			decimals: u8,
			initial_supply: T::Balance,
		) -> DispatchResult {
			let owner = ensure_signed(origin.clone())?;

			let id = Self::get_asset_id();

			let token_owner =
				<T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(owner.clone());

			pallet_assets::Pallet::<T>::create(
				origin.clone(),
				id.clone(),
				token_owner,
				initial_supply,
			)?;

			pallet_assets::Pallet::<T>::set_metadata(origin, id, name.clone(), symbol, decimals)?;

			Self::deposit_event(Event::TokenCreated { name, owner });

			Ok(())
		}
	}
}

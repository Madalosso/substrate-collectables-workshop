#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Learn about Macros used in the `polkadot-sdk`, making pallet development easier.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::mint(who)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn mint(owner: T::AccountId) -> DispatchResult {
			Self::deposit_event(Event::<T>::Created { owner });
			Ok(())
		}
	}
}

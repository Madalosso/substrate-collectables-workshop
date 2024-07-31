#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// Learn about Macros used in the `polkadot-sdk`, making pallet development easier.
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Learn about the Pallet struct: the structure on which we implement all functions and traits
	// for the Pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Learn about frame_system, and `Config`.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		// Using 16 bytes to represent a kitty DNA
		pub dna: [u8; 16],
		pub owner: T::AccountId,
	}

	/// Learn about storage value.
	#[pallet::storage]
	pub(super) type CountForKitties<T: Config> = StorageValue<Value = u64, QueryKind = ValueQuery>;

	/// Learn about storage maps.
	#[pallet::storage]
	pub(super) type Kitties<T: Config> = StorageMap<Key = [u8; 16], Value = Kitty<T>>;

	/// Track the kitties owned by each account.
	#[pallet::storage]
	pub(super) type KittiesOwned<T: Config> = StorageMap<
		Key = T::AccountId,
		Value = BoundedVec<[u8; 16], ConstU32<100>>,
		QueryKind = ValueQuery,
	>;

	// Learn about events.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		/* TODO: Create a new event called `Transferred`:
			- Parameters are:
				- `from` which is `T::AccountId`.
				- `to` which is `T::AccountId`.
				- `kitty_id` which is `[u8; 16]`.
		*/
	}

	#[pallet::error]
	pub enum Error<T> {
		TooManyKitties,
		DuplicateKitty,
		TooManyOwned,
	}

	// Learn about callable functions and dispatch.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			// Learn about `origin`.
			let who = ensure_signed(origin)?;
			let dna = Self::gen_dna();
			Self::mint(who, dna)?;
			Ok(())
		}

		/* TODO: Make a new extrinsic called `transfer`.
			- Input parameters are:
				- `origin` which is `OriginFor<T>`.
				- `to` which is `T::AccountId`.
				- `kitty_id` which is `[u8; 16]`.
			- Returns a `DispatchResult`.
			- The inner logic should be:
				- Get the caller `who` from `ensure_signed`.
				- Call `Self::do_transfer`, and propagate the result.
				- End with Ok(()).
		*/
	}

	// Learn about internal functions.
	impl<T: Config> Pallet<T> {
		// Generates and returns DNA and Sex
		fn gen_dna() -> [u8; 16] {
			// Create randomness payload. Multiple kitties can be generated in the same block,
			// retaining uniqueness.
			let unique_payload = (
				frame_system::Pallet::<T>::parent_hash(),
				frame_system::Pallet::<T>::block_number(),
				frame_system::Pallet::<T>::extrinsic_index(),
				CountForKitties::<T>::get(),
			);

			let encoded_payload = unique_payload.encode();
			frame_support::Hashable::blake2_128(&encoded_payload)
		}

		// Learn about `AccountId`.
		fn mint(owner: T::AccountId, dna: [u8; 16]) -> DispatchResult {
			let kitty = Kitty { dna, owner: owner.clone() };
			// Check if the kitty does not already exist in our storage map
			ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);

			let current_count: u64 = CountForKitties::<T>::get();
			let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;

			KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyOwned)?;
			Kitties::<T>::insert(dna, kitty);
			CountForKitties::<T>::set(new_count);

			Self::deposit_event(Event::<T>::Created { owner });
			Ok(())
		}

		/* TODO: Create an internal function called `do_transfer`:
			- It has inputs:
				- `from` which is `T::AccountId`.
				- `to` which is `T::AccountId`.
				- `kitty_id` which is `[u8; 16]`.
			- It returns a `DispatchResult`
			- The inner logic for now is:
				- Call `Self::dispatch_event` on and emit `Event::<T>:Transferred` with params.
				- Return `Ok(())`.
		*/
	}
}

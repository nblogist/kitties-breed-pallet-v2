// ! step 1
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error,
    decl_event,
    decl_module,
    decl_storage,
    ensure,
    traits::Randomness,
    RuntimeDebug, // RuntimeDebug builds simplified debug, (it is similar to rust debug)
    StorageDoubleMap,
    StorageValue,
};
use frame_system::ensure_signed;
// !Note we use [u8;16] in kitty because
/*
fn blake2_128(data: &[u8]) -> [u8; 16] { // ! it returns [u8; 16] so its easier for assignement
        sp_core::hashing::blake2_128(data)
    }
*/
use sp_io::hashing::blake2_128;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
    trait Store for Module<T:Trait> as Kitties{
        // Stores all the kitties with the accountid aof owner and kitty number
        pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
        // Stores the next kitty id
        pub NextKittyId get(fn next_kitty_id):u32;
    }
}

decl_event! {
    pub enum Event<T> where <T as frame_system::Trait>::AccountId{
        KittyCreated(AccountId, u32, Kitty),
        // ! [owner, kitty_id, kitty]
    }
}

decl_error! {
    pub enum Error for Module<T: Trait>{
        KittyIdOverFlow,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin{
        fn deposit_event()= default;
        type Error = Error<T>;

        // ! gas for this method ~
        #[weight = 1000]
        pub fn create(origin){
            // ! Putting it in the start so we don't bother doing any thing else if kittyid is overflown
            
            ensure!(!Self::next_kitty_id().checked_add(1).is_none(), Error::<T>::KittyIdOverFlow);

            let sender = ensure_signed(origin)?; // ! is the account id from which you are sending the trx
            // ! generating random number for dna
            let payload = (
                <pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
                &sender,
                <frame_system::Module<T>>::extrinsic_index(),
            );
            let dna = payload.using_encoded(blake2_128);

            // ! creating kitty
            let kitty = Kitty(dna);
            let kitty_id = Self::next_kitty_id();
            Kitties::<T>::insert(&sender, kitty_id, kitty.clone());

            NextKittyId::put(kitty_id+1);

            // emit events
            Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty));
        }
    }
}

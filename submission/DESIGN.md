<!-- For my own reference: https://discordapp.com/channels/772968587060445244/772968587060445251/813166983364739095 -->
# Kitties Pallet Design

This is a design document submitted for substrate developer academy assignment 2 (Kitties Pallet)

## Storage (decl_storage!)

    * kitties: double_map (kitty_dna: u128, owner: AccountId )  => kitty_id:u32
<!-- types look like TS, not RUST but made it to convey better -->

## Events (decl_event!)

    * KittyBorn(AccountId, u32, u128, u128, Gender, Kitty),
<!-- [owner, kitty_id, dad_dna, mom_dna, kitty] -->

## Errors (decl_error!)

    * GenderMismatchForBreeding,
  <!-- tbh I don't think we'd ever encounter this error, but I guess for the hygiene, and in case any 2 random dna match (astronomical chances tho) -->

## Calls (decl_module!)

    * fn breed(origin, dad_dna: u128, mom_dna: u128)
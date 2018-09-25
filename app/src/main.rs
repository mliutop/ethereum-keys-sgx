extern crate secp256k1_enclave_rust;
use secp256k1_enclave_rust::dothing::run;
/*
 *
 * TODO: Make into CLI with docopt - init to create new key (can only do once? Store state?)
 * TODO: add messsage to get back the signed hash of it.
 * TODO: Have the first call the enc. do an ::new, which spits out the PublicKey
 * and a sealed privkey.
 * TODO: Docopt logic desired: init (for a new key - have it check if one exists first, can force new?)
 * TODO: Have a way we can use a specific key if passed as an arg, and it'll attempt to find a file called that and decrpy it.
 * TODO: Have a way we can pass in a message and have it sign it. Hash the message outside as it currently does.
 **/
fn main() {
   run();
}

/*

newkey

usekey

signmsg (either uses default key or you can pass a named key)

*/
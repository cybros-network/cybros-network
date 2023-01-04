Worker
====

notes:

- https://github.com/paritytech/substrate/pull/12897
- https://github.com/paritytech/substrate/tree/master/client/cli
- https://github.com/cberner/redb/blob/master/examples/int_keys.rs
- https://github.com/paritytech/polkadot-introspector
- https://github.com/paritytech/polkadot/blob/master/node/overseer/src/lib.rs

```rust
	// OLM in practice

	// Say Alice is the user
	let alice = Account::new();
	// Alice has an identity key
	let alice_identity_key = alice.curve25519_key();
	// Say Bob is the worker
	let mut bob = Account::new();
	// Bob has an identity key,
	// Idea: When a worker add to a cluster, the identity key should store in its entry,
	// so users can get it from chain state
	let bob_identity_key = bob.curve25519_key();

	// Bob should generate OTK before accept messages
	bob.generate_one_time_keys(1);
	// Bob need a way to expose its OTK(s)
	// Idea: When a worker add to a cluster, it needs to push few OTKs to its entry,
	// so users can take one to build encrypt channel, when a user push message through chain,
	// the call should also require the OTK, and remove it from OTK list
	let bob_otk = *bob.one_time_keys().values().next().unwrap();

	// Idea: The worker already push all its OTKs to the chain, so we call this here for worker states
	bob.mark_keys_as_published();

	// Alice use Bob's identity key and a OTK to build the session
	let mut alice_session = alice
		.create_outbound_session(SessionConfig::version_2(), bob_identity_key, bob_otk);

	// Test message, for real case it should be a secret info, e.g. access token
	let message = "Keep it between us, OK?";
	// Alice use session to encrypt it
	let alice_msg = alice_session.encrypt(message);
	// Get the pre-key message, this is the encrypted message
	let OlmMessage::PreKey(m) = alice_msg.clone() else {
		// Get the pre-key can fail, this is just a demo so we don't handle it
		return Ok(())
	};

	// Bob create an inbound session
	// Idea: The chain should have a call e.g. `set_env(cluster, worker, sender_identity_key, otk, pre_key_message)`
	// it removes the otk first, then generate an event that the exact worker (Bob here) can catch,
	// when it catch the event, create an inbound_session
	let result = bob.create_inbound_session(alice_identity_key, &m)?;
	// if the sender's identity key and pre-key message is valid, the session should create
	let mut bob_session = result.session;

	println!("Alice session id: {:?}", alice_session.session_id());
	println!("Bob session id: {:?}", bob_session.session_id());

	// and the worker can decrypt the message
	let what_bob_received = result.plaintext;
	println!("Bob received: {:?}", String::from_utf8(what_bob_received)?);

	// Test message for respond
	let bob_reply = "Yes. Take this, it's dangerous out there!";
	// Use the inbound session, the worker can reply an encrypted message
	// Idea: I'm not sure we need this, but we need a way to let the user know the worker got the secret and store it
	// `set_env` can only allow cluster's moderator invoke, so maybe we don't need this part,
	// but we made a `get_env` for verify the result
	// or we can make a `send_set_env_result` call to let the worker report the result
	let bob_encrypted_reply = bob_session.encrypt(bob_reply).into();

	// Alice can decrypt the reply
	let what_alice_received = alice_session.decrypt(&bob_encrypted_reply)?;
	println!("Alice received: {:?}", String::from_utf8(what_alice_received)?);

	// TODO: MEGOLM for broadcasting secret to the cluster

	// TODO: deno_core
```

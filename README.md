# InductionDB

Realtime collaborative P2P database library

- Realtime: changes are propagated with low latency
- Collaborative: designed for multiple agents to change the same data structure simultaneously, without overwriting each other's changes
- P2P: no centralized or federated servers are needed, the application can effectively host itself
- Database: can query it's own data, and save it atomically with consistency and durability guarantees
- Library: can be embedded into a larger app, and used as it's data layer

## How does this work? I want to learn more!

The following links should give you some basic knowledge about the underlying technologies used:

- [dotJS 2019 - James Long - CRDTs for Mortals](https://www.youtube.com/watch?v=DEcwa68f-jY&t=0s)
  - [CRDTs: The Hard Parts](https://www.youtube.com/watch?v=x7drE24geUw) for further reading
  - [Braid's Demo](https://braid.org/demo/interact) for an interactive demo
- [libp2p DHT](https://curriculum.pl-launchpad.io/curriculum/libp2p/dht/)
- [libp2p Publish/Subscribe](https://docs.libp2p.io/concepts/pubsub/overview/)
  - [Demystifying libp2p Gossipsub](https://www.youtube.com/watch?v=BUc4xta7Mfk) (if you prefer video form)

## Differences

- Does not rely on blockchain tech like [Ceramic](https://ceramic.network/)
- Bindings can be written in multiple languages and is wasm compatible, isn't a js-only project like [Gun](https://gun.eco/) or [OrbitDB](https://github.com/orbitdb/orbit-db)
- Isn't meant to be used as a part of a web application server like with the [Braid](https://braid.org/) family of projects.
- Native code with safety guarantees thanks to Rust
  - Highly performant and perfect for resource limited devices
- Batteries included unlike [Yjs](https://yjs.dev/) and [Automerge](https://automerge.org/): 

## Planned Features

- user auth, multiple clients can be registered to one identity
- user groups aka "trusted peers"
  - list can only be modified if consensus is reached by existing users (critical mass of users cryptographically signs new list)
  - can be used to enforce access control and data locality with confidence
    - data can be configured to only sync to trusted peers, ensuring deletes are respected
    - data can also be configured to be public, but only allow modification by trusted peers, with or without consensus based group approval
  - more robust than "if you have the private key, you can read it" style access control; multiple keys need to be broken to add a new user, and a compromised user can be removed
- data validators to ensure writes from other clients are rejected if they don't follow an application's internal logic
- peer discovery, relay servers for nat traversal/pseudo-privacy
- table-based events: can listen on queries, matching records will be fetched from other peers dynamically
  - queries predicted to be slow (ie on rows without an index) will be rejected to prevent DOS attacks
  - large query results will be paginated in order to ensure backpressure and prevent DOS attacks
- some kind of slow-mode / read only mode when there are too many clients (TBD)

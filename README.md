# InductionDB

Realtime collaborative P2P database library

- Realtime: changes are propagated with low latency
- Collaborative: designed for multiple agents to change the same data structure simultaneously, without overwriting each other's changes
- P2P: no centralized or federated servers are needed, the application can effectively host itself
- Database: can query it's own data, and save it atomically with consistency and durability guarantees
- Library: can be embedded into a larger app, and used as it's data layer

##Technologies

- [libp2p](https://libp2p.io/)
- [Yrs](https://github.com/y-crdt/y-crdt/tree/main/yrs)

## Differentiating Planned Features:

- Does not rely on blockchain tech like [Ceramic](https://ceramic.network/)
- Bindings can be written in multiple languages, isn't a js-only project like [Gun](https://gun.eco/) or [OrbitDB](https://github.com/orbitdb/orbit-db)
- Isn't meant to be used as a part of a web application server like with the [Braid](https://braid.org/) family of projects.
- Native code with safety guarantees thanks to Rust
  - Highly performant and perfect for resource limited devices
- Options for data locality, ie you can specify a list of trusted peers for data to be served from in order to guarantee deletes
- Batteries included: 
  - user auth + adding multiple devices to one identity
  - peer discovery, relay servers for nat traversal
  - data access control

# InductionDB

Realtime collaborative P2P database library

- Realtime: changes are propagated with low latency
- Collaborative: designed for multiple agents to change the same data structure simultaneously, without overwriting each other's changes
- P2P: no centralized or federated servers are needed, the application can effectively host itself
- Database: can query it's own data, and save it atomically with consistentcy and durability guarantees
- Library: can be embedded into a larger app, and used as it's data layer

Technologies:

- [libp2p](https://libp2p.io/)
- [Yrs](https://github.com/y-crdt/y-crdt/tree/main/yrs)

More details soon

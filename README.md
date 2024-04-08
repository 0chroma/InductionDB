# InductionDB

InductionDB is an easy to use database library that helps you write collaborative, decentralized applications.
It abstracts away the complexities that come with data syncing and p2p, so you can focus on building your app
instead of managing it's data.

Since it's written in Rust, it can be used anywhere, but is primarily meant for applications that can run
persistently. In theory it can be used with web and embedded applications, and could work over transports
like WebRTC, Bluetooth, or LoRa.

The word "induction" in InductionDB describes both how each replica influences each other (like in
electrical induction), as well as how its CRDTs store changes to data (as in proof-by-induction).

## Structure

### Manifolds

Since there are many different types of CRDTs, ranging from simple numerical counters to fully queriable
embedded databases, InductionDB doesn't have a concept of "documents" like you may be familiar with from
other databases. Instead, the base unit of data in InductionDB are called manifolds. A manifold has a
key that you can use to access it's data just like you would in a key-value store, the difference being
that multiple instances of that manifold can exist across different peers.

The first time a manifold is requested, InductionDB will create it's own instance of the manifold and replicate
it's data from other peers. Once initialized, you can mutate the manifold instance using it's underlying CRDT's
APIs. InductionDB will automatically handle syncing changes to it's instance of the manifold to other peers.
You can also subscribe to updates when you receive changes to the manifold from other peers.

### Validators

Since peers in a p2p system can't always be trusted, validators are used to ensure that a change to data is
valid. For example, if you were writing a game, you could translate the rules of your game into validators
for any changes coming from a remote peer. You could also use validators to enforce data schemas when using
unstructured document-based CRDTs, or validate that a given peer is allowed to change specific data.

You can also use validators to reject changes from older versions of the application. This ensures that all
changes will be on the latest schema, and data won't be corrupted from version mismatches.

In the future, validators could be used to implement a "slow-mode" for when there are too many clients
changing a manifold at once. You could have a validator that drops changes from peers that send them
too quickly, or drop changes that aren't from a specific subset of peers.

### Decentralized migrations

When you need to change your data's schema, you can use InductionDB's migration system to ensure that
migrations can be applied across the entire network safely and without causing network congestion.

### Replication Control

In a centralized application, you might want to restrict who can view specific data with access control.
In InductionDB, we do this by restricting which peers are allowed to replicate a given manifold through
replication control.

Your application will need to specify how these peers are decided using something similar to the validator
API. The recommended way is through consensus-based cryptographic signing of a new membership from
a majority of other peers. There will be a few pre-implemented options you can use without having to write
your own.

Trust is particularly important in p2p applications, since a request to delete data requires good faith on
the part of the peers that have replicated it. It's important to be mindful of this when deciding on how
replication should be controlled.

### Users

A user might have many different devices that they use the same application from. InductionDB provides tools
for a user to group multiple peer IDs together for use in validators or replication control.

### Peer Discovery and Relays

Peer discovery is done through a set of trusted servers and through gossip across the network. Relay
servers can also be set up for NAT traversal/pseudo-privacy.

## How does this work? I want to learn more!

The following links should give you some basic knowledge about the underlying technologies used:

- [dotJS 2019 - James Long - CRDTs for Mortals](https://www.youtube.com/watch?v=DEcwa68f-jY&t=0s)
  - [CRDTs: The Hard Parts](https://www.youtube.com/watch?v=x7drE24geUw) for further reading
  - [Braid's Demo](https://braid.org/demo/interact) for an interactive demo
- [libp2p DHT](https://curriculum.pl-launchpad.io/curriculum/libp2p/dht/)
- [libp2p Publish/Subscribe](https://docs.libp2p.io/concepts/pubsub/overview/)
  - [Demystifying libp2p Gossipsub](https://www.youtube.com/watch?v=BUc4xta7Mfk) (if you prefer video form)

## Differences from other projects

- Does not rely on blockchain tech like [Ceramic](https://ceramic.network/)
- Bindings can be written in multiple languages and is wasm compatible, isn't a js-only project like [Gun](https://gun.eco/) or [OrbitDB](https://github.com/orbitdb/orbit-db)
- Isn't meant to be used as a part of a web application server like with the [Braid](https://braid.org/) family of projects.
- Native code with safety guarantees thanks to Rust
  - Highly performant and perfect for resource limited devices
- Batteries included unlike [Yjs](https://yjs.dev/), [Automerge](https://automerge.org/), and [cr-sqlite](https://github.com/vlcn-io/cr-sqlite)

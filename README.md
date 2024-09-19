# Graphia

A simple graph data structure.  

📣 _Built for [Pax](https://github.com/paxdotdev/pax): a user interface engine with an integrated vector design tool, built in Rust._

Graphia's `Graph` tracks nodes as `Arc<Mutex<_>>` and edges with `Weak<_>`

This separation of `Arc` / `Weak` evades memory leaks that might otherwise happen with `Arc` cycles, while maintaining the flexibility, memory management, and distinct cloning characteristics of `Arc`.  As a result, this library is very simple and has no dependencies.

## Usage

See the [tests](https://github.com/paxdotdev/graphia/blob/master/src/lib.rs#L80)

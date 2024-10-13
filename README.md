Basalt is a set of utilities for knowledge management in the terminal.

# Functionality

- [ ] Create new notes
  - [x] Manually
  - [ ] With default setup
  - [ ] From templates
- [ ] Forward links
  - [x] Traversable
  - [ ] Canonical target for linking to other notes
  - [ ] Autocomplete
- [ ] Back links
  - [x] Get list for link
  - [x] Traversable
  - [ ] Template to display note backlinks
- [ ] Search
  - [x] Possible (use any fuzzy finder)
  - [ ] Canonical method
- [ ] Live preview
  - [x] Real time
  - [ ] Forward search (like synctex)
  - [ ] Backward search (like synctex)
  - [ ] Focus on active note
- [ ] Full vault export
  - [x] As pdf
  - [ ] As HTML
- [ ] Single note mode
  - [ ] Live preview
  - [ ] Links
  - [ ] Full vault export
- [ ] Query API
  - [ ] Links to document
  - [ ] Links in document
  - [ ] Character count
  - [ ] Word count
- [ ] Graph view
- [ ] User defined metadata

# Design

Basalt is split into two parts: a CLI written in Rust and a Typst library.

When possible, functionality should be implemented in the Typst library and be
usable in a standalone mode without using CLI. The CLI should be a convenience,
and only include functionality that isn't possible to achieve with pure Typst.

# Extensibility

For the CLI, all you have to do is run another program on your vault. I don't
plan on providing any sandboxing, though, so you have to be careful. Hopefully,
with the query API, a standard set of information about the vault can be made
available for programs to use.

For the Typst library, every part is modular. Places to hook in will be
plenty, and metadata about the vault will be generated in the document. And,
since state markers are namespaced globally, one could interface with the low
level implementation details at the risk of it changing in the future.

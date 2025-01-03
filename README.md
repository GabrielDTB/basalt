Basalt is a set of utilities for knowledge management.

# Design

Basalt is split into two parts: a CLI written in Rust and a Typst library.

When possible, functionality should be implemented in the Typst library and be
usable in a standalone mode without using CLI. The CLI should be a convenience,
and only include functionality that isn't possible to achieve with pure Typst.

# Functionality

- [x] Create new notes
  - [x] Manually
  - [x] With default setup (formatters)
  - [x] From templates (formatters)
- [ ] Forward links
  - [x] Traversable (xlink)
  - [x] Canonical target for linking to other notes (meta)
  - [ ] Autocomplete
- [ ] Back links
  - [ ] Get list for link
  - [ ] Traversable
  - [ ] Template to display note backlinks
- [ ] Search
  - [x] Possible (use any fuzzy finder)
  - [ ] Canonical method
- [ ] Live preview
  - [x] Real time
  - [ ] Forward search (like synctex)
  - [ ] Backward search (like synctex)
  - [x] Focus on active note (single-note mode)
- [ ] Full vault export
  - [x] As pdf
  - [ ] As HTML (waiting on upstream)
- [x] Single-note mode
  - [x] Live preview (individual pdfs)
  - [x] Links (xlink, as-branch)
- [ ] Query API
  - [ ] Links to document
  - [ ] Links in document
  - [ ] Character count
  - [ ] Word count
  - [ ] User defined metadata
- [ ] Graph view

# Priorities

Features are prioritized by how useful I find them, how feasible they are to implement,
and how necessary Basalt is for them to exist. Here's a rough ordering:

- [ ] Back links
  - [ ] Get list for link
  - [ ] Traversable
  - [ ] Template to display note backlinks
- [ ] Graph view
- [ ] Query API
  - [ ] Links to document
  - [ ] Links in document
  - [ ] Character count
  - [ ] Word count
  - [ ] User defined metadata
- [ ] Forward links
  - [ ] Autocomplete
- [ ] Search
  - [ ] Canonical method
- [ ] Live preview
  - [ ] Forward search (like synctex)
  - [ ] Backward search (like synctex)
- [ ] Full vault export
  - [ ] As HTML (waiting on upstream)

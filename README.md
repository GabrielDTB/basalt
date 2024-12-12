Basalt is a set of utilities for knowledge management.

It's broken up into 3 parts:

+ A Typst library that provides most of the functionality
+ A command line interface that provides ergonomics for managing a set of notes (a vault)
+ Any editor that can be glued into this system

Basalt is made to be agnostic to what layout you use for your vault and what content you put in your notes.
However, it's currently required to put the following at the top of a note:

```typ
#import "<PATH_TO_LIBRARY>/basalt.typ": note
#note()
```

It's recommended to attach some metadata to the note for cross-note linking.
In particular, I recommend to attach a UUID that you never change.
If you want to link to a document by name, you'll have to attach a name metadata as well.
This is a limitation from Typst, as it doesn't provide ways to interact with the filesystem from within Typst.

```typ
#import "<PATH_TO_LIBRARY>/basalt.typ": note
#note(
  uuid: "rupip-gozus",
  name: "Some Note",
)
```

You can add arbitrary metadata fields.
These metadata fields are exposed to the Typst query interface.

To link to other notes, use xlink.

```typ
#import "<PATH_TO_LIBRARY>/basalt.typ": note, xlink
#note(
  uuid: "rupip-gozus",
  name: "Some Note",
)

Here's a #xlink(uuid: "tunur-kufun-josun")[link] to another note.
```

Currently, all notes need to be \#included in a top level document.
Then you compile that top level document, and view it in a PDF viewer that supports live updating.
Alternatively, I've heard that VSCode has a good preview plugin for Typst.

I want to get more documentation coverage for what's currently implemented, how to use it, and what's left.
Though since I'm currently building this for myself, the interface is just whatever I find convenient for my use.
I'd like for the documentation to be self-hosted with Typst, but web export of Typst is still [in the incubator](https://github.com/typst/typst/issues/5512).

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
  - [ ] User defined metadata
- [ ] Graph view

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

# Priorities

Features are prioritized by how useful I find them and how necessary Basalt is
for them to exist.

## Tier 1

- [ ] Forward links
  - [ ] Canonical target for linking to other notes
  - [ ] Autocomplete
- [ ] Query API
  - [ ] Links to document
  - [ ] Links in document
  - [ ] Character count
  - [ ] Word count
  - [ ] User defined metadata
- [ ] Graph view

## Tier 2

- [ ] Live preview
  - [ ] Forward search (like synctex)
  - [ ] Backward search (like synctex)
  - [ ] Focus on active note
- [ ] Full vault export
  - [ ] As HTML
- [ ] Single note mode
  - [ ] Live preview
  - [ ] Links
  - [ ] Full vault export

## Tier 3

- [ ] Create new notes
  - [ ] With default setup
  - [ ] From templates
- [ ] Search
  - [ ] Canonical method
- [ ] Back links
  - [ ] Template to display note backlinks

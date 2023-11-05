---
title: RSS and Atom
slug: norss
started: 2023-11-05
published: 2023-11-05
tags: [blog-meta, rust, xml, rss, atom, rant]
public: true
---

# Why there's no rss feed for this blog

I'm taking a stance against RSS. The format itself is great, and I make use of
"RSS Readers" to consume a lot of content, but RSS itself is a dead format to
me.

## What's wrong with RSS?
RSS is an incredibly old format and it's not used for it's initial purpose. RSS
was initially envisioned as a format named RDF; intended to be used to summarise
the content of a site. RSS evolved out of this format, and still carries some
baggage because of it.

### RSS Shortcomings

- Not formalised by a traditional standards organisation
- Multiple, subtly incomparable versions
- Does not take advantage of good and useful XML features like namespaces,
  schemas or modern features like the ``lang`` attribute.
- Uses RFC-822 for datetimes which has since been superseded by RFC-3339 and
  ISO-8601
- Does not have a registered MIME type
- Parts of the RSS XML vocabulary collide with now standard XML vocabulary.

## What should I use instead?
Atom!

## Why?

Atom is a superior format in almost every respect, solving every shortcoming
addressed above! There are no modern feed readers that do not support atom.

This blog provides an Atom feed, in the future it might provide a
[jsonfeed](https://www.jsonfeed.org/) but this is not well supported by most
feed readers.

## While writing this

### Music
- [The Greatest Show On Earth by
  Nightwish](https://www.youtube.com/watch?v=BMbPFqkTEfQ)

### Coffee

[Mud Flood by Dark
Arts](https://www.darkartscoffee.co.uk/collections/filter/products/mud-flood-peru-1)

Clever dripper, 24 clicks on Commandante C40 MK4.

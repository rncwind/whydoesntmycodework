---
title: Building a Blog
slug: howitsmade
started: 2023-04-16
tags: [rust, axum, blog-meta]
---

# How The Sausage is made

I read a _lot_ of technical blogs. I love reading about how people make
interesting, and often complex software.

Two of my favorite blogs are from [Xe Iaso](https://xeiaso.net/) and
[Amos/Fasterthanlime](https://fasterthanli.me/), and their articles on how they
made their blogs were what made me realise how interesting the project would be.

# The Stack

## Language

I really like Rust. In fact I'm lucky enough to be employed writing rust (in a
job that doesn't involve crypto currency! a true rarity), as such i'm pretty
well aquainted with it's ecosystem. 

I have my critiques of the language (HKTs when?) but for my preferences it sits
in a very happy position, giving me good performance whilst also having a
rigorous ML Inspired type system.

Whilst I doubt any post I make will ever require huge throughput capabilities,
the fact that Rust was designed to make concurrency easy makes it a very
attractive platform for web applications, especially when it's main contender is Go.

## Web Application Frameworks

I have a good amount of experience using
Hyper, and previously used [Actix](https://actix.rs/) at university for a project.

I had issues with Actix, namely the _really bad_ compile times, I'm unsure if 
this was resolved at a later point, but it soured my opinion of the framework.

I have in the past made little toys using Warp and Rocket, but each of them have
their own issues.
Warp encodes all routes into types, whilst a really cool idea, the issue is it
makes compile times really bad, and also gives you **HUGE** error messages.
Rocket on the other hand, looked promising, but also seems to mostly be
maintained by a single person at this point, which does not instil confendence.

As a result, I have opted to use Axum for this project. Axum is pretty neat, in
that it was designed to make heavy use of the ``tower`` ecosystem for its
middleware, which makes it surprisingly composable.
Axum is also one of the few Rust WAFs that I've seen _not_ use macros for it's
routing, instead opting for relativley plain structs.

## Templating

Maud.

## Parsing


## Config

Dhall is based

## Build system

Nix curse

# Design Decisions

Moral Linked List.

# The Future

Right now, all of this prose was written in Markdown. Whilst I don't _mind_
markdown, and everyone seems to use it, it's not my prefered "fancy plain text"
format.

In the future, I'm planning on converting the blog to run off of
(org-mode)[https://orgmode.org/] files.

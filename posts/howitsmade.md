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

Rust has a fair few templating libraries. I previously used [Ructe](https://github.com/kaj/ructe), which is a
compiled templating language for Rust. It's _really_ fast, it compiles your
templates into native code and serves them out of memory. This is awesome,
however, the templating language itself leaves a lot to be desired.

All of your template has to be annotated with types, which is just a bit gross,
and the templates end up looking like a lovecraftian mix of rust and html, as an
example.

```html
@(params: &[(&str, usize)], title: &str)
<!doctype html>
    <head>
        <title> @title </title>
        <meta charset="UTF-8">
    <head>
    <body>
        <h1> @title </h1>
        <ul>
        @for p in params {
            <li> @p </li>
        }
        </ul>
    </body>
</html>
```

Also annoyingly, it means we're leaving the nice and cozy rust editing context,
and entering my less cozy, more pointy-angle-bracket filled world of ``.html``
files, it's also somewhat brittle and not particularly composable.

It's rather well known that rust has pretty good support for macros. After
reading around and looking at some other rust-based blogs, I stumbled upon
[Maud](https://maud.lambda.xyz/).

Maud is a ~~My little pony joke~~ HTML template engine for rust, that is
implemented as a procedural macro. This means that it embeds a templating DSL
that maps to HTML within rust itself!

This is great, because it means that rust-analyzer works with it, and our
templates are "just normal rust functions" that return some ``Markup`` type.

The same template can be implemented in maud as

```rust
use maud::{DOCTYPE, html, Markup}
pub async fn head(title: String) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (title) }
        h1 { (title) }
        ul {
            @for p in params {
                li { (p) }
            }
        }
    }
}
```

Maud also eschews "partials" like in other templating languages, and favours
function composition, letting us re-write the above in a much nicer way!

```rust
use maud::{DOCTYPE, html, Markup}
pub async fn head(title: String) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (title) }
    }
}

pub async fn body(params: Vec<String>) -> Markup {
    html! {
        ul {
            @for p in params {
                li { (p) }
            }
        }
    }
}

pub fn page(title: String, params: Vec<String>) -> Markup {
    html!{
        // Add the header
        (header(title))
        
        // append a <h1> </h1> with the title
        h1 { (title) }
        
        // Add the body
        (body(params))
    }
}

```

Oh, and because it's a macro, just like with Ructe the templates get compiled to
native code so still run _stupidly_ fast.

## Parsing


## Config

Dhall is based

## Build system

I fell for the reproducability ambush and now i'm cursed to use NixOS. Well,
it's less of a curse more of a monkey's paw.

# Design Decisions

Moral Linked List.

# But why do this?

I wanted to learn! I love messing about with new tools I've not touched before.
I initially was planning on using a SSG like Jekyl or Hugo, but decided doing it
this way would be a lot more fun!

In doing this, I got to mess about with a load of tools i've not used before,
Axum, Maud, Comrak and Dhall were all new to me before this and I've learned a
load about them.

# The Future

Right now, all of this prose was written in Markdown. Whilst I don't _mind_
markdown, and everyone seems to use it, it's not my prefered "fancy plain text"
format.

In the future, I'm planning on converting the blog to run off of
(org-mode)[https://orgmode.org/] files.

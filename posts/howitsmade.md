---
title: Building a Blog
slug: buildingablog
started: 2023-04-16
published: 9999-12-31
tags: [rust, axum, blog-meta]
---

I read a _lot_ of technical blogs. I love reading about how people make
interesting, and often complex software.

Two of my favourite blogs are from [Xe Iaso](https://xeiaso.net/) and
[Amos/Fasterthanlime](https://fasterthanli.me/), and their articles on how they
made their blogs were what made me realise how interesting the project would be.

## The Stack

### Language

I really like Rust. In fact, I'm lucky enough to be employed writing rust (in a
job that doesn't involve crypto currency! a true rarity), as such I'm pretty well
 acquainted with its ecosystem.

I have my critiques of the language (HKTs when?) but for my preferences it sits
in a very happy position, giving me good performance whilst also having a
rigorous ML Inspired type system.

Whilst I doubt any post I make will ever require huge throughput capabilities,
the fact that Rust was designed to make concurrency easy makes it a very
attractive platform for web applications, especially when its main contender is Go.

### Web Application Frameworks

I have a good amount of experience using Hyper, and previously used [Actix](https://actix.rs/) at university for a project.

I had issues with Actix, namely the _really bad_ compile times, I'm unsure if 
this was resolved at a later point, but it soured my opinion of the framework.

I have in the past made little toys using Warp and Rocket, but each of them has its issues.
Warp encodes all routes into types, whilst a really cool idea, it
makes compile times really bad, and also gives you **HUGE** error messages.
Rocket, on the other hand, looked promising but also seems to mostly be
maintained by a single person at this point, which does not instill confidence.

As a result, I have opted to use Axum for this project. Axum is pretty neat, in
that it was designed to make heavy use of the ``tower`` ecosystem for its
middleware, which makes it surprisingly composable.
Axum is also one of the few Rust WAFs that I've seen _not_ use macros for its routing, instead opting for relatively plain structs.

### Templating

Rust has a fair few templating libraries. I previously used [Ructe](https://github.com/kaj/ructe), which is a
compiled templating language for Rust. It's _really_ fast, it compiles your
templates into native code and serves them out of memory. This is awesome,
however, the templating language itself leaves a lot to be desired.

All of your templates have to be annotated with types, which is just a bit gross,
and the templates end up looking like a lovecraftian mix of Rust and HTML, as an
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
and entering my less cozy, more pointy-angle-bracket-filled world of ``.html``
files, it's also somewhat brittle and not particularly composable.

It's rather well-known that Rust has pretty good support for macros. After
reading around and looking at some other rust-based blogs, I stumbled upon
[Maud](https://maud.lambda.xyz/).

Maud is a ~~My little pony joke~~ HTML template engine for Rust, that is
implemented as a procedural macro. This means that it embeds a templating DSL
that maps to HTML within rust itself!

This is great because it means that rust-analyzer works with it, and our
templates are "just normal rust functions" that return some `Markup``` type.

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

### Parsing

I don't want to be writing my prose as HTML. Thankfully markdown is a very well-supported
format these days (I blame Reddit), and there's a very nice and ergonomic
parser library in the form of Comrak.

Comrak is a _fantastic_ library, it was able to parse any markdown I threw at
it, from tables, to code blocks and even skip over the YAML frontmatter that my
posts start with!

The latter part was especially important, this document starts with a big block
of YAML which contains various metadata about this blogpost. As a concrete
example, this was the yaml at the time of writing.

```yaml
title: Building a Blog
slug: buildingablog
started: 2023-04-16
published: 9999-12-31
tags: [rust, axum, blog-meta]
```

As you can see, the blog title (at the top of the page), the slug (what the
route for this specific post is), when I started writing it, when it's set to be
published, and finally, what tags this post has.

Some of this has little use at the time of writing (The distinction between started/published
doesn't matter, and the tags are not displayed), but it leaves me room for
further development which is nice.

On the subject of YAML, by choosing rust for this project I get to use the
wonderful ``serde`` crate for Serializing and Deserializing data. ``serde`` is
heavyweight, but it is incredibly powerful. 95% of the time you throw a
``#[derive(Deserialize)]`` annotation on a structure, and serde will create the
deserializer automagically, now it's up to you to just give the
what-you're-deserializing-from function a string containing your metadata.


Unfortunately,
as mentioned earlier, the YAML is frontmatter. That means it's _in band
signalling_. To get around this, all of the yaml is "fenced" at the start of the
file with a ``---``. The code that extracts this is a rather ugly, but it's
replicated here in case you're interested.


```rust
fn new(content: &str) -> Result<FrontMatter, PostParseError> {
    let matches: Vec<_> = content.match_indices("---").collect();
    if matches.is_empty() {
        Err(PostParseError::NoFrontmatter)
    } else if matches.len() == 1 {
        Err(PostParseError::UnterminatedFrontmatter)
    } else {
        let start = (matches[0].0) + 3; // Skip over the first 3 ---
        let end = matches[1].0;
        let slice = &content[start..end].to_string();
        info!("{}", slice);
        match serde_yaml::from_str(slice) {
            Ok(x) => Ok(x),
            Err(e) => {
                error!("{}", e);
                Err(PostParseError::FrontmatterError)
            }
        }
    }
}
```


There are probably better ways to solve this problem, but I wrote this code at
11PM and Just Wanted It Done ™️. The one downside of this, is that it will
_always_ use the first two sets of ``---``, but that's unlikely as by convention
the first thing in any of these markdown documents is the yaml frontmatter.

Honestly this whole section exists as a test case for this horrible parsing
code, so thank you dear reader for participating.

### Config

I've used a lot of configuration languages in the past, ranging from the
venerable ``.ini``, to json-as-config (please do not do this) and even emacs
lisp. For this project, I wanted to try out a configuration language that I'd
read a lot about, but never used. That language is
[Dhall](https://dhall-lang.org).

Dhall has many features that make it attractive, it's programmable but
explicitly _not_ Turing complete (That is, it is
[Total](https://en.wikipedia.org/wiki/Total_functional_programming)), it's
strongly typed, and makes use of Semantic Hashing to ensure that refactors are
behaviour preserving.

All of this makes it easy to ensure that configuration related outages are much
more difficult, providing a similar "if it compiles it works" guarantee to
Haskell or Rust.

All of this is powered by the
[serde_dhall](https://crates.io/crates/serde_dhall) crate which allows us to
directly serialise or deserialise our config files to rust structs, without a
go-between in the form of yaml or json.

### Build system

I fell for the reproducibility ambush and now I'm cursed to use NixOS. Well,
it's less of a curse and more of a monkey's paw.

For those of you not in the know, Nix is a pure functional, declarative package
manager and build system. By using Nix and Nix Flakes you can create a rather
easy to use (in the dwarf fort kind of way) hermetic build system.

A deep dive into Nix is beyond the scope of this article, but is something I
intend to write in the near future.

As a result of using Nix, I chose to use Naersk as the build system for this
project. Naersk is a rust build system for Nix. It's simple to use, providing a
sane default flake that Just Builds Stuff ™️. It is slightly more limited than
other rust builders available for nix (such as Crane) in that it does not
support cross-compilation. This is not an issue for this use-case as the binary
will be running on x86_64.

Naersk works well, the current build script is rather simple, compiling the rust
binary, and using ``symlinkJoin`` to link in the static content (CSS) to create
a final Derivation.

This was my first experience using Naersk in anger, and it was an enjoyable
experience. It was simple to use, and very easy to compose into a larger
derivation.

## Design Decisions

### Software Design

In order to keep the codebase somewhat simple, all posts are stored in a ``Vec`` and
iterated until the passed in slug is matched. This means that morally, all posts
are stored in a linked list.

Some of you might have just recoiled in horror, but this was an intentional
design decision. The load characteristics of a blog are somewhat unique in that
a vast amount of traffic is directed at the most recent post. This means that if
we optimise for this case, we can keep the code for loading a page _very_ simple
whilst also being very performant in most cases!

Speaking of storing posts, Initially I had planned on only reading the posts
once, at application startup. This would be great, because it would be pure
after startup. On the other hand, it would require restarting the server every
time a new post was written!

In order to get around this, initially I just threw a ``tokio::sync::Mutex`` on
it, however after thinking about it for a bit, I realised that it would result
in mutex contention when reading posts which we really do not want! As a
consequence I discovered [ ``tokio::sync::RwLock`` ](https://docs.rs/tokio/latest/tokio/sync/struct.RwLock.html) which allows N many threads to
read from a resource, but only one to write. This is a perfect fit for our use case!

Of course, this necessitates a way to trigger a refresh of the posts. In order
to handle this, a very simple, single REST endpoint was created that takes a
token. If that token matches what was generated by the server on startup, it
will refresh. This is an incredibly simple authentication method, but it is
acceptable as the worst case is that someone could trigger a DoS if they worked
out the token.

Out of interest, I profiled the memory usage by using ``heaptrack``. Serving the
homepage, the post list, and then this post resulted in a peak memory usage of
1.1MiB, which is pretty good all things considered.

### Web Design

At my day job, I'm a backend/embedded engineer, and any front-end devs in the
are probably laughing at how obvious that statement is given the design here.

I'm not a web developer. My HTML and CSS knowledge is mostly limited to
what I picked up throughout a CS Degree. This means I know enough to
know I know nothing.

As such, this website uses minimal HTML and CSS, and all dynamic parts of the site
(as of writing this) are done server-side and templated out. I know that
server-side rendering is somewhat passé in the era of Angular/React/Vue etc, and
at some point, I endeavour to learn them, but for something "simple" like this
blog, it seemed rather overkill.

With all that said then, it should come as no surprise to most readers that over
the course of writing this whole project, I learned quite a bit of "modern" and
"new" (to me at least) CSS and HTML tricks!

A list of New To Me web stuff.

- The ``<nav>`` Element
- CSS Variables
- CSS Selectors like ``last-child``


Oh, and lest I forget, the colours of this site are based on the fantastic
[Horizon](https://horizontheme.netlify.app/) colourscheme. I use a [modified
version](https://github.com/aodhneine/horizon-theme.el) of this theme in [Doom
Emacs](https://github.com/doomemacs/doomemacs) which is the Text Editor I wrote
all of this in!

## But why do this?

It's funny, one of the first questions I got asked when i mentioned to some of
my friends that I was building my own blog was "why not just use a SSG".

The answer Is that firstly, I wanted to learn. I love messing with new tools
that I've not used before, and this was a great opportunity to do that, and
secondly, I'd already tried a few SSGs but didn't really like any of them.

I'm glad I chose this path, since it's resulted in me using a lot of tools that
otherwise would have passed me by.

## The Future

Right now, all of this prose was written in Markdown. Whilst I don't _mind_
markdown, and everyone seems to use it, it's not my preferred "fancy plain text"
format. As such, I'm planning on converting the blog to run off of
[org-mode](https://orgmode.org/) files.

There are no parsers for org-mode that are as good as Comrak is for markdown,
and so undertaking this will be a challenge, especially as org-mode files, like
the editor they were designed for, are a far larger specification to implement
than Markdown.

As such, if I do decide to undertake this, it will probably become a blog series!

Another thing I'd like to implement at some point is Socratic Dialogue. I
really enjoy it as a style of writing, especially for explanatory and exploratory writing.

Implementing this will probably be somewhat complex, as there is no support for
it in markdown or org-mode by default. As such, I will probably end up writing
an Extension for Comrak, or baking it into my homebrew org-mode parser.

I'm also a Habitual RSS user (In 2023? I know!), and as such I would like to
implement RSS support.

## While Writing This

I hope for this to become a recurring feature in all my blog posts.

### Music 

- [Transgender Dysphoria Blues](https://www.youtube.com/watch?v=f3isaRfr9aY)
- [Northen Exposure](https://www.youtube.com/watch?v=aaY3spCDdpY)

### Coffee 

Pink by [Kofra](https://www.kofra.co.uk/coffee) and [Costa Rica El Perezoso
 Natural](https://strangerscoffee.com/coffee/costa-rica-el-perezoso-natural).
Clever Dripper. 20 clicks on Commandante C40 MK4.

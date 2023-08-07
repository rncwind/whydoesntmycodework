---
title: Adding Feeds
slug: addingfeeds
started: 2023-07-28
published: 2038-01-19
tags: [blog-meta, rust, xml, rss, atom]
---

# Feeding the hand that bites

The fact I've started my own blog should tell you a lot about the kind of
content I consume, and how I consume it.

I have a rather extensive Feed collection which I read with Fluent Reader on
desktop, and Feeder on Android.

One of the things that really sucks about Feeds though, is that they are old,
and as such specified using XML. I do not like XML.

Or rather, on a technical level I do like XML. It has advantages over other
formats such as JSON due to it's tag system; however that same tag system is
what makes it complex to parse, reason about and annoying to write.

The Atom feed should now be available [Here](whydoesntmycode.work/feeds/atom.xml)

## RSS, Atom and JSONFeed Oh My!
When deciding to implement feeds on this blog, I stood at a crossroads. There
are 3 feed formats in common usage on the web, of which two are well supported
(RSS and Atom).

RSS is the oldest of the three formats, first released in 1999, and as such has
a fair bit of technical cruft as it is based around earlier XML versions than
what are in common usage today. Atom was standardised by the W3C in 2005, and
aimed to fix many of the issues that were present with the earlier RSS
specifications.

JSONFeed on the other hand, is a relative newcomer, it's popular amongst the
[IndieWeb](https://indieweb.org/) crowd. JSONFeed recognised that RSS and Atom
were complex, XML based languages that didn't _really_ take advantage of the XML
format. Because JSON is simpler than XML, it's also easier for an individual to
make their own feeds.

I have made the conscious decision to not support RSS, and instead support Atom.
The only feed reader that I am aware of which does not support Atom out of the
box is Emacs' Gnus (It can however use ¬`nnatom`¬ with a little bit of elisp).
Because of this, there is almost zero incentive for me to duplicate work and
support a older, worse format.

JSONFeed is the inverse, it is quite a nice format, and is something I will
probably implement further down the line, however it has next to no reader
support! If you do use jsonfeed, let me know by emailing any address at this
domain!

## How did you implement it?
It's literally just string concatenation!

The start of a Atom feed is very much boilerplate and is the same every time. It
sets out the XML namespace, the source of the feed, the feed's title, who wrote
it, and what software generated it.

After that, we go through each post, and build up a string for an Atom Entry,
dump it's content as XML CDATA and finally close out the opening ``<feed>`` tag.

By the end of it, we've got every post XMLified by the magic of rust's
``format!`` macro and a single loop.

This is probably not the best way to do it; in fact I know it's not, but I did
not want to have to add, and learn, an XML library in order to provide a simple,
highly structured XML document. I'm glad that Atom only requires us to write
XML, rather than parse it!

### Optimization

The first parse "rendered" this XML each time the page was hit. This is
obviously very expensive.

This codebase has two points at which the feed would change. When the program
starts up, and when a refresh is triggered by me. Because of this, it was rather
simple to memoize the generation of the feed.

The big blob of state which stores the Post vector was adjusted to also store a
string which contained the Atom feed.
When the application starts, or when a refresh is triggered, the atom feed is
generated and stored. When a user requests the feed, they are served the
contents of that string out of memory.

Doing it this way is slightly more complex (Requires us to RwLock the feed's
in-memory representation), but ensures that everything is served out of memory
and all "dynamic" components to the site are hidden from the user.

## An Aside, X-Clacks-Overhead
When I was working on this, I also remembered a non-standard HTTP header that I
often see included on hobbyist sites, that of X-Clacks-Overhead. It initially
started as a reference to, and a way to immortalize Terry Pratchett after he
passed.

If you have not read the discworld books, The Clacks is a semaphore system used
to send messages long distance quickly. This is the key plot point of the book
Going Postal, in which the Postal System is made to compete against the Clacks.

In the book, it is mentioned that the inventor of The Clacks system immortalised
his sons name inside the system by crafting a message which started with GNU.

- G: Send the message to the next tower
- N: Do not log this message to your logbook
- U: Once you reach the end of the network, send it back.

Similarly, fans of discworld adopted this message, and added this non-standard
header to their webpages in order to memorialise, and immortalize people who
tragically have passed.

This website now also sends a X-Clacks-Overhead message of it's own. This will (sadly)
likely evolve over time.

## While writing this

### Music
[Bemani Symphony](https://www.youtube.com/watch?v=UFnU1G53330&list=PL85SVkB-NjYLtemXCtGR9IYXhzBLCJxWV)

### Coffee
[THE COSMIC KEY from Dark
Arts](https://www.darkartscoffee.co.uk/collections/filter/products/the-cosmic-key-colombia)
Clever dripper, 22 clicks on a commandante C40 MK4.

This one is really weird, and the initial tasting notes are strongly of Concord
grape (Artificial/Jolly Rancher grape flavour) which exits quickly and leaves a
well balanced body behind.


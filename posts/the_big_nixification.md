---
title: Nixifying This Blog
slug: blog_nixification
published: 2023-11-06
tags: [rust, axum, blog-meta]
public: true
---


# The Nixification of This Blog

When I wrote my first blogpost, about building this blog, I made note of my
usage of Nix as the build tool. One of the major advantages of Nix, is it
provides a pretty seamless devops experience, everything from your drive mounts
to configuration of individual services can be managed with only Nix and NixOS.

Unfortunately, i was previously hosting this blog on a OVH dedi. OVH does not
have good support for NixOS, and after multiple attempts at trying to install it
via rescue mode I gave up, and moved to Hetzner. Hetzner is well supported by
the NixOS community, and as such installing from Hetzner's recovery system was
rather painless.

The key advantage of using NixOS on both my server that hosts this blog, and on
my desktop is it means i can seamlessly deploy from one to the other. Any post
that is set to go live will go live the minute i push a redeploy to the system
hosting my blog.

As the system hosting this blog also provides other core infrastructure (My VTT
instance, and my mail server), it is likely to be updated pretty frequently,
meaning i can use NixOS+Deploy-rs as my CMS!

## What this means for you

More frequent updates! Previously whenever I wanted to update this blog, it
would require me to do a lot of hand cranked work.

- Install Nix if it's not there already (thanks debian)
- ``git pull`` [this blog's
  repo](https://github.com/rncwind/whydoesntmycodework)
- manually trigger a ``nix build`` in the root of the repo
- kill the systemd service that the old version of the blog was running
- start the systemd service so it's using the new blog binary

Now i just need to run ``deploy`` in my flake root and it will update both the
server, and the blog itself!

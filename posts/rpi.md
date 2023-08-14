---
title: Why small computers are awesome
slug: smallcomputers
published: 2023-08-14
tags: [nix, Raspberry Pi, rpi, homelab, nixos]
---

One of my earliest experiences using a Linux-based computer, at my desktop was
with a Raspberry Pi Model B. It was around 2013, I was still in the early years
of High School, I was learning C++ from an old "C++ For Dummies" book that I
picked up at a charity shop and my autistic hyperfixation on computing was in
full swing.

I'm lucky to have some very supportive parents, and around this time the
Raspberry Pi was taking off like wildfire in the UK, being lauded even in the
non-technical press as a good way to get children invested in programming.
Because of this, they decided to get me one.

The Original Raspberry Pi models were under powered by today's standards,
sporting a whole 256 MiB of RAM, a _single core_ ARMv7 processor, and no Wi-Fi. I
used it a bit, read some tutorials, made some LEDs blink with the GPIO, but I
started to become too swamped with work at high school to really do much with it.

Since then, it's sat in drawers collecting dust, an interesting technical
novelty that is less powerful than even my Wi-Fi router at this point. Whilst I
didn't find much use for it, others did. Each generation has become increasingly
powerful and capable. Presumably this is due to the meteoric rise in demand for
both IoT devices, and Mobile Phones, forcing innovation in fitting more and more
computation into tiny packages.

All of this is to say, last week I bought a Raspberry Pi 4B with *8 GiB* of
memory for retail price, and I am dumbstruck by how powerful, and how useful this tiny SBC is!

# Why?

I have a pretty extensive infrastructure. I own 2 physical servers (1 NAS and 1
ProxMox), 1 OVH Dedi and 2 VPSes (Which Are retiring soon, and their stuff moved
to the dedi).

As such, it's sensible to ask why I would want a Raspberry Pi since I already
Own/Rent so many devices. The answer is rather simple, I like to own my devices,
and I also despise subscription fees.

As such, there was a cost-benefit analysis to perform, so I ended up
breaking it down like so.

## VPS Pros
- You don't have to manage it or worry about uptime/availability
- You can expose it to the internet without trouble if you need to.
- amd64 is well-supported
- Can use ZFS

## VPS Cons
- You don't have to manage it and physically can't most of the time if it breaks.
- Recurring cost
- NixOS is unsupported on most VPS Providers.
- If you use OVH, [Your Server might catch on fire](https://www.theregister.com/2021/03/10/ovh_strasbourg_fire/)
- Annoying to build stuff that we don't want to expose (Have to use
  Tailscale or something similar)
  
## Pi Pros
- If something breaks I can physically intervene if required
- It can run NixOS
- Upfront cost, then power draw, is almost nothing.
- Easy to build internal infrastructure with it.
- aarch-64
- Unlikely to catch fire.

## Pi Cons
- If something breaks, and I can't fix it, it's game over
- Hard to expose to the wider internet.
- aarch-64
- No ZFS

After weighing this up, I thought it would be worth taking a punt. I waited for
a UK seller to get the 8 GiB model in stock (Because I refuse to take part in the
absolute farce that is profiteering from chip shortages by buying from a
scalper), and waited for it to arrive.

# Nixification

I am a NixOS user, and as such the first thing I did, after unwrapping and
checking the device powers on was to install NixOS.

I was prepared to have
to fight my toolchain in order to do this, but other than having to enable
`aarch-64` emulation (`boot.binfmt.emulatedSystems = ["aarch64-linux"]` on an
x86_64 NixOS machine) due to `deploy-rs` needing to build its activation binary
on my machine, it was rather painless.

## Integration with existing config

When I wrote my NixOS config for all of my current systems, I made certain
assumptions. Since the first device I wrote it for, was a desktop computer, some
of the assumptions that I made are an incredibly bad.

- All devices are x86_64-linux with GCC.
- All devices can run ZFS.
- Users will want a graphical Session
- Users will have a physical input device
- Users will want audio
- Users will want docker
- Users will want Emacs installed locally and compiled from source.

Because this device broke all of these assumptions, it forced me to
remove these assumptions. This has resulted in slimming down the minimal "just a
system" configuration by quite a large degree. After sorting out the issues with
deploy-rs, the resultant size of the closure (on top of the default NixOS
Raspberry Pi install) was an additional 1GiB. Most of this seems to come from
deploy-rs, sops, tailscale and tools that I require to feel at home on any
device (zoxide, exa etc.).

# What I use it for

So, given the pro/con list above, it should be obvious that my main interest was
in setting up "internal infrastructure" with the device. My main interests were
the following

- Universal ad-blocking
- Local DNS so I don't have to remember IP addresses
- NGINX running, so I can test web stuff locally.
- Some centralised method of collecting metrics (more on this later)
- Some way of visualising these metrics.

## Universal Ad Blocking

My Firefox setup is rather paranoid. I run uBlock, Privacy Badger, HTTPS
Everywhere, ClearURLS, FastForward, SponsorBlock and various userscripts with
ViolentMonkey. Almost all of these serve one of two purposes; either to block
ads, or to block trackers.

Unfortunately, I can't always use my desktop. I own a Fairphone 4 (Fantastic phone,
it's user repairable by design!), multiple games consoles, a FireTV stick and a
Kobo e-reader. With all of these devices it is either impossible, or difficult
to install an ad blocker on.

My phone is the least plagued by this, as Firefox on mobile supports uBlock and
Privacy Badger, but that doesn't stop adverts and tracking outside the device
itself.

What I sought then, was a method to block adverts and trackers at a "router
level" so that all devices on my network would be protected from web-based trackers
and advertising.

Many of you are probably thinking I'm now going to talk about PiHole. This was
one of the first major platforms to do this, however, it is rather heavyweight,
difficult enough to package that it's not on nixpkgs and doesn't support DoH or
DNSCrypt without some tinkering.

Instead, I opted for AdGuard Home. AdGuard provide commercial "plug and play" DNS
ad-blocking devices, however the software that actually _runs_ on these devices
is open source and licensed under GPL3! 

AdGuard has some advantages over PiHole; It's written in Go so is rather easy
to package, it supports DoH, DNSCrypt and even the less popular DNS over TLS,
and it's already packaged on nixpkgs.

## Local DNS

Funnily enough, given that AdGuard itself is a DNS Resolver, it also provides
the ability to write your own DNS resolutions. Previously, I was using the
``dnsmasq`` implementation that was part of my router, however this involved
_telneting_ in, and adjusting the ``dnsmasq`` config.

This was obviously a horrible user experience. Partly because the only reason to
 use telnet when SSH exists is to [watch Star Wars](towel.blinkenlights.nl), but
 also partly because it involved manually editing a text file, over telnet, on
 my router, and then restarting the device (Because you can't restart ``dnsmasq``
 from the telnet session!).
 
 As such, this has meant that 99% of the time I have just not done it, and stuck
 to memorising IP addresses.
 
 The way I am currently handling this, is to write an entry for each service in
 the form ``$SERVICENAME.local``, but, given the next part, this might not stick
 around forever.
 
## Local NGINX

I like NGINX, I hate configuring it. Thankfully Nix makes this a lot less
trouble since if I mess up the config file it will often error when building the
derivation which is rather convenient.

Because I intended to be hosting multiple services off of this one device, it
made sense to make use of NGINX for its actual purpose, as a reverse proxy.
This is quite nice, as it means I can make each service its own logical "domain" on the
local network, whilst still running off of one device.

For example, whilst this device hosts ``Prometheus``, ``Grafana`` (spoilers!) and
AdGuard each one has its own domain ``prometheus.local`` ``grafana.local`` and so on.

## Metrics

When I first soft-deployed this blog, I realized I had no way of collecting
metrics. I didn't want anything invasive/tracker-y, and so I sought to do what
any sane person does, roll it from scratch.

In that search, I came across Open Metrics, and thus Prometheus. There's a good
[rust library](https://docs.rs/prometheus/latest/prometheus/) for exposing an
Open Metrics endpoint, and so I decided to settle on using it.

The metrics collected are intentionally minimal, providing me hit counts, and
nothing more. The metrics are public, and are available
[here](https://whydoesntmycode.work/metrics) if you would like to inspect them
yourself.

Of course, it's all well and good exposing these metrics, but I also need a way
to consume, and visualize them! My Raspberry Pi now runs a small Prometheus
instance which reaches out to the blog's metrics endpoint, and in turn stores
those metrics in a time series database.

These metrics can then be read from Prometheus into something to visualise it. I
personally chose Grafana for this, mostly because I've used it before (for 5
minutes) and also because it's well-supported in nixos.

Eventually, this will expend to more than my blog. The goal is to build myself a
single pane of glass for all of my devices and services.

## Future work

There are some services that already run on my Venerable R710 (such as ZNC)
which I intend to move to the raspberry pi soon. There are also some new
services that I intend to spin up. Most of these are infrastructure such as Loki
and Gitea, but some are actual useful tools like PrivateBin, an encrypted
pastebin clone.

# Summary

In summary, I am incredibly impressed with the power of modern SBCs. The fact
that a device the same width and height as a credit card is able to provide this
much value to me, and my home lab setup is incredible.

This tiny device has rapidly become an indispensable asset for my
homelab, and was well worth the money.
In the future, I expect to see more and more small, useful services delegated to
this device.

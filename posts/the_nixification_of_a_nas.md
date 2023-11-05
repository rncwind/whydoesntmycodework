---
title: Breaking up with TrueNAS
slug: truenastonix
published: 2023-08-14
tags: [homelab, nix, freebsd, truenas, freenas]
public: false
---

# My breakup with TrueNAS.

Oh freenas, we spent a good few years together. Parting is bittersweet, but you
are just not for me.

For those of you unaware, Truenas (née, FreeNAS) is a distribution of FreeBSD
optimized for Network Attached Storage workloads.

TrueNAS is popular for a variety of reasons. It's simple to set up, it's got a
nice UI (as long as you're using freenas 11 onwards...), it provides a "plugin"
system that uses the power of FreeBSD's bhyve to it's full advantage and it's
also _incredibly_ stable.

It has one major, glaring downside however. It's FreeBSD.

## For better or for worse

This isn't going to be a post where i complain about FreeBSD. In fact, i'm a
huge fan of the BSD family, and FreeBSD in particular. I find FreeBSD very
enjoyable to administrate, and it is something I know is stable, secure, and
dependable in almost any situation.

With this in mind, at the time TrueNAS seemed like the perfect pitch. I'd been
wanting to experiment more with FreeBSD administration, I wanted to use ZFS, and
I had initially planned for the NAS to just be a dumb, network-accessable
storage system.

With time, my requirements for what i wanted in a NAS grew. I wanted it to be
able to download *a lot of copies of big buck bunny of of usenet* and torrent 
*a lot of linux isos*, and TrueNAS met these needs fine, as long as it was a
supported Plugin, and this is where the rub began.

TrueNAS makes heavy use of Behyve Jails. This is a really cool system

<!-- Unfortunatley for me, the latter part turned out to not be the case. -->
<!-- At first it was simple, I wanted to run a torrent client on it. At the time this -->
<!-- for me meant using rtorrent (Which I have since moved away from due to the -->
<!-- personaly opinions of the developer...). This was a good exercise, as it forced -->
<!-- me to learn how to make a jail, configure ACLs, use ports to install rtorrent -->
<!-- and lighttpd to host rutorrent with. -->

<!-- Overall it was a fine experience, and i learned a lot about FreeNAS -->
<!-- administration by doing it. However, that all changed when i wanted to add -->
<!-- usenet into the picture. -->

<!-- ## Usenet -->
<!-- In order to automate the downloading of *big buck bunny* and other *blender open -->
<!-- movies* I wanted to set up Sonarr and Radarr. -->

<!-- This stability is the main reason I was drawn to FreeBSD in the first place. I -->
<!-- wanted to use ZFS and I intended the NAS to be a dumb data storage system. This -->
<!-- is not how it panned out. -->
<!-- At first it was simple, I wanted to run a torrent client on it so i could -->
<!-- download movies like Big Buck Bunny and Linux ISOs. This was "unsupported" at -->
<!-- the time, and resulted in me learning how to make my own jail, spin up -->
<!-- ``lighttpd``, use ports to get rtorrent and finally set up rutorrent. -->

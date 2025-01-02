---
title: New Blog!
date: "Thu Jan 02 2025 04:42:58 GMT-0800 (Pacific Standard Time)" 
description: >
 So! I have a blog now. I'm not sure how often I'll be posting to it,
 but I'll probably be using it as a place to talk about nerdy things I don't usually get to talk about.
---

So! I have a blog now. I'm not sure how often I'll be posting to it, 
but I'll probably be using it as a place to talk about nerdy things I don't usually get to talk about.

## Design

At the moment it's pretty basic, but it can do some fancy syntax highlighting things thanks to [shiki](https://shiki.style)!

```rs
fn main() {
    env_logger::init();

    let event_loop = winit::event_loop::EventLoop::new()
        .expect("failed to create event loop");

    let mut app = App { state: None };
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("Event Loop Error: {e}")
    }
}
```

There's even an [rss feed](https://melody-is.gay/blog/rss)!

Everything is written in markdown and translated to a svelte component thanks to [mdsvex](https://github.com/pngwn/MDsveX).
I'll likely be swapping at least shiki out (thinking of using tree sitter) and maybe mdsvex, as I'm not super happy with how mdsvex produces markdown and the whole thing is like a house of cards.
Tbh, that's kinda how the rest of the site feels...

## What now

Anyway!
With this thing done, my site is now an MVP, aside from adding comments to blog posts (something I'm looking to add) and a distinct lack of gadgets.
Hopefully with this done I can figure out how to integrate [wasm-pack](https://github.com/rustwasm/wasm-pack) with [vite](https://vite.dev/) so I can actually start working on some Rust gadgets!
I was originally planning to upload a fluid sim but that'd need WebGPU, so it feels a bit shit to have my first major gadget be locked off if you don't use Chrome.

After that, I'll probably come back to making this blog a bit nicer. I'd like to add something to jump between sections, generally figure out a writing style, and toss in some gimmicks to my blog.
Maybe I can do something like fasterthanlime's cool bear? I dunno, but we'll see!
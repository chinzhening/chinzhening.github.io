#import "/typ/templates/post.typ": *

#show: post-template.with(
  date: "2025-05-21",
  title: "Week 1",
  tags: ("typst", "rust", "blog-engine", "dev-log",),
  description: "The project so far.",
)

= Building ssg: A Static Site Generator in Typst and Rust

== Introduction
ssg is a static site generator built using Typst and #link("https://www.rust-lang.org/")[Rust]. It transforms content written in #link("https://typst.app/")[Typst] into a polished, publishable website. By building on top of Typst's CLI-based HTML export, ssg aims to become a fully-featured blogging engine—something akin to #link("https://jekyllrb.com/")[Jekyll] or #link("https://gohugo.io/")[Hugo], but with Typst as the authoring language at its core.

The motivation behind ssg is to offer a more modern and flexible publishing workflow for Typst users, especially writers and developers who want to take advantage of static site generation without locking themselves into rigid structures.

== Development Logs
After a week of development, it's clear that ssg is just getting started. While the foundational pieces are in place, there's a lot left to build—and a lot to rethink.

One of the growing pains right now is the code structure. The current implementation is tightly coupled with the specific site structure I initially adopted. This makes the system inflexible and goes against one of the core goals for ssg: being _structure-agnostic_. To be truly customizable, ssg needs to allow users to define their own layouts and organizational patterns without being constrained by the generator itself.

Another essential feature is hot reloading. It's a must-have for any content-heavy workflow—particularly when drafting new blog posts or testing layout tweaks. Ideally, only changed files are rebuilt, dramatically speeding up development and preview cycles.

That said, I've already learned a lot in just one week of building ssg.

One thing that's stood out is how enjoyable Typst is to work with. Its clean syntax and expressive capabilities make it a pleasure for writing structured content. In fact, that's a big part of why I'm committed to making ssg work—Typst deserves a solid publishing pipeline.

Working around the current limitations of the Typst CLI has been a challenge. Many tasks require scripting and external tooling, since the CLI wasn't originally designed with full-fledged site generation in mind. One specific issue is how inline SVGs are handled: by default, they're not properly sized for dynamic resizing when the DOM content loads, and styling them for dark mode compatibility isn't straightforward.

These kinds of challenges highlight the need for a more intelligent build process. For instance, I'm planning to improve SVG compatibility—which are needed to display math equations—through #link("https://cloudinary.com/guides/image-formats/a-developers-guide-to-svg-optimization")[minification] and smarter rendering during the build. These are the kinds of behind-the-scenes improvements that can make a big difference in how Typst content looks and performs on the web.

== Roadmap
Below is a tentative roadmap for ssg. It's by no means complete but reflects the current direction of the project:

- SCSS support - Stylize your site using modular, maintainable CSS.
- Image support - Native handling of image assets.
- Image & SVG compression - Optimize assets for the web without losing quality.
- Contacts page - Basic support for contact forms or profile info.
- Decoupled build system - Separate the generator logic from the build pipeline, with deployment via a gh-pages branch.
- Hot reloading in dev mode - Automatically update the preview when files change.
- new command - Quickly scaffold new posts or sections from the CLI.
- Embeddable content - Support for rich embeds like tweets, videos, and code snippets.
- Component library - A flexible set of reusable Typst components for layouts and themes.
- Theme ecosystem - Pre-built themes to jumpstart site creation.

== Get Involved
While still early in development, ssg is evolving quickly. If you're interested in static site generation, Rust, or Typst—or if you just want a simpler way to publish content—feel free to follow the project and contribute ideas or code.

Stay tuned for updates as ssg grows into a tool that makes publishing with Typst not just possible, but powerful and fun.
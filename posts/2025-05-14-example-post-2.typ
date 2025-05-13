#import "/typ/templates/post.typ": *

#show: post-template.with(
  date: "2025-05-14",
  title: "Another Example Post",
  tags: ("oh", "my", "gosh", "it", "works", "again"),
  description: "Testing out the metadata querying.",
)

= Examples: We have too many of them.

And yet—we keep writing more.

Examples are the duct tape of documentation. They hold everything together, even when no one knows how the underlying system really works. They are copied, pasted, broken, and fixed again. They become the folklore of a codebase or a toolchain.

== Why do examples work?

- They're faster to absorb than a paragraph of explanation.
- They often show the *happy path*—how something is supposed to work.
- They give people a sense of "what right looks like."

== The risk

Examples without explanation can become dangerous. They may only work in specific versions, or rely on assumptions that go unstated. A user who copies an example without understanding it might be successful—or they might end up debugging for hours.

== The balance

The best documentation provides both:

- *An example*, to get started
- *A description*, to understand the context
- *A pointer*, for what to do when the example doesn't quite fit

== Conclusion

So yes, we have too many examples. But it's not time to stop.

It's time to write better ones.

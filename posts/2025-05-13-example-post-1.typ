#import "/typ/templates/post.typ": *

#show: post-template.with(
  date: "2025-05-13",
  title: "Example Post",
  tags: ("oh", "my", "gosh", "it", "works"),
  description: "Testing this out.",
)

= Examples: The Best Way to Learn

Examples serve many purposes. They help explain concepts, clarify expectations, and provide practical guidance. Whether you're teaching maths, learning a new programming language, testing a new workflow, or documenting a process, examples can often speak louder than paragraphs of theory.

Here's a very meta example: this post itself is an example—of an example.

== Why Use Examples?

- To clarify abstract ideas
- To give readers a head start
- To test functionality
- To make documentation more engaging

For example, if I didn't write math equations examples
$ cal(F) { f (t) } (omega) = integral_(-oo)^(oo) f(t) e^(- i omega t) dif t, $
I would never know whether my site dynamically rescales the svgs! Here are some more
examples:

The equation
$ diff/(diff t) p (x, t) = - diff/(diff x) [mu (x, t) p (x, t)] + (diff^2)/(diff x^2) [D (x, t) p (x, t)]. $
is the Fokker-Planck equation for the probability density $p(x, t)$ of the random variable $X_t$ described by the SDE
$ dif X_t = mu (X_t, t) dif t + sigma (X_t, t) dif W_t $
with drift $mu (X_t, t)$ and diffusion coefficient $D (X_t, t) = sigma^2 ((X_t, t))/2$.

== Conclusion

If you're ever unsure how to start something, start with an example. Then improve it.
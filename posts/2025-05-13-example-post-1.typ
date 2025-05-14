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
$ cal(F) { f (t) } (omega) = integral_(-oo)^(oo) f(t) e^(- i omega t) dif t => cal(F) { e^(-t^2) } (omega) = sqrt(pi) e^(-omega^2 / 4), $
I would never know whether my site dynamically rescales the svgs! Here are some more
examples:

$ diff/(diff t) p (x, t) = - diff/(diff x) [mu (x, t) p (x, t)] + (diff^2)/(diff x^2) [D (x, t) p (x, t)]. $
is the Fokker-Planck equation for the probability density $p(x, t)$ of the random variable $X_t$ described by the SDE
$ dif X_t = mu (X_t, t) dif t + sigma (X_t, t) dif W_t $
with drift $mu (X_t, t)$ and diffusion coefficient $D (X_t, t) = sigma^2 ((X_t, t))/2$.

For every finitely generated module $M$ over a principal ideal domain $R$, there is a unique decreasing sequence of proper ideals $(d_1) supset.eq (d_2) supset.eq dots.c supset.eq (d_n)$ such that $M$ is isomorphic to the sum of cyclic modules:
$ M tilde.equiv limits(plus.circle.big)_i R "/" (d_i) = R "/" (d_1) plus.circle R "/" (d_2) plus.circle dots.c plus.circle R "/" (d_n). $

Here is a result from score identity distillation, improving denoising diffusion models. After modifying the minimization objective to $cal(L)_5$, we see that it is equivalent to minimizing the _Fisher information_ of $p$ and $p_theta$.  
$ cal(F)^+ (p, p_theta) &= integral p_("sg"[theta]) (x_t^((g))) ||nabla_(x_t^((g))) log p_theta (x_t^((g))) - nabla_    (x_t^((g))) log p (x_t^((g))) || ^2 dif x_t^((g)) \
 &= EE_(z, epsilon ~ N(0, I)) [ || nabla_(x_t^((g))) log p_theta ("sg"[x_t^((g))]) - nabla_(x_t^((g))) log p ("sg"[x_    t^((g))]) ||^2 ] \
 &prop EE_(z, epsilon ~ N(0, I) ) [ ||epsilon.alt_(phi^*) ("sg"[x_t^((g))], t) - epsilon.alt_(psi^*) ("sg"[x_t^((g)    )], t) ||^2 ] = cal(L)_5. $

== Conclusion

If you're ever unsure how to start something, start with an example. Then improve it.
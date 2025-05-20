#import "/typ/templates/post.typ": *

#show: post-template.with(
  date: "2025-05-13",
  title: "Example",
  tags: ("it", "is", "working", "yay"),
  description: "This is a post that tests various features of ssg and how the generated content looks and feels.",
)

= Examples: The Best Way to Learn
Examples are powerful. They help explain abstract concepts, clarify expectations, and provide hands-on guidance. Whether you're learning a new tool, testing a build system, or checking how content renders, examples often reveal more than theory ever could.

This post is exactly that: a testbed of features, a trial run for how content looks and feels in ssg.

== Why Use Examples?
- To clarify abstract ideas
- To give yourself or readers a head start
- To verify functionality
- To make documentation and output more engaging

Take math rendering, for instance. If I didn't include math equation examples like:

$ cal(F) { f (t) } (omega) = integral_(-oo)^(oo) f(t) e^(- i omega t) dif t => cal(F) { e^(-t^2) } (omega) = sqrt(pi) e^(-omega^2 / 4), $

I'd never know if SVGs dynamically resize or style correctly in dark mode! Here are a few more examples for testing layout and math compatibility:
$ diff/(diff t) p (x, t) = - diff/(diff x) [mu (x, t) p (x, t)] + (diff^2)/(diff x^2) [D (x, t) p (x, t)]. $
This is the Fokker-Planck equation for the probability density $p(x, t)$ of the random variable $X_t$, governed by the SDE:
$ dif X_t = mu (X_t, t) dif t + sigma (X_t, t) dif W_t $
with drift $mu (X_t, t)$ and diffusion coefficient $D (X_t, t) = sigma^2 ((X_t, t))/2$.

Or, for a more algebraic example: for every finitely generated module $M$ over a principal ideal domain $R$, there exists a unique descending chain of proper ideals $(d_1) supset.eq (d_2) supset.eq dots.c supset.eq (d_n)$ such that $M$ is isomorphic to the direct sum of cyclic modules:

$ M tilde.equiv limits(plus.circle.big)_i R "/" (d_i) = R "/" (d_1) plus.circle R "/" (d_2) plus.circle dots.c plus.circle R "/" (d_n). $

And from machine learning research, here's a result from score identity distillation. After modifying the minimization objective to $cal(L)_5$, we find it's equivalent to minimizing the Fisher information between $p$ and $p_θ$:

$ cal(F)^+ (p, p_theta) &= integral p_("sg"[theta]) (x_t^((g))) ||nabla_(x_t^((g))) log p_theta (x_t^((g))) - nabla_    (x_t^((g))) log p (x_t^((g))) || ^2 dif x_t^((g)) \
 &= EE_(z, epsilon ~ N(0, I)) [ || nabla_(x_t^((g))) log p_theta ("sg"[x_t^((g))]) - nabla_(x_t^((g))) log p ("sg"[x_    t^((g))]) ||^2 ] \
 &prop EE_(z, epsilon ~ N(0, I) ) [ ||epsilon.alt_(phi^*) ("sg"[x_t^((g))], t) - epsilon.alt_(psi^*) ("sg"[x_t^((g)    )], t) ||^2 ] = cal(L)_5. $

== Conclusion
This post exists not just to share examples, but to be one—of how ssg handles equations, layout, themes, and metadata.

If you're ever unsure how to start something, start with an example. Then iterate. See what breaks. See what surprises you. That's the whole point of this post.
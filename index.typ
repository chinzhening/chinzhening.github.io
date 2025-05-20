#import "/typ/templates/index.typ": *

#show: base-template.with(description: "The landing page, includes about me.")

#div(class: "about-me")[
  #h1[About Me]
  Hello, I'm #span(id: "author")[Zhe Ning] #sym.dash.em a final-year mathematics undergraduate at
  #a(href: "https://en.wikipedia.org/wiki/National_University_of_Singapore","NUS").
  I also like building things: #sym.dash.em writing software,
  making music, or just brewing coffee. 
  This blog is a record of what I make, learn, and think about.

  You will find posts on:
  1. Creative projects and development notes
  2. Reflections on learning, time and work
  3. Mathematics, typesetting, and problem writeups

  Thanks for reading.
]

#post-list(posts-metadata)
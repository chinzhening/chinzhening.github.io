#import "/typ/templates/index.typ": *

#show: base-template.with(description: "The landing page, includes about me.")

#div(class: "about-me")[
  #h1[About Me]
  Hello, I'm #span(id: "author")[Zhe Ning] #sym.dash.em a third-year maths undergrad at
  #a(href: "https://en.wikipedia.org/wiki/National_University_of_Singapore","NUS"),
  originally from New Zealand. I like to create things#sym.dash.em writing software,
  making music, or brewing a cup of coffee. This blog is my way of capturing and
  sharing those moments.

  You can expect:
  1. Creative projects, dev notes
  2. Reflections
  3. Writeups, mathematics, typesetting
  4. And maybe a few surprises?

  Thanks for visiting and stay tuned for more!
]

#post-list(posts-metadata)
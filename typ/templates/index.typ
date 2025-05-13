#import "base.typ": *

#let post-list(posts) = {
  div(
    class: "post-list",
    posts,
  )
}

#let post-item(item) = {
  let (path, frontmatter) = item
  let (date, description, tags, title) = frontmatter

  // This is absurd bro.
  let href = post-link(path)

  div(
    class: "post-item",
    {
      a(href: href, title)

      div(
        class: "post-prop",
        {
          "published: " + date
          " "
          "tags: " + tags.join(", ")
        },
      )
      div(
        class: "post-desc",
        description,
      )
    },
  )
}
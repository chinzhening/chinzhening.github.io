#import "base.typ": *

#let post-item(item) = {
  let (path, frontmatter) = item
  let (date, description, tags, title) = frontmatter

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

#let post-list(posts-metadata) = {
  let posts-sorted = posts-metadata
    .sorted(key: (it) => it.frontmatter.date)
    .map(post-item)
    .rev()
    .join()
  div(
    class: "post-list",
    posts-sorted,
  )
}
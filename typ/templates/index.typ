#import "base.typ": *

#let post-item(item) = {
  let (path, frontmatter) = item
  let (date, description, tags, title) = frontmatter

  let href = post-link(path)

  div(
    class: "post-item",
    {
      a(
        class: "post-title",
        href: href,
        title)

      div(
        class: "post-prop",
        {
          div(
            class: "post-date",
            "published: " + date
            )
          div(
            class: "post-tag-container",
            "tags: " + tags.map(
              tg => {
                span(
                  class: "post-tag",
                  "#" + tg
                  )
              }
            ).join(", ")
          )
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
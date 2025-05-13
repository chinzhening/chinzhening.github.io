// Cli input to query front matter
#let is-front-matter = sys.inputs.at("front-matter", default: none) != none

#let post-template(
  date: none,
  title: none,
  tags: (),
  description: none,
  content,
) = {
  set document(title: title, description: description)
  // return frontmatter only
  if (is-front-matter) {
    return [
      #metadata(
        (
          title: title,
          date: date,
          description: description,
          tags: tags
        )
      ) <front-matter>
    ]
  }

  import "base.typ": *
  base-template(
    go-prev: post-link("/index.typ"),
    {
       div(
        class: "post-document",
        {
          div(
            class: "title-block",
            {
              div(class: "title", title)
              div(class: "desc", description)
              div(class: "date", "updated: " + date)
              div(class: "tags", "tags: " + tags.join(", "))
            }
          )
          content
        }
      )
    }
  )
}
#import "/typ/packages/utility.typ": *

// All metadata of post content.
#let post-list-metadata-filepath = sys.inputs.at("post-metadata-path", default: "/posts/meta/post-list.json")
#let post-data = json(bytes(read(post-list-metadata-filepath)))

// Converts source path to post link.
#let post-link(src) = {
  let href = src.replace(".typ", ".html")
  href
}

#let header = {
  /// Not Implemented
}

#let footer = {
  /// Not Implemented
}

#let navigation(go-prev: none, go-next: none) = {
  div(
    class: "nav-wrap left",
    {
      if go-prev != none {
        a(
          class: if go-prev != none {"nav"} else {"nav-hidden"},
          href: if go-prev != none {go-prev} else {""},
          div(id: "prev", class: "side", "")
        )
      }
    }
  )
  div(
    class: "nav-wrap right",
    {
      if go-next != none {
        a(
          class: if go-next != none {"nav"} else {"nav-hidden"},
          href: if go-next != none {go-next} else {""},
          div(id: "next", class: "side", "")
        )
      }
    }
  )
}

#let main-font = (
  "Libertinus Serif",
)

#let base-template(go-prev: none, go-next: none, description: none, content) = {
  set document(description: description) if description != none
  
  show: load-html-template.with(
    "/base.html",
  )

  /// TODO: inline math and display math block support
  show math.equation.where(block: true): p-frame.with(
    attrs: (class: "block-equation")
  )
  show math.equation.where(block: false): span-frame.with(
    attrs: (class: "inline-equation")
  )

  set text(font: main-font)

  header
  div(class: "main-content", content)
  navigation(go-prev: go-prev, go-next: go-next)
  footer
}
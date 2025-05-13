#import "supports-text.typ": *


/// Utility functions for HTML.
#let url-base = "/build/"
#let assets-url-base =  "/"

/// Converts loaded XML to HTML
#let to-html(data) = {
  if type(data) == str {
    let data = data.trim()
    if data.len() > 0 {
      data
    }
  } else {

    html.elem(
      data.tag,
      attrs: data.attrs,
      data.children.map(to-html).join(),
    )
  }
}

/// Creats a ```html <meta>``` tag for the ```html <head>```.
#let head-meta(name, content) = html.elem(
  "meta",
  attrs: (
    "name": name,
    "content": content,
  ),
)

// Loads HTML template and inserts content.
#let load-html-template(template-path, content, extra-head: none) = {
  let head-data = xml(template-path).at(0).children.at(1)
  let head = to-html(head-data).body



  html.elem(
    "html",
    {
      html.elem(
        "head",
        {
          head
          extra-head
          context if document.description != none {
            head-meta("description", plain-text(document.description))
          }
        },
      )
      html.elem("body", content) 
    },
  )
}


// Wrapper for html.elem
//
// - content (content): The content of the element.
// - tag (str): The tag of the element.
#let html-elem(content, ..attrs, tag: "div") = html.elem(
  tag,
  content,
  attrs: attrs.named(),
)

// Shorthands for creating HTML elements.
#let a = html-elem.with(tag: "a")
#let span = html-elem.with(tag: "span")
#let div = html-elem.with(tag: "div")
#let style = html-elem.with(tag: "style")
#let h1 = html-elem.with(tag: "h1")
#let h2 = html-elem.with(tag: "h2")

#let div-frame(content, attrs: (:), tag: "div") = html.elem(tag, html.frame(content), attrs: attrs)
#let span-frame = div-frame.with(tag: "span")
#let p-frame = div-frame.with(tag: "p")
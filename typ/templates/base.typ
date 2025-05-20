#import "/typ/packages/utility.typ": *

// All metadata of the posts.
#let posts-metadata-path = sys.inputs.at("posts-metadata-path", default: "/meta/posts.json")
#let fonts-metadata-path = sys.inputs.at("fonts-metadata-path", default: "/meta/fonts.json")
// All metadata of the fonts.
#let posts-metadata = json(bytes(read(posts-metadata-path)))
#let fonts-metadata = json(bytes(read(fonts-metadata-path)))

// Converts source path to post link.
#let post-link(src) = {
  let href = src.replace(".typ", ".html")
  href
}

#let font-item(item) = {
  span(
    class: "font-menu",
    a(
      class: "font-toggle" + if item.status == "default" {" active"} else {""},
      id: item.toggle_class,
      onmousedown: "event.stopPropagation()",
      onclick: "handleFontChange(this)",
      item.font_family
    )
  )
}

#let dark-mode-button = {
  span(
    id: "darkModeToggle",
    onclick: "toggleDarkMode()",
    "Toggle Dark Mode",
  )
}

#let font-script = {
  let default = fonts-metadata.at(0)
  script({
    ```js
    
    // Font Toggling
    window.addEventListener("load", event => {
      const default_font_class = "{{class}}";
      setBodyText(localStorage.getItem("activeFont") || default_font_class);
    })

    function setBodyText(toggle_class) {
      console.log(`setting body text to ${toggle_class}`);
      
      Array.from(document.body.classList)
      .filter(name => name.startsWith("font--"))
      .map(name => document.body.classList.remove(name));
      document.body.classList.add(toggle_class)

      Array.from(document.getElementsByClassName("font-toggle"))
      .map(el => el.classList.remove("active"));
      document.getElementById(toggle_class).classList.add("active");

      localStorage.setItem('activeFont', toggle_class)

    }

    function handleFontChange(evt) {
      setBodyText(evt.id);
    }
    ```
      .text
      .replace("{{class}}", default.toggle_class)
})
}

#let header = {
  div(
    class: "toolbar",
    {
      fonts-metadata
        .map(font-item)
        .join()
      dark-mode-button
    }
  )
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
          div(id: "prev", class: "side",
            span("↤")
          )
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
          div(id: "next", class: "side",
            span("↦")
          )
        )
      }
    }
  )
}

#let base-template(go-prev: none, go-next: none, description: none, content) = {
  set document(description: description) if description != none
  
  show: load-html-template.with(
    extra-head: {
      font-script
      fonts-metadata
        .map((item) => {
          let href = item.font_style_path.slice(1).replace("\\\\", "/")
          href
        })
        .map(preload-css)
        .join()
    },
    "/base.html",
  )

  /// TODO: inline math and display math block support
  show math.equation.where(block: true): p-frame.with(
    attrs: (class: "block-equation")
  )
  show math.equation.where(block: false): span-frame.with(
    attrs: (class: "inline-equation")
  )

  header
  div(class: "main-content", content)
  navigation(go-prev: go-prev, go-next: go-next)
  footer
  
}
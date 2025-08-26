use dioxus::prelude::*;
use crate::Markdown;
use crate::Route;

/// Sample markdown content for demonstration
fn get_blog_content(id: i32) -> String {
    match id {
        1 => r#"
# Welcome to Our Markdown Blog!

This is a **markdown** blog post that demonstrates the enhanced markdown component with syntax highlighting and image support.

## Features

- Syntax highlighted code blocks
- Image rendering with proper sizing
- External and internal links with different styling
- Tables and other markdown elements

### External Links

[Visit Dioxus Documentation](https://dioxuslabs.com) - This link opens in a new tab with a special indicator.

### Internal Links

[Go to Blog 2](#/blog/2) - This is an internal link that stays within the application.

### Code Example with Syntax Highlighting

```rust
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
}

fn main() {
    let user = User {
        name: String::from("John Doe"),
        email: String::from("john@example.com"),
        active: true,
    };
    
    println!("User: {:?}", user);
}
```

### Image Example

![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Syntax Highlighting | ✅ | Using syntect |
| Image Support | ✅ | With lazy loading |
| External Links | ✅ | With visual indicator |
| Tables | ✅ | With responsive design |

> This is a blockquote that can be used for important notes or quotes.
> 
> It supports multiple paragraphs and *formatting*.
        "#.to_string(),
        2 => r#"
# Advanced Markdown Features

This page demonstrates more advanced markdown features and styling.

## JavaScript Example

```javascript
// A simple React-like component in JavaScript
function Counter() {
  const [count, setCount] = useState(0);
  
  return (
    <div className="counter">
      <p>You clicked {count} times</p>
      <button onClick={() => setCount(count + 1)}>
        Click me
      </button>
    </div>
  );
}
```

## CSS Example

```css
.markdown-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
  line-height: 1.6;
}

.markdown-heading-1 {
  font-size: 2.2em;
  color: #333;
  border-bottom: 1px solid #eee;
}
```

## HTML Example

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Markdown Demo</title>
  <link rel="stylesheet" href="styles.css">
</head>
<body>
  <div id="app"></div>
  <script src="app.js"></script>
</body>
</html>
```

## Task Lists

- [x] Implement syntax highlighting
- [x] Add support for images
- [x] Style external links differently
- [ ] Add dark mode support
- [ ] Implement table of contents

## Nested Lists

1. First level
   - Second level
     - Third level
       - Fourth level
   - Back to second level
2. Back to first level

## Definition Lists

Term 1
: Definition 1

Term 2
: Definition 2a
: Definition 2b

## Footnotes

Here's a sentence with a footnote[^1].

[^1]: This is the footnote content.
        "#.to_string(),
        3 => r#"
# Image Gallery Example

This blog post demonstrates various image handling features.

## Responsive Images

![Nature Image 1](https://images.unsplash.com/photo-1501854140801-50d01698950b?ixlib=rb-1.2.1&auto=format&fit=crop&w=1050&q=80)

## Multiple Images

![Nature Image 2](https://images.unsplash.com/photo-1441974231531-c6227db76b6e?ixlib=rb-1.2.1&auto=format&fit=crop&w=1051&q=80)

![Nature Image 3](https://images.unsplash.com/photo-1470071459604-3b5ec3a7fe05?ixlib=rb-1.2.1&auto=format&fit=crop&w=1050&q=80)

## Local Images

These would work if you have local images in your assets folder:

![Local Image 1](/assets/images/sample1.jpg)
![Local Image 2](/assets/images/sample2.jpg)

## Image with Caption

![This is an image caption](https://images.unsplash.com/photo-1472214103451-9374bd1c798e?ixlib=rb-1.2.1&auto=format&fit=crop&w=1050&q=80)
        "#.to_string(),
        _ => format!(r#"
# Blog {id}

This is blog post number {id}.

## Dynamic Content

- This is a dynamically generated blog post
- You can navigate to other posts using the links below
- Try posts 1-3 for special content examples

```rust
// This is a code block in blog post {id}
fn get_blog_id() -> i32 {{
    {id}
}}
```

[Go to Blog 1](/blog/1)
        "#)
    }
}

#[component]
pub fn Blog(id: i32) -> Element {
    // Get the blog content for this ID
    let markdown_content = get_blog_content(id);
    
    // Define the base path for images (if you have local images)
    let image_base_path = "/assets/images";
    
    rsx! {
        document::Link { rel: "stylesheet", href: "/assets/blog.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/markdown.css"}
        document::Link { rel: "stylesheet", href: "/assets/styling/syntax.css"}

        div {
            id: "blog",
            class: "markdown-container",

            // Render the markdown content using our enhanced component
            Markdown {
                content: markdown_content,
                image_base_path: Some(image_base_path.to_string()),
                id: Some(format!("blog-content-{}", id))
            }

            // Navigation links
            div {
                class: "blog-navigation",
                Link {
                    to: Route::Blog { id: id - 1 },
                    class: if id <= 1 { "disabled-link" } else { "" },
                    "Previous"
                }
                span { " | " }
                Link {
                    to: Route::Home {},
                    "Home"
                }
                span { " | " }
                Link {
                    to: Route::Blog { id: id + 1 },
                    "Next"
                }
            }
        }
    }
}

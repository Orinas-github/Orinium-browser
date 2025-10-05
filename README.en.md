# Orinium Browser
**🚧 _This project is under development and does not yet function as a fully working browser._**

## A browser independent of Google
The source code of this browser engine **does not depend on Google**.  
Apart from some browsers like Firefox, most browsers in the world depend on Google's Chromium.  
This project provides a new browser engine as an alternative to Chromium.

## Custom Extension Formats
In the future, this browser engine will support extensions. The currently planned supported formats are:
* Orinium's custom format
* Firefox add-ons
* Chromium manifest v2 (partial)

Supporting these formats helps maintain compatibility with other browsers while providing unique features tailored to this browser for a better user experience.

---

## 🧪 Development Tests (Examples)
The file `examples/tests.rs` contains development tests that allow you to individually verify the main components of Orinium Browser.  
You can check GUI, network, and HTML parser functionality in an integrated manner.

### How to run
```bash
cargo run --example tests help
```

### Usage

| Command           | Description                                              |
| ----------------- | -------------------------------------------------------- |
| `help`            | Display the list of available commands                   |
| `create_window`   | Create and display a window                              |
| `fetch_url <URL>` | Fetch the specified URL and display response             |
| `parse_dom <URL>` | Fetch HTML from a URL and construct & print the DOM tree |

#### Examples

```bash
# Test window creation
cargo run --example tests create_window

# Test network fetch
cargo run --example tests fetch_url https://example.com

# Test DOM parsing
cargo run --example tests parse_dom https://example.com
```

This example allows you to easily verify asynchronous and GUI operations that are difficult to run in `#[test]`.

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

TODOs are listed in [tasks.md](./tasks.md).

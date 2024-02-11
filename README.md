
# 📦 Flexi Func 🚀

Welcome to **Flexi Func** - a Rust crate designed to supercharge your Rust programming experience with two powerful macros: `ff` (Flexi Func) and `fb` (Flexi Block) *or* (Function Builder) 🛠️.  

These macros are crafted to simplify and enhance the way you write synchronous and asynchronous code, making your Rust journey smoother and more efficient.

## 🎁 Features

- **`ff` (Flexi Func)**: Automatically generates synchronous and asynchronous versions of your functions with optional custom error handling 🌟.
- **`fb` (Flexi Block) *or* (Function Builder)**: Dynamically creates functions based on specified modes (`sync` or `async`), reducing the boilerplate for conditional function generation 🔄.

## 🚀 Getting Started

To start using **flexi_func** in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
flexi_func = "0.2.1"
```

Then, import the macros in your Rust file:

```rust
use flexi_func::{ff, fb};
```

## 💻 Usage

### 🛠 `ff` - Flexi Func

The `ff` macro simplifies the creation of synchronous and asynchronous function variants, including customizable error handling.

#### Basic Example

```rust
#[ff]
fn compute(data: Vec<u8>) -> Result<usize, MyError> {
    // Your synchronous code here
}
```

This generates an asynchronous version `compute_async` alongside the original `compute` function.

### 🐞 Custom Error Type

```rust
#[ff(error_type = "MyCustomError")]
fn process(data: Vec<u8>) -> Result<usize, MyCustomError> {
    // Your code here
}
```

### 🔄 `fb` - Flexi Block or Function Builder

Create synchronous or asynchronous functions on the fly with `fb`, tailored to reduce redundancy and improve code clarity.

#### ✅ Synchronous Function

```rust
fb!("sync", greet, (name: String), -> String, {
    format!("Hello, {}", name)
});
```

#### ⚡ Asynchronous Function

```rust
fb!("async", fetch_data, (url: String), -> Result<String, reqwest::Error>, {
    // Async fetch operation
});
```

## 💡 Advanced Tips

- Use conditional compilation with `fb` to adaptively generate sync or async functions based on your application's needs 🎛️.
- Combine `ff` with Rust's powerful error handling to streamline async error management 🚦.

## 🐳 Contributing

Contributions are welcome! If you'd like to help improve **flexi_func**, please feel free to create issues or submit pull requests 🤝.

## 📃 License

This project is licensed under [MIT](LICENSE.md).


# 📦 Flexi Func 🚀

Welcome to **Flexi Func** - a Rust crate designed to supercharge your Rust programming experience with two powerful macros: `ff` (Flexi Func) and `fb` (Flexi Block) *or* (Function Builder) 🛠️.  

These macros are crafted to simplify and enhance the way you write synchronous and asynchronous code, making your Rust journey smoother and more efficient.

## 🎁 Features

- **`ff` (Flexi Func)**: Mark your async function with this proc macro first, with optional custom error handling 🌟.
- **`fb!` (Flexi Block) *or* (Function Builder)**: Inside the sync function write down your (`sync` or `async`) versions using fb!  
- **`fb!`** Reducing the boilerplate for conditional function generation 🔄.

## 🚀 Getting Started

To start using **flexi_func** in your project, add it to your `Cargo.toml`:

```toml
[dependencies]
flexi_func = "0.2.7"
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
If you need to specify an async version of your code inside your sync function use the fb! declarative macro.

### 🐞 Custom Error Type

```rust
#[ff(error_type = "MyCustomError")]
fn process(data: Vec<u8>) -> Result<usize, MyCustomError> {
    // Your code here
}
```


### 🔄 `fb!` - Flexi Block or Function Builder

Enhance your Rust arsenal with `fb!`, a versatile macro designed to dynamically generate both synchronous and asynchronous functions or code blocks. This macro is engineered to minimize redundancy and elevate code clarity, offering a seamless way to craft adaptable code constructs.

#### ✅ Synchronous Function

Create a synchronous function with ease:

```rust
fb!(sync, greet, (name: String), -> String, {
    format!("Hello, {}", name)
});
```

#### ⚡ Asynchronous Function

Generate an asynchronous function for operations that require awaiting:

```rust
fb!(async, fetch_data, (url: String), -> Result<String, reqwest::Error>, {
    // Async fetch operation
});
```

#### 🔄 Returning a Closure

For scenarios where you need to capture the surrounding environment or defer execution:

- **Async Closure**

```rust
let async_closure = fb!(async, closure, {
    // Async code here
});
// Usage
async_closure().await;
```

- **Sync Closure**

```rust
let sync_closure = fb!(sync, closure, {
    // Sync code here
});
// Usage
sync_closure();
```

#### 🚀 Immediate Execution

Execute code blocks immediately, without the need to define a separate function:

- **Async Block**

```rust
let result = fb!(async, execute, {
    // Immediate async execution
});
// Await the block if necessary
result.await;
```

- **Sync Block**

```rust
fb!(sync, execute, {
    // Immediate sync execution
});
```

## 💡 Advanced Tips

- Leverage `fb!` for conditional compilation to dynamically generate sync or async functions, tailoring your code to the application's needs 🎛️.
- Enhance error management in async operations by combining `fb!` with Rust's robust error handling features 🚦.

## 🐳 Contributing

We welcome contributions to make `fb!` even better. If you're interested in enhancing its functionality or have suggestions, feel free to open issues or submit pull requests 🤝. Your input is invaluable in evolving this tool.

## 📃 License

This project is licensed under the [MIT License](LICENSE.md), fostering open collaboration and innovation.

<div align="center">

  <h1><code>Fast Fluid Dynamics</code></h1>

  <strong>Built with <a href="https://rustwasm.github.io/">Rust+WASM</a> and WebGL</strong>
</div>

## About

For our final project we implemented an algorithm called Fast Fluid Dynamics (FFD) using Rust and WebGL. This is a method for creating real-time, stable fluid simulations that run entirely on the GPU based on Jos Stam’s paper, “Stable Fluids” (Stam 1999).

Our implementation is based on the tutorial in [this](https://developer.download.nvidia.com/books/HTML/gpugems/gpugems_ch38.html) GPU Gems Chapter.
## 🚴 Usage

### 🐑 Install dependencies

In order to use Rust and WASM you will need the nightly version of Rust. 
* First, install [rustup](https://rustup.rs/)
* Then, run the following commands to install the rust nightly toolchain and make it the default:
```
$ rustup install nightly
$ rustup default nightly
```
You will also need to install [Node.js](https://nodejs.org/en/). Then run the following command to install the javascript dependencies. 
```
$ npm install
```

### 🛠️ Build 
To build and run the project run the following command.
```
$ npm run serve
```
You can access the simulation from `localhost:3000`

To build without running the webserver execute: 
```
$ npm run build
```



## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
 
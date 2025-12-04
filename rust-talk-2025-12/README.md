---
title: Exploring JavaScript with Rust
author: Kevin Ness
theme:
    name: catppuccin-mocha
    override:
        code:
            alignment: left
---

Who am I?
---

My name's Kevin Ness

- Open source maintainer and contributor 
  - Boa, a Rust JavaScript engine
  - temporal_rs, a Rust date/time library

<!-- end_slide -->

Exploring JavaScript in Rust
---

- Background on ECMAScript (AKA JavaScript) implementations
- Discuss Rust's footprint on ECMAScript implementations
- Walkthrough implementing a JavaScript runtime feature in Rust

<!-- end_slide -->

<!-- jump_to_middle -->
Background on ECMAScript, AKA JavaScript
===

<!-- end_slide -->

What is ECMAScript, AKA JavaScript
---

A language specified by tc39's specifications:

  - ECMA262, the core language specification
  - ECMA402, option i18n specification  

General conformance is measured by the `test262` conformance test suite.

There are also runtime features -- like `URL`, `console`, `fetch`, and
`setTimeout` -- but these are not technically part of the core language
specification.

<!-- end_slide -->


What is a "JavaScript engine"?
---

An interpreter that implements the ECMAScript specification, ECMA262.

Major browser / runtime implementations:

  - V8 (Chromium, NodeJS, Deno)
  - SpiderMonkey (FireFox),
  - JavaScriptCore (WebKit, Bun)
  - LibJS (LadyBird)*

Notably, all of these implementations are all large C++ projects.

(*) LibJS is not typically included in the "big three" of V8, SpiderMonkey,
and JavaScriptCore, but it is a formidable implementation itself that is
at the core of the new LadyBird.

<!-- end_slide -->

Why C++ and not Rust?
---

<!-- pause -->

- Most of the major engines are old projects (pre-2006), so Rust was not an option.
- Secondary tier implementations consisted mostly of Java (Rhino/GraalJS) or C (QuickJS)

<!-- pause -->

Can Rust's memory safety guarantees be brought to the world of ECMAScript?
===

<!-- pause -->

- Room from greenfield projects to reapproach ECMAScript implementations
in order to bring.

<!-- pause -->

- Boa began as one such project to write a new ECMAScript implementation in Rust.

<!-- end_slide -->


<!-- jump_to_middle -->
Rust + ECMAScript = ?
===

<!-- end_slide -->

Rust ECMAScript ecosystem overview
---

ECMAScript implementations:

  - Boa
  - Brimstone
  - Nova
  - YavaShark

<!-- pause -->

ECMAScript related projects:

  - temporal_rs, date/time library for ECMAScript's Temporal
  - regress, Regex with EcmaScript syntax
  - ryu-js, ECMA compliant fork of ryu
  - ICU4X, Unicode internationalization libraries 

<!-- pause -->

JavaScript tooling:

  - biome (linting/formatting)
  - oxc (linting/formatting/minifying/etc.)
  - swc (JS/TS compiler)
  - rspack (bundler)
  - rolldown (bundler)
  - And probably much, much more.

<!-- end_slide -->

General Rust adoption?
---

Yes, some! There has been growing adoption of Rust!

<!-- pause -->

- V8 uses `temporal_rs` and ICU4X for it's Temporal implementation
- SpiderMonkey uses ICU4X for some `Intl` components

<!-- pause -->

`temporal_rs` was the first introduction of Rust into V8.

<!-- end_slide -->

<!-- jump_to_middle -->
Boa
===

<!-- end_slide -->

About Boa
---

- Started in 2018 by Jason Williams
- Aims to be a fully conformant and performant Rust engine with minimal unsafe Rust.
    - Unsafe Rust should ideally be auditable and pass test suite with MIRI


<!-- end_slide -->

Current progress and state
---

Boa has a high conformance!

- Currently sits at about a 93% test262 conformance

Performance still needs to be improved.

- Regional / pool allocator
- Garbage collector rewrite  
- String ropes
- And more!

<!-- end_slide -->

Bridging the gap between Rust and JavaScript
---

- Boa offers interoperability macros for defining JavaScript modules in Rust.
- Used for defining runtime features in Rust.

<!-- end_slide -->

<!-- jump_to_middle -->
Let's take a deeper look
===

<!-- alignment: center -->
Implementing a mini JavaScript URL class

<!-- end_slide -->

JavaScript's URL class
---

Let's walkthrough putting together a URL class for JavaScript for Boa.

This feature is part of the Web API defined by WHATWG, not the core
ECMAScript specification.

```js
// N.B. example sourced from MDN.
const url = new URL("../cats", "http://www.example.com/dogs");
console.log(url.hostname); // "www.example.com"
console.log(url.pathname); // "/cats"
```

<!-- end_slide -->

<!-- jump_to_middle -->
Let's begin to implement our URL class
===

<!-- alignment: center -->
Defining the URL struct itself


<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
struct Url(url::Url);
```

<!-- column: 1 -->

Let's begin with a basic `Url` struct.

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsData, Trace, Finalize};

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);
```

<!-- pause -->

<!-- column: 1 -->

- **Debug**: Provides Rust debug printing
- **Clone**: Allows object to be cloned
- **JsData**: Marks a struct that can be stored in a `JsObject`
- **Trace**: Marks the object as GC traceable + implements methods
- **Finalize**: Marks the struct as GC finalizeable + implements methods

<!-- pause -->

Also, worth noting the attributes added to the struct as well.

<!-- end_slide -->

<!-- jump_to_middle -->
Adding a constructor
===

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsData, Trace, Finalize};

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

impl Url {
    fn new() -> Self {
        todo!()
    }
}
```

<!-- pause -->

<!-- column: 1 -->

But how does the Rust `Url` struct map into JavaScript's `URL`?

Also, how does Boa know what method to use for the constructor?

<!-- pause -->

Answer: procedural macro attributes!

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsData, Trace, Finalize};
use boa_engine::boa_class;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new() -> Self {
        todo!()
    }
}
```

<!-- pause -->

<!-- column: 1 -->

We are able to remap the `Url` struct to `URL`!

We can also map any methods in the impl block from Rust's snake case to JavaScript's
camel case.

We can also declare a specific method as the constructor for that type.

<!-- end_slide -->

But what actually is the URL class?
---

The URL class is defined by WHATWG in Web IDL as:

```idl
interface URL {
  constructor(USVString url, optional USVString base);

  // ... property + methods definitions
};
```

From this Web IDL overview, we know that our constructor takes one arg `url` and one
optional arg `base`.

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsData, Trace, Finalize};
use boa_engine::boa_class;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new(url: String, base: Option<String>) -> Self {
        todo!()
    }
}
```

<!-- pause -->

<!-- column: 1 -->

But there's one problem
===

<!-- pause -->

JavaScript strings are UTF16; meanwhile, Rust strings are UTF8.

Luckily, this conversion is already covered by Boa.

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsData, Trace, Finalize};
use boa_engine::boa_class;
use boa_engine::value::Convert;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new(Convert(ref url): Convert<String>, base: Option<Convert<String>>) -> Self {
        todo!()
    }
}
```

<!-- column: 1 -->

But there's one problem
===

JavaScript strings are UTF16; meanwhile, Rust strings are UTF8.

Luckily, this conversion is already covered by Boa.

Now we're ready to move forward.

<!-- end_slide -->

URL Class Example
---

<!-- column_layout: [2, 1] -->

<!-- column: 0 -->

```rust +line_numbers
use boa_engine::{JsString, JsData, Trace, Finalize};
use boa_engine::boa_class;
use boa_engine::value::Convert;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new(Convert(ref url): Convert<String>, base: Option<Convert<String>>) -> Self {
        todo!()
    }

    #[boa(getter)]
    fn host(&self) -> JsString {
        JsString::from(url::quirks::host(&self.0))
    }

    #[boa(setter)]
    #[boa(rename = "host")]
    fn set_host(&mut self, value: Convert<String>) {
        let _ = url::quirks::set_host(&mut self.0, &value.0);
    }
}
```

<!-- column: 1 -->

We can also easily define getters and setters on the class itself.

<!-- end_slide -->

<!-- jump_to_middle -->
Exposing `Url` to JavaScript code
===

<!-- end_slide -->

Creating a Boa module
---

<!-- column_layout: [1, 2] -->

<!-- column: 0 -->

For `Url` to be usable, the class needs to be registered in Boa's context.

This can be achieved again with the power of macros.

<!-- pause -->

<!-- column: 1 -->

```rust +line_numbers
// Declare a Boa module
//
// #[boa_module] automatically implements a `boa_register` method for `js_module`
#[boa_module]
pub mod js_module {
  type Url = super::Url;
}
```

<!-- end_slide -->

<!-- jump_to_middle -->
Let's put everything together
===

<!-- end_slide -->

In `url.rs`:

```rust +line_numbers
use boa_engine::{Context, realm::Realm, JsResult, JsString, JsData, Trace, Finalize};
use boa_engine::{boa_class, boa_module, js_error};
use boa_engine::value::Convert;

#[derive(Debug, Clone, JsData, Trace, Finalize)]
#[boa_gc(unsafe_no_drop)]
struct Url(#[unsafe_ignore_trace]url::Url);

impl Url {
    pub fn register(realm: Option<Realm>, context: &mut Context) -> JsResult<()> {
        js_module::boa_register(realm, context)
    }
}

#[boa_class(rename = "URL")]
#[boa(rename_all = "camelCase")]
impl Url {
    #[boa(constructor)]
    fn new(Convert(ref url): Convert<String>, base: Option<Convert<String>>) -> JsResult<Self> {
        // implementation code
        unimplemented!()
    }

    #[boa(getter)]
    fn host(&self) -> JsString {
        JsString::from(url::quirks::host(&self.0))
    }

    #[boa(setter)]
    #[boa(rename = "host")]
    fn set_host(&mut self, value: Convert<String>) {
        let _ = url::quirks::set_host(&mut self.0, &value.0);
    }
}

#[boa_module]
pub mod js_module {
  type Url = super::Url;
}
```

<!-- end_slide -->

In `main.rs`:

```rust +line_numbers
use boa_engine::{Context, Source};

pub mod url;

fn main() {
    let mut context = Context::default();
    url::Url::register(None, &mut context).expect("successful registration");
    
    // We now have JavaScript context that can
    // use the `URL` class when evaluating JavaScript.
    context.eval(Source::from_bytes("console.log('Hello world!')".as_bytes()))
}
```

<!-- end_slide -->

<!-- jump_to_middle -->
Let's run the example!
---

<!-- end_slide -->

Boa runtime features
---

Good news!

Boa's runtime crate `boa_runtime` already implements `Url` (although,
`URLSearchParams` still needs to be implemented).

`boa_runtime` currently implements:

  - console
  - fetch
  - URL
  - setTimeout
  - microTask
  - postMessage
  - structuredClone

```rust +line_numbers
use boa_runtime::extensions::ConsoleExtension;
fn main() {
    let mut context = Context::default();
    boa_runtime::register(
        (ConsoleExtension::default(),),
        None,
        context
    ).expect("registering runtime features should not fail")
}
```

<!-- end_slide -->

<!-- jump_to_middle -->
What did we learn?
---

<!-- end_slide -->

ECMAScript implementations
---

<!-- pause -->
- the ECMA262 and ECMA402 specifications
<!-- pause -->
- briefly overviewed current ECMAScript engines

<!-- pause -->

Rust + ECMAScript
---

<!-- pause -->
- the state of Rust in the ECMAScript implementation ecosystem
<!-- pause -->
- Boa and its current state
<!-- pause -->
- walkthroughed implementing a JavaScript runtime feature in Rust with Boa

<!-- end_slide -->

<!-- jump_to_middle -->
Interested in learning more or contributing?
---

<!-- alignment: center -->
Feel free to reach out!

<!-- end_slide -->

<!-- jump_to_middle -->
Any questions?
---


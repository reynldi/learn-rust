Rust brings contemporary developer tools to the systems programming world:

- Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
- Rustfmt ensures a consistent coding style across developers.
- The Rust Language Server powers Integrated Development Environment (IDE) integration for code completion and inline error messages.

## Notes

- Rust file naming convention is **file_name.rs**
- You can find the solution if you got error from compiler by executing `rustc --explain ERROR_CODE` for example `rustc --explain E0765`
- We can use cargo to organize our code. with cargo, we got so many advantages such as managing dependency, building.etc
- Create new project with cargo `cargo new project_name`
- Rust project configuration is using TOML and all main source code will be stored inside `src` directory
- To build cargo project, execute `cargo build` it will generate a `target` directory. You can execute your program that you just build inside `target/debug`
- After building, cargo will generate a new file called `cargo.lock` this act same like `package-json.lock` the purpose is to lock and track fixed dependency version
- Cargo figured out that the files hadn’t changed, so it just ran the binary. If you had modified your source code, Cargo would have rebuilt the project before running it.
- Cargo also provides a command called `cargo check`This command quickly checks your code to make sure it compiles but doesn’t produce an executable.
- To build production-ready application, you can execute `cargo build --release` and the executable app will be store in `target/release` instead `target/debug`
- To generate all function and dependency documentation locally, we can use `cargo doc --open`

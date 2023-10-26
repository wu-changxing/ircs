## feadback from professor Zed

Incredible style in this assignent, great work!
It's clear that a lot of time went into the assignment as it's extremely polished.
File structure is logical and easy to navigate, rust features like traits,
enums and structs have been used throughout in a thoughtful way. Code is
consistently commented and doccumented professionally.

The implementation of multi-platform dynamic library loading is extroadinarily impressive!
Similarly, effective use of tokio for concurrency is incredibe.

Great work!

ID Mark: 50/50

# Final Marks

| Mark Name                                                    | Mark                           |
| ------------------------------------------------------------ | ------------------------------ |
| mechanical_style                                             | 10.0/10                        |
| functional_correctness                                       | 40.0/40                        |
| idiomatic_design                                             | 50.0/50                        |
| ------------------------------------------------------------ | ------------------------------ |
| Mark Before Adjustment                                       | 100.00/100                     |

## code comments

````
     1  ====================================================
     2  =                  Code Comments                   =
     3  ====================================================
     4 diff --git a/iris/iris/iris_lib/connect.rs b/iris/iris/iris_lib/connect.rs
     5 index eac693a..7d09b86 100644
     6 --- a/iris/iris/iris_lib/connect.rs
     7 +++ b/iris/iris/iris_lib/connect.rs
     8 @@ -67,7 +67,6 @@ impl ConnectionManager {
     9          }
    10      }
    11
    12 -    //// Oh wow, even re-wrote the connection manager. Well done!
    13      pub async fn accept_new_connection(&mut self) -> (ConnectionRead, ConnectionWrite) {
    14          loop {
    15              match self.listener.accept().await {
    16 diff --git a/iris/iris/iris_lib/handler.rs b/iris/iris/iris_lib/handler.rs
    17 index d5b520d..708ecce 100644
    18 --- a/iris/iris/iris_lib/handler.rs
    19 +++ b/iris/iris/iris_lib/handler.rs
    20 @@ -171,7 +171,6 @@ pub async fn handle_privmsg(
    21              }
    22          }
    23          Target::Channel(target_channel) => {
    24 -            //// i really hope this isn't still todo :)
    25              // TODO: Handle channel messages
    26              let mut channels_locked = channels.lock().await;
    27              if let Some(channel_members) = channels_locked.get_mut(&target_channel) {
    28 diff --git a/iris/iris/iris_lib/plugins/plugin.rs b/iris/iris/iris_lib/plugins/plugin.rs
    29 index d757e4c..b532457 100644
    30 --- a/iris/iris/iris_lib/plugins/plugin.rs
    31 +++ b/iris/iris/iris_lib/plugins/plugin.rs
    32 @@ -24,7 +24,6 @@ struct PluginConfig {
    33  pub type ProcessMessageFn =
    34      fn(&mut dyn Plugin, &Nick, &str) -> (Option<String>, Option<Nick>, Option<String>, u32);
    35
    36 -//// This trait is excellent!
    37  pub trait Plugin: Send + Sync {
    38      fn name(&self) -> &'static str;
    39      /// Determines if the given message is applicable for processing by the plugin.
    40 @@ -41,7 +40,6 @@ pub trait Plugin: Send + Sync {
    41      fn process_message_fn(&self) -> ProcessMessageFn;
    42  }
    43
    44 -//// The implementation of this plugin manager is very good
    45  pub struct PluginManager {
    46      plugins: Vec<Box<dyn Plugin>>,
    47  }
    48 @@ -100,8 +98,6 @@ impl PluginManager {
    49      /// * `plugin_config` - A reference to the `PluginConfig` containing the plugin's configuration.
    50      fn load_plugin(&mut self, plugin_config: &PluginConfig) {
    51          let library_path = Path::new(&plugin_config.path);
    52 -        //// WOW!~
    53 -        //// This is incredibly impressive
    54          match unsafe { Library::new(library_path) } {
    55              Ok(library) => {
    56                  let constructor: Symbol<unsafe extern "C" fn() -> *mut PluginWrapper> =
    57 @@ -163,8 +159,6 @@ impl PluginManager {
    58                  let (response, target_nick, error, delay) =
    59                      process_message(plugin.as_mut(), sender_nick, message);
    60                  if let Some(response) = response {
    61 -                    //// Be careful with excess indenting.
    62 -                    //// This could be a done as a guard clause to prevent that
    63                      if let Some(target_nick) = target_nick {
    64                          if delay > 0 {
    65                              tokio::spawn(async move {
    66 diff --git a/iris/iris/src/concurrency.rs b/iris/iris/src/concurrency.rs
    67 index 57fd05f..5fbe55d 100644
    68 --- a/iris/iris/src/concurrency.rs
    69 +++ b/iris/iris/src/concurrency.rs
    70 @@ -41,8 +41,6 @@ use tokio::sync::Mutex;
    71  /// );
    72  /// ```
    73
    74 -//// This function could probably be split into two
    75 -//// However, it does sufficient delegation of handling messages
    76  pub fn handle_client_connection(
    77      mut conn_read: ConnectionRead,
    78      mut conn_write: ConnectionWrite,
    79 diff --git a/iris/iris/src/main.rs b/iris/iris/src/main.rs
    80 index 050dcfd..20a4a21 100644
    81 --- a/iris/iris/src/main.rs
    82 +++ b/iris/iris/src/main.rs
    83 @@ -27,7 +27,6 @@ struct Arguments {
    84      config_path: String,
    85  }
    86
    87 -//// Nice, incredibly clean main file
    88  #[tokio::main(flavor = "multi_thread", worker_threads = 4)]
    89  async fn main() {
    90      let arguments = Arguments::parse();
    91
````

# Iris Project

Welcome to the Iris Project, a comprehensive suite designed to facilitate efficient and robust software development. This project encompasses various components, each tailored to specific functionalities within the software development lifecycle.

## Overview

Iris is structured into multiple crates, each serving a distinct purpose within the broader ecosystem of the project. The primary components include:

- **Iris Core**: The central module of the project, responsible for the primary operations and functionalities.
- **Iris Lib**: A library module providing essential utilities and functions used across the project.
- **Plugin System**: A flexible plugin architecture allowing for the integration of various functionalities like calculators, reminders, and more.

## Build Logs

The build process of Iris is meticulously logged to ensure transparency and ease of debugging. The logs include details of the build process, testing, linting (clippy), and formatting (fmt). Here are some highlights from the recent build logs:

- **Build Success**: All modules, including `iris`, `iris_lib`, and various plugins, have successfully compiled.
- **Testing**: Unit tests have been executed with positive outcomes, ensuring the reliability of the code.
- **Linting and Formatting**: Code quality is maintained with `clippy` and `fmt`, ensuring adherence to Rust's standard coding practices.

### Key Build Log Excerpts

- Iris Core Build: Completed in 1.90s
- Iris Lib Test: 5 tests passed, completed in 1.48s
- Plugin - Calculator: Compiled successfully in 9.99s
- Plugin - Reminder: Compiled successfully in 0.67s

## Getting Started

To get started with Iris, clone the repository and follow the setup instructions detailed in the documentation. Ensure you have Rust and Cargo installed on your system for a seamless setup experience.

```bash
cd iris
cargo build
```

## Contributing

Contributions to Iris are welcome! Whether it's bug reports, feature requests, or code contributions, your input is valuable. Please refer to our contributing guidelines for more information on how to get involved.

## License

Iris is licensed under [MIT License](LICENSE). Feel free to use, modify, and distribute the code as per the license terms.

## Acknowledgements

Special thanks to all contributors and users of Iris. Your support and feedback have been invaluable in shaping this project.

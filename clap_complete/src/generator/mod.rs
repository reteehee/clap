//! Shell completion machinery

pub mod utils;

use std::ffi::OsString;
use std::fs::File;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;

use clap::App;

/// Generator trait which can be used to write generators
pub trait Generator {
    /// Returns the file name that is created when this generator is called during compile time.
    ///
    /// # Panics
    ///
    /// May panic when called outside of the context of [`generate`] or [`generate_to`]
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io::Write;
    /// # use clap::App;
    /// use clap_complete::Generator;
    ///
    /// pub struct Fish;
    ///
    /// impl Generator for Fish {
    /// #   fn generate(&self, app: &App, buf: &mut dyn Write) {}
    ///     fn file_name(&self, name: &str) -> String {
    ///         format!("{}.fish", name)
    ///     }
    /// }
    /// ```
    fn file_name(&self, name: &str) -> String;

    /// Generates output out of [`clap::App`](App).
    ///
    /// # Panics
    ///
    /// May panic when called outside of the context of [`generate`] or [`generate_to`]
    ///
    /// # Examples
    ///
    /// The following example generator displays the [`clap::App`](App)
    /// as if it is printed using [`std::println`].
    ///
    /// ```
    /// use std::{io::Write, fmt::write};
    /// use clap::App;
    /// use clap_complete::Generator;
    ///
    /// pub struct ClapDebug;
    ///
    /// impl Generator for ClapDebug {
    ///     fn generate(&self, app: &App, buf: &mut dyn Write) {
    ///         write!(buf, "{}", app).unwrap();
    ///     }
    /// #   fn file_name(&self, name: &str) -> String {
    /// #    name.into()
    /// #   }
    /// }
    /// ```
    fn generate(&self, app: &App, buf: &mut dyn Write);
}

/// Generate a completions file for a specified shell at compile-time.
///
/// **NOTE:** to generate the file at compile time you must use a `build.rs` "Build Script" or a
/// [`cargo-xtask`](https://github.com/matklad/cargo-xtask)
///
/// # Examples
///
/// The following example generates a bash completion script via a `build.rs` script. In this
/// simple example, we'll demo a very small application with only a single subcommand and two
/// args. Real applications could be many multiple levels deep in subcommands, and have tens or
/// potentially hundreds of arguments.
///
/// First, it helps if we separate out our `App` definition into a separate file. Whether you
/// do this as a function, or bare App definition is a matter of personal preference.
///
/// ```
/// // src/cli.rs
///
/// use clap::{App, Arg};
///
/// pub fn build_cli() -> App<'static> {
///     App::new("compl")
///         .about("Tests completions")
///         .arg(Arg::new("file")
///             .help("some input file"))
///         .subcommand(App::new("test")
///             .about("tests things")
///             .arg(Arg::new("case")
///                 .long("case")
///                 .takes_value(true)
///                 .help("the case to test")))
/// }
/// ```
///
/// In our regular code, we can simply call this `build_cli()` function, then call
/// `get_matches()`, or any of the other normal methods directly after. For example:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
///
/// fn main() {
///     let _m = cli::build_cli().get_matches();
///
///     // normal logic continues...
/// }
/// ```
///
/// Next, we set up our `Cargo.toml` to use a `build.rs` build script.
///
/// ```toml
/// # Cargo.toml
/// build = "build.rs"
///
/// [dependencies]
/// clap = "*"
///
/// [build-dependencies]
/// clap = "*"
/// clap_complete = "*"
/// ```
///
/// Next, we place a `build.rs` in our project root.
///
/// ```ignore
/// use clap_complete::{generate_to, shells::Bash};
/// use std::env;
/// use std::io::Error;
///
/// include!("src/cli.rs");
///
/// fn main() -> Result<(), Error> {
///     let outdir = match env::var_os("OUT_DIR") {
///         None => return Ok(()),
///         Some(outdir) => outdir,
///     };
///
///     let mut app = build_cli();
///     let path = generate_to(
///         Bash,
///         &mut app, // We need to specify what generator to use
///         "myapp",  // We need to specify the bin name manually
///         outdir,   // We need to specify where to write to
///     )?;
///
///     println!("cargo:warning=completion file is generated: {:?}", path);
///
///     Ok(())
/// }
/// ```
///
/// Now, once we compile there will be a `{bin_name}.bash` file in the directory.
/// Assuming we compiled with debug mode, it would be somewhere similar to
/// `<project>/target/debug/build/myapp-<hash>/out/myapp.bash`.
///
/// **NOTE:** Please look at the individual [shells][crate::shells]
/// to see the name of the files generated.
pub fn generate_to<G, S, T>(
    gen: G,
    app: &mut clap::App,
    bin_name: S,
    out_dir: T,
) -> Result<PathBuf, Error>
where
    G: Generator,
    S: Into<String>,
    T: Into<OsString>,
{
    app.set_bin_name(bin_name);

    let out_dir = PathBuf::from(out_dir.into());
    let file_name = gen.file_name(app.get_bin_name().unwrap());

    let path = out_dir.join(file_name);
    let mut file = File::create(&path)?;

    _generate::<G, S>(gen, app, &mut file);
    Ok(path)
}

/// Generate a completions file for a specified shell at runtime.
///
/// Until `cargo install` can install extra files like a completion script, this may be
/// used e.g. in a command that outputs the contents of the completion script, to be
/// redirected into a file by the user.
///
/// # Examples
///
/// Assuming a separate `cli.rs` like the [example above](generate_to()),
/// we can let users generate a completion script using a command:
///
/// ```ignore
/// // src/main.rs
///
/// mod cli;
/// use std::io;
/// use clap_complete::{generate, shells::Bash};
///
/// fn main() {
///     let matches = cli::build_cli().get_matches();
///
///     if matches.is_present("generate-bash-completions") {
///         generate(Bash, &mut cli::build_cli(), "myapp", &mut io::stdout());
///     }
///
///     // normal logic continues...
/// }
///
/// ```
///
/// Usage:
///
/// ```shell
/// $ myapp generate-bash-completions > /usr/share/bash-completion/completions/myapp.bash
/// ```
pub fn generate<G, S>(gen: G, app: &mut clap::App, bin_name: S, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    app.set_bin_name(bin_name);
    _generate::<G, S>(gen, app, buf)
}

fn _generate<G, S>(gen: G, app: &mut clap::App, buf: &mut dyn Write)
where
    G: Generator,
    S: Into<String>,
{
    app._build_all();

    gen.generate(app, buf)
}

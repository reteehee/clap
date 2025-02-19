/// Command line argument parser kind of error
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Occurs when an [`Arg`][crate::Arg] has a set of possible values,
    /// and the user provides a value which isn't in that set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("speed")
    ///         .possible_value("fast")
    ///         .possible_value("slow"))
    ///     .try_get_matches_from(vec!["prog", "other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidValue);
    /// ```
    InvalidValue,

    /// Occurs when a user provides a flag, option, argument or subcommand which isn't defined.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(arg!(--flag "some flag"))
    ///     .try_get_matches_from(vec!["prog", "--other"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::UnknownArgument);
    /// ```
    UnknownArgument,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which meets the threshold for
    /// being similar enough to an existing subcommand.
    /// If it doesn't meet the threshold, or the 'suggestions' feature is disabled,
    /// the more general [`UnknownArgument`] error is returned.
    ///
    /// # Examples
    ///
    #[cfg_attr(not(feature = "suggestions"), doc = " ```no_run")]
    #[cfg_attr(feature = "suggestions", doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "confi"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    InvalidSubcommand,

    /// Occurs when the user provides an unrecognized [`Subcommand`] which either
    /// doesn't meet the threshold for being similar enough to an existing subcommand,
    /// or the 'suggestions' feature is disabled.
    /// Otherwise the more detailed [`InvalidSubcommand`] error is returned.
    ///
    /// This error typically happens when passing additional subcommand names to the `help`
    /// subcommand. Otherwise, the more general [`UnknownArgument`] error is used.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind, };
    /// let result = App::new("prog")
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .help("The configuration file to use")
    ///             .index(1)))
    ///     .try_get_matches_from(vec!["prog", "help", "nothing"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::UnrecognizedSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`InvalidSubcommand`]: ErrorKind::InvalidSubcommand
    /// [`UnknownArgument`]: ErrorKind::UnknownArgument
    UnrecognizedSubcommand,

    /// Occurs when the user provides an empty value for an option that does not allow empty
    /// values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .takes_value(true)
    ///          .forbid_empty_values(true)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color="]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind(), ErrorKind::EmptyValue);
    /// ```
    EmptyValue,

    /// Occurs when the user doesn't use equals for an option that requires equal
    /// sign to provide values.
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let res = App::new("prog")
    ///     .arg(Arg::new("color")
    ///          .takes_value(true)
    ///          .require_equals(true)
    ///          .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--color", "red"]);
    /// assert!(res.is_err());
    /// assert_eq!(res.unwrap_err().kind(), ErrorKind::NoEquals);
    /// ```
    NoEquals,

    /// Occurs when the user provides a value for an argument with a custom validation and the
    /// value fails that validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// fn is_numeric(val: &str) -> Result<(), String> {
    ///     match val.parse::<i64>() {
    ///         Ok(..) => Ok(()),
    ///         Err(..) => Err(String::from("Value wasn't a number!")),
    ///     }
    /// }
    ///
    /// let result = App::new("prog")
    ///     .arg(Arg::new("num")
    ///          .validator(is_numeric))
    ///     .try_get_matches_from(vec!["prog", "NotANumber"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::ValueValidation);
    /// ```
    ValueValidation,

    /// Occurs when a user provides more values for an argument than were defined by setting
    /// [`Arg::max_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("arg")
    ///         .max_values(2))
    ///     .try_get_matches_from(vec!["prog", "too", "many", "values"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::TooManyValues);
    /// ```
    /// [`Arg::max_values`]: crate::Arg::max_values()
    TooManyValues,

    /// Occurs when the user provides fewer values for an argument than were defined by setting
    /// [`Arg::min_values`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .min_values(3))
    ///     .try_get_matches_from(vec!["prog", "--opt", "too", "few"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::TooFewValues);
    /// ```
    /// [`Arg::min_values`]: crate::Arg::min_values()
    TooFewValues,

    /// Occurs when a user provides more occurrences for an argument than were defined by setting
    /// [`Arg::max_occurrences`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("verbosity")
    ///         .short('v')
    ///         .max_occurrences(2))
    ///     .try_get_matches_from(vec!["prog", "-vvv"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::TooManyOccurrences);
    /// ```
    /// [`Arg::max_occurrences`]: crate::Arg::max_occurrences()
    TooManyOccurrences,

    /// Occurs when the user provides a different number of values for an argument than what's
    /// been defined by setting [`Arg::number_of_values`] or than was implicitly set by
    /// [`Arg::value_names`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("some_opt")
    ///         .long("opt")
    ///         .takes_value(true)
    ///         .number_of_values(2))
    ///     .try_get_matches_from(vec!["prog", "--opt", "wrong"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::WrongNumberOfValues);
    /// ```
    ///
    /// [`Arg::number_of_values`]: crate::Arg::number_of_values()
    /// [`Arg::value_names`]: crate::Arg::value_names()
    WrongNumberOfValues,

    /// Occurs when the user provides two values which conflict with each other and can't be used
    /// together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .conflicts_with("color"))
    ///     .arg(Arg::new("color")
    ///         .long("color"))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--color"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
    /// ```
    ArgumentConflict,

    /// Occurs when the user does not provide one or more required arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .required(true))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::MissingRequiredArgument);
    /// ```
    MissingRequiredArgument,

    /// Occurs when a subcommand is required (as defined by [`AppSettings::SubcommandRequired`]),
    /// but the user does not provide one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, AppSettings, ErrorKind};
    /// let err = App::new("prog")
    ///     .setting(AppSettings::SubcommandRequired)
    ///     .subcommand(App::new("test"))
    ///     .try_get_matches_from(vec![
    ///         "myprog",
    ///     ]);
    /// assert!(err.is_err());
    /// assert_eq!(err.unwrap_err().kind(), ErrorKind::MissingSubcommand);
    /// # ;
    /// ```
    ///
    /// [`AppSettings::SubcommandRequired`]: crate::AppSettings::SubcommandRequired
    MissingSubcommand,

    /// Occurs when the user provides multiple values to an argument which doesn't allow that.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .arg(Arg::new("debug")
    ///         .long("debug")
    ///         .multiple_occurrences(false))
    ///     .try_get_matches_from(vec!["prog", "--debug", "--debug"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::UnexpectedMultipleUsage);
    /// ```
    UnexpectedMultipleUsage,

    /// Occurs when the user provides a value containing invalid UTF-8.
    ///
    /// To allow arbitrary data
    /// - Set [`Arg::allow_invalid_utf8`] for argument values
    /// - Set [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`] for external-subcommand
    ///   values
    ///
    /// # Platform Specific
    ///
    /// Non-Windows platforms only (such as Linux, Unix, OSX, etc.)
    ///
    /// # Examples
    ///
    #[cfg_attr(not(unix), doc = " ```ignore")]
    #[cfg_attr(unix, doc = " ```")]
    /// # use clap::{App, Arg, ErrorKind, AppSettings};
    /// # use std::os::unix::ffi::OsStringExt;
    /// # use std::ffi::OsString;
    /// let result = App::new("prog")
    ///     .arg(Arg::new("utf8")
    ///         .short('u')
    ///         .takes_value(true))
    ///     .try_get_matches_from(vec![OsString::from("myprog"),
    ///                                 OsString::from("-u"),
    ///                                 OsString::from_vec(vec![0xE9])]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidUtf8);
    /// ```
    ///
    /// [`Arg::allow_invalid_utf8`]: crate::Arg::allow_invalid_utf8
    /// [`AppSettings::AllowInvalidUtf8ForExternalSubcommands`]: crate::AppSettings::AllowInvalidUtf8ForExternalSubcommands
    InvalidUtf8,

    /// Not a true "error" as it means `--help` or similar was used.
    /// The help message will be sent to `stdout`.
    ///
    /// **Note**: If the help is displayed due to an error (such as missing subcommands) it will
    /// be sent to `stderr` instead of `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .try_get_matches_from(vec!["prog", "--help"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayHelp);
    /// ```
    DisplayHelp,

    /// Occurs when either an argument or a [`Subcommand`] is required, as defined by
    /// [`AppSettings::ArgRequiredElseHelp`] and
    /// [`AppSettings::SubcommandRequiredElseHelp`], but the user did not provide
    /// one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, AppSettings, ErrorKind, };
    /// let result = App::new("prog")
    ///     .setting(AppSettings::ArgRequiredElseHelp)
    ///     .subcommand(App::new("config")
    ///         .about("Used for configuration")
    ///         .arg(Arg::new("config_file")
    ///             .help("The configuration file to use")))
    ///     .try_get_matches_from(vec!["prog"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand);
    /// ```
    ///
    /// [`Subcommand`]: crate::Subcommand
    /// [`AppSettings::ArgRequiredElseHelp`]: crate::AppSettings::ArgRequiredElseHelp
    /// [`AppSettings::SubcommandRequiredElseHelp`]: crate::AppSettings::SubcommandRequiredElseHelp
    DisplayHelpOnMissingArgumentOrSubcommand,

    /// Not a true "error" as it means `--version` or similar was used.
    /// The message will be sent to `stdout`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use clap::{App, Arg, ErrorKind};
    /// let result = App::new("prog")
    ///     .version("3.0")
    ///     .try_get_matches_from(vec!["prog", "--version"]);
    /// assert!(result.is_err());
    /// assert_eq!(result.unwrap_err().kind(), ErrorKind::DisplayVersion);
    /// ```
    DisplayVersion,

    /// Occurs when using the [`ArgMatches::value_of_t`] and friends to convert an argument value
    /// into type `T`, but the argument you requested wasn't used. I.e. you asked for an argument
    /// with name `config` to be converted, but `config` wasn't used by the user.
    ///
    /// [`ArgMatches::value_of_t`]: crate::ArgMatches::value_of_t()
    ArgumentNotFound,

    /// Represents an [I/O error].
    /// Can occur when writing to `stderr` or `stdout` or reading a configuration file.
    ///
    /// [I/O error]: std::io::Error
    Io,

    /// Represents a [Format error] (which is a part of [`Display`]).
    /// Typically caused by writing to `stderr` or `stdout`.
    ///
    /// [`Display`]: std::fmt::Display
    /// [Format error]: std::fmt::Error
    Format,
}

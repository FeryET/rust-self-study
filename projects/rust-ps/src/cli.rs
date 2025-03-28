use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "ps-rust", version, about, long_about, next_line_help = true)]
pub struct Cli {
    #[command(flatten, next_help_heading = "Selection Options")]
    selection_args: ProcessSelectionArgs,
    #[command(flatten, next_help_heading = "Output Format Options")]
    output_format_args: ProcessOutputFormatArgs,
}

#[derive(Args, Clone, Debug)]
#[group(required = false, multiple = true)]
struct ProcessSelectionArgs {
    #[arg(short = 'A', short = 'e', help = "Select all processes.")]
    all: bool,
    #[arg(
        short = 'a',
        help = "Select all processes except both session leaders (see getsid(2)) and processes not associated with a terminal."
    )]
    all_except_session_leaders_and_no_tty: bool,
    #[arg(short = 'd', help = "Select all processes except session leaders.")]
    all_except_session_leaders: bool,
    #[arg(
        short = 'N',
        long = "deselect",
        help = "Select all processes except those that fulfill the specified conditions  (negates  the  selection)."
    )]
    all_except_with_condition: bool,
}

#[derive(Args, Clone, Debug)]
#[group(required = false, multiple = true)]
struct ProcessOutputFormatArgs {
    #[arg(
        short = 'c',
        help = "Show different scheduler information for the -l option."
    )]
    scheduler_alternative_format: bool,
    #[arg(
        long = "context",
        help = "Display security context format (for SELinux)."
    )]
    context: bool,
    #[arg(
        short = 'f',
        long_help = "Do full-format listing. \
                     This option can be combined with many other UNIX-style options to add additional \
                     columns. It  also  causes  the  command  arguments  to  be printed. \
                     When used with -L, the NLWP (number of threads) and LWP (thread ID) \
                     columns will be added.  See the c option, the format keyword args, \
                     and the format keyword comm."
    )]
    full_format_list: bool,
    #[arg(
        short = 'F',
        help = "Extra full format.  See the -f option, which -F implies."
    )]
    extra_full_format: bool,
    #[arg(
        short = 'o',
        long = "format",
        help = "user-defined format.  Identical to -o and o."
    )]
    format: String,
    #[arg(short = 'j', help = "Jobs format.")]
    jobs_format: bool,
    #[arg(
        short = 'l',
        help = "Long format.  The -y option is often useful with this."
    )]
    long_format: bool,
    #[arg(
        short = 'M',
        help = "Add a column of security data.  Identical to Z (for SELinux)."
    )]
    extra_security_data_column: bool,
}

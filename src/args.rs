use clap::Parser;
use clap::ColorChoice;

#[derive(Parser, Debug)]
#[command(about, long_about=None, version, color = ColorChoice::Auto)]
pub struct GradiateArgs {
    #[arg(long, short, long_help, help="Colors of the gradient separated by commas (,)")]
    /// Colors of the gradient separated by semicolons (,)
    /// 
    /// You may specify named colors like "red" or "yellow",
    /// but you can also specify hex colors.
    /// Specifying 1 color will result in a solid output.
    pub colors: Option<String>,
    
    #[arg(long, short='o', help="Gradient offset between lines", default_value = "0.1")]
    /// Gradient offset to apply when coloring multiline content.
    /// 
    /// An offset of 0 means all aligned characters will have the same color.
    pub line_offset: f32,
    
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    /// The text to color
    /// 
    /// If empty, will read stdin
    pub text: Vec<String>
}
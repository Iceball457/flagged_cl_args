# Overview

I hate parsing command line arguments I hate parsing command line arguments I hate parsing command line argum...

# Usage

Named flags can go anywhere, so `binary first_string_arg second_string_arg -i 34` and `binary -i 34 first_string_arg second_string_arg` would result in the same structure.

```rust
use flagged_cl_args as fca;
fn main() {
    // Assume the binary is called with
    // hello_world ~/example/path 5 --remote-address www.example.com:442 -f 3.14159
    let args = fca::gather_command_line_flags(
        &[VariantFlags::path(), VariantFlags::int()],
        &[
            FlagDefinitions {
                name: "remote-address".to_string(),
                abbreviation: Some('r'),
                allowed_type: VariantFlags::socket(),
            },
            FlagDefinitions {
                name: "i-want-float".to_string(),
                abbreviation: Some('f'),
                allowed_type: Variant::float(),
            },
        ],
    );
    let bin: &str = args.binary(); // "hello_world"
    let path: Option<&Variant> = args.get_positional(0); // Some(Variant::Path( /* PathBuf with "~/example/path" inside it */ ))
    let integer: Option<&Variant> = args.get_positional(1); // Some(Variant::Int(5))
    let socket: Option<&Variant> = args.get_named("remote-address"); // Some(Variant::Socket( /* Socket address pointing to 104.18.26.120:442 */ ))
    let float: Option<&Variant> = args.get_named("i-want-float"); // Some(Variant::Float(3.14159))

    // I usually do this:
    if let Some(Variant::Float(user_input)) = args.get_named("i-want-float") {
        println!("Your number is {user_input}");
    } else {
        // Notice that the remote address flag is marked as optional with `[]`, but the i-want-float flag is not.
        // In this example, we are exiting if the flag is not present, so it's practically mandatory for the end-user.
        println!("usage: hello_world <path> <int> [--remote-address|-r <remote-address>] --i-want-float|-f <float>");
    }
}
```

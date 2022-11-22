# gta-5 native documentation generator

Generates a natives.json from script files that can be loaded as a special.json on https://nativedb.dotindustries.dev.

Stuff like multi line comments (`/* ... */`) and preprocessor statements (like `#IF`) have to be manually removed.


## Installation

```sh
git clone https://github.com/DottieDot/gta-5-native-documentation-generator.git
```

## Usage

```sh
cargo run -- --help
```

Example:
```sh
cargo run -- -s /some/path/*.sch -o ./output
```

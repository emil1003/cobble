_: {
  languages.rust.enable = true;

  tasks = {
    "cobble:build".exec = "cargo build";
    "cobble:clippy".exec = "cargo clippy";
    "cobble:test".exec = "cargo test";
  };
}

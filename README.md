# json_easy2use
Use Rust's JSON more easily like python's dict and javascript JSON. includes set, append, get, delete, and more
#How this works?
I made a series ofcustom macro's for rust Serde JSON crate. This will make it easier to work with JSON if you are farmiliar with Python or JavaScript. 

Let's take a look how this works. 

consider the following:
<pre>
// Setting values
set!(mydict, "level1.level2.level4" => [1,2,3]);

// Appending values
append!(mydict, "level1.level2" => json!({"level5": "value_d"}));

// Deleting values
delete!(mydict, "level1.level2.level3b");

// Append to root only
root_append!(mydict, json!({"new_root_key": "new_root_value"}));
</pre>

`set!`, `append!`,`delete!`, `get!`, and `root_append!` are macros avalible to use.

#Example Usage
enter commands `cargo add serde_json` and `cargo add json_easy2use`

in a new rust project, add the following to your main.rs file
<pre>
#[macro_use]
extern crate json_easy2use;
extern crate serde_json;
use serde_json::json;

fn main() {
    let mut mydict = serde_json::json!({
        "level1": {
            "level2": {
                "level3a": "value_a",
                "level3b": "value_b",
                "level3c": "value_c"
            }
        }
    });

    // Using the `get` macro to retrieve a value
    if let Some(value) = get!(mydict, "level1.level2.level3a") {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }

    // Using the `root_append` macro to add a new key-value pair at the root level
    root_append!(mydict, serde_json::json!({"new_root_key": "new_root_value"}));

    // Using the `set` macro to set a value at a specific path
    set!(mydict, "level1.level2.level4" => [1, 2, 3]);

    // Using the `append` macro to add a new key-value pair at a specific path
    append!(mydict, "level1.level2" => serde_json::json!({"level5": "value_d"}));

    // Using the `delete` macro to remove a key-value pair at a specific path
    delete!(mydict, "level1.level2.level3b");

    // Print the final JSON structure
    println!("Output");
    println!("{}", mydict);
}

</pre>
the output from this code is the following:
<pre>
Found: {
  "level3a": "value_a",
  "level3b": "value_b",
  "level3c": "value_c"
}

Output:
{
  "level1": {
    "level2": {
      "level3a": "value_a",
      "level3c": "value_c",
      "level4": [
        1,
        2,
        3
      ],
      "level5": "value_d"
    }
  },
  "new_root_key": "new_root_value"
}
</pre>

<img src="https://raw.githubusercontent.com/RetributionByRevenue/json_easy2use/main/logo.png"><br>
As seen on Crates.io: https://crates.io/crates/json_easy2use <br>
Use Rust's JSON more easily like python's dict and javascript JSON. includes set, append, get, delete, and more.

# How this works?
I made a series of custom macro's for Rust's Serde JSON crate. This will make it easier to work with JSON if you are familiar with Python or JavaScript. 

# Now added support for Saving and Loading <3

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

//Getting a value
get!(mydict, "level1.level2")

//Checking a value
exists!(mydict, "level1.level2.level3b")
    
//Loading a JSON file
load!(mydict, "./test.db");

//Saving a JSON file
save!(mydict, "./test.db");
</pre>

`set!`, `append!`,`delete!`, `get!`, and `root_append!` are macros avalible to use. <br>
Drill down to a specifc key-value entery like `level1.level2.level3b` for easy usage. <br>
Now supporting pseudo database like functionality with `load!` and `save!`. 


# Example Usage
in a new rust project,

enter commands `cargo add serde_json` and `cargo add json_easy2use`

add the following to your main.rs file:
<pre>
#[macro_use]
extern crate json_easy2use;

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
    if let Some(value) = get!(mydict, "level1.level2") {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }

    // Using the `root_append` macro to add a new key-value pair at the root level
    root_append!(mydict, json!({"new_root_key": "new_root_value"}));

    // Using the `set` macro to set a value at a specific path
    set!(mydict, "level1.level2.level4" => [1, 2, 3]);

    // Using the `append` macro to add a new key-value pair at a specific path
    append!(mydict, "level1.level2" => json!({"level5": "value_d"}));

    // Using the `delete` macro to remove a key-value pair at a specific path
    delete!(mydict, "level1.level2.level3b");

    if exists!(mydict, "level1.level2.level3b") {
        println!("Key exists!");
    } else {
        println!("Key does not exist.");
    }

    // Print the final JSON structure
    println!("Output");
    println!("{}", mydict);

    //Saving the JSON to file
    save!(mydict, "./test.db");
}

</pre>
the output from this code is the following:
<pre>
Found: {
  "level3a": "value_a",
  "level3b": "value_b",
  "level3c": "value_c"
}

Key does not exist.
    
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

and `test.db` created in the current directory.

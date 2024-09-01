<img src="https://raw.githubusercontent.com/RetributionByRevenue/json_easy2use/main/logo.png"><br>
As seen on Crates.io: https://crates.io/crates/json_easy2use <br>
Use Rust's JSON more easily like python's dict and javascript JSON. includes set, append, get, delete, and more.

# How this works?
I made a series of custom macro's for Rust's Serde JSON crate. This will make it easier to work with JSON if you are familiar with Python or JavaScript. 

<ol>

<li>
  <strong>query_key_pair!</strong>
  <p>Finds the path to a key-value pair in a JSON-like structure. The key is a string, and the value can be either a string or a complex JSON value.</p>
  <ul>
    <li><code>query_key_pair!(mydict, "key" => "value");</code></li>
    <li>Expects: <code>serde_json::Value, &str => serde_json::Value</code></li>
    <li>Returns: <code>String</code> (path to the parent object) or <code>"Null"</code> if not found.</li>
  </ul>
</li>
<br>

  
  <li>
    <strong>print_pretty!</strong>
    <p>Pretty-prints a JSON-like structure in a formatted way.</p>
    <ul>
      <li><code>print_pretty!(mydict);</code></li>
      <li>Expects: <code>serde_json::Value</code></li>
      <li>Returns: <em>This macro does not return a value; it prints the formatted JSON structure to the console.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>ensure_exist_with_schema!</strong>
    <p>Checks if a file exists, and if it does, checks if the schema exists inside it. If it dose not exist, it will make the file with the JSON specified.</p>
    <ul>
      <li><code>ensure_exist_with_schema!("test.db", serde_json::json!({"key": "value"}));</code></li>
      <li>Expects: <code>&str, serde_json::Value</code></li>
      <li>Returns: <em>This macro does not return a value; it ensures the file and schema exist.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>ensure_exist!</strong>
    <p>Ensures that a file exists by creating it if it doesn't.</p>
    <ul>
      <li><code>ensure_exist!("data.db");</code></li>
      <li>Expects: <code>&str</code></li>
      <li>Returns: <em>This macro does not return a value; it creates the file if it doesn't exist.</em></li>
    </ul>
  </li>
  <br>

  <br>
  <li>
    <strong>query_value!</strong>
    <p>Searches for a specific value in a JSON-like structure and returns its path. Value is string or complex JSON value.</p>
    <ul>
      <li><code>query_value!(mydict, "value");</code></li>
      <li>Expects: <code>serde_json::Value, &str or serde_json::Value</code></li>
      <li>Returns: <code>String</code> (path to the value) or <code>"Null"</code> if not found.</li>
    </ul>
  </li>
  <br>
  <li>
    <strong>exists!</strong>
    <p>Checks if a specific key or value exists in a JSON-like structure.</p>
    <ul>
      <li><code>exists!(mydict, "key");</code></li>
      <li>Expects: <code>serde_json::Value, &str</code></li>
      <li>Returns: <code>bool</code> (whether the key or value exists).</li>
    </ul>
  </li>
  <br>
  <li>
    <strong>load!</strong>
    <p>Loads a JSON-like structure from a file.</p>
    <ul>
      <li><code>let mydict = load!("data.json");</code></li>
      <li>Expects: <code>&str</code></li>
      <li>Returns: <code>serde_json::Value</code> (the loaded JSON structure).</li>
    </ul>
  </li>
  <br>
  <li>
    <strong>save!</strong>
    <p>Saves a JSON-like structure to a file.</p>
    <ul>
      <li><code>save!(mydict, "data.json");</code></li>
      <li>Expects: <code>serde_json::Value, &str</code></li>
      <li>Returns: <em>This macro does not return a value; it saves the JSON structure to a file.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>get!</strong>
    <p>Retrieves a value from a JSON-like structure by its path.</p>
    <ul>
      <li><code>let value = get!(mydict, "key.subkey");</code></li>
      <li>Expects: <code>serde_json::Value, &str</code></li>
      <li>Returns: <code>serde_json::Value</code> (the retrieved value).</li>
    </ul>
  </li>
  <br>
  <li>
    <strong>root_append!</strong>
    <p>Appends a key-value pair to the root of a JSON-like structure.</p>
    <ul>
      <li><code>root_append!(mydict, "new_key" => "new_value");</code></li>
      <li>Expects: <code>serde_json::Value, &str => serde_json::Value</code></li>
      <li>Returns: <em>This macro does not return a value; it appends the key-value pair to the root object.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>set!</strong>
    <p>Sets a value in a JSON-like structure by its path.</p>
    <ul>
      <li><code>set!(mydict, "key.subkey", "new_value");</code></li>
      <li>Expects: <code>serde_json::Value, &str, serde_json::Value</code></li>
      <li>Returns: <em>This macro does not return a value; it sets the value at the specified path.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>append!</strong>
    <p>Appends a value to an array within a JSON-like structure.</p>
    <ul>
      <li><code>append!(mydict, "key.array", "new_value");</code></li>
      <li>Expects: <code>serde_json::Value, &str, serde_json::Value</code></li>
      <li>Returns: <em>This macro does not return a value; it appends the value to the specified array.</em></li>
    </ul>
  </li>
  <br>
  <li>
    <strong>delete!</strong>
    <p>Deletes a key or value from a JSON-like structure by its path.</p>
    <ul>
      <li><code>delete!(mydict, "key.subkey");</code></li>
      <li>Expects: <code>serde_json::Value, &str</code></li>
      <li>Returns: <em>This macro does not return a value; it deletes the key or value at the specified path.</em></li>
    </ul>
  </li>
</ol>

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

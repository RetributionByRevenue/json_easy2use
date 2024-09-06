<img src="https://raw.githubusercontent.com/RetributionByRevenue/json_easy2use/main/logo.png"><br>
As seen on Crates.io: https://crates.io/crates/json_easy2use <br>
Use Rust's JSON more easily like python's dict and javascript JSON. includes set, append, get, delete, and more.

# How this works?
I made a series of custom macro's for Rust's Serde JSON crate. This will make it easier to work with JSON if you are familiar with Python or JavaScript. 

# Changelog 0.2.9
I added a macro exist_same_level!, for determining if multiple key-pairs exist on the same level. 
<pre>
let result = exist_same_level!(mydict,
      "destination_ip" => destination_ip,
      "packet_type" => packet_type,
      "source_ip" => source_ip
);
</pre>
Changed Macros to return `None` instead of a string `"None"` or `"Null"`. I want to avoid unnecessarily working with strings as a return unless it makes sense too.
<br>

# Macro Collection
| **Querying** | **Helper** | **File System** |
|--------------|------------|------------------|
| **`query_key_pair!`**<br>Finds the path to a key-value pair in a JSON-like structure.<br>**Example:**<br>```query_key_pair!(mydict, "key" => "value");``` | **`print_pretty!`**<br>Pretty-prints a JSON-like structure in a formatted way.<br>**Example:**<br>```print_pretty!(mydict);``` | **`ensure_exist_with_schema!`**<br>Checks if a file exists and its schema; creates it with the specified JSON if not.<br>**Example:**<br>```ensure_exist_with_schema!("test.db", serde_json::json!({"key": "value"}));``` |
| **`exist_same_level!`**<br>Checks if all provided key-value pairs exist at the same level in a JSON-like structure.<br>**Example:**<br>```exist_same_level!(mydict, "destination_ip" => destination_ip, "packet_type" => packet_type);``` | **`root_append!`**<br>Appends a key-value pair to the root of a JSON-like structure.<br>**Example:**<br>```root_append!(mydict, "new_key" => "new_value");``` | **`ensure_exist!`**<br>Ensures that a file exists by creating it if it doesn't.<br>**Example:**<br>```ensure_exist!("data.db");``` |
| **`query_value!`**<br>Searches for a specific value in a JSON-like structure and returns its path.<br>**Example:**<br>```query_value!(mydict, "value");``` | **`set!`**<br>Sets a value in a JSON-like structure by its path.<br>**Example:**<br>```set!(mydict, "key.subkey", "new_value");``` | **`load!`**<br>Loads a JSON-like structure from a file.<br>**Example:**<br>```let mydict = load!("data.json");``` |
| **`exists!`**<br>Checks if a specific key or value exists in a JSON-like structure.<br>**Example:**<br>```exists!(mydict, "key");``` | **`append!`**<br>Appends a value to an array within a JSON-like structure.<br>**Example:**<br>```append!(mydict, "key.array", "new_value");``` | **`save!`**<br>Saves a JSON-like structure to a file.<br>**Example:**<br>```save!(mydict, "data.json");``` |
| **`get!`**<br>Retrieves a value from a JSON-like structure by its path.<br>**Example:**<br>```let value = get!(mydict, "key.subkey");``` | **`delete!`**<br>Deletes a key or value from a JSON-like structure by its path.<br>**Example:**<br>```delete!(mydict, "key.subkey");``` |  |

---

### **Detailed Descriptions**

#### **Querying**

1. **`query_key_pair!`**
   - **Description:** Finds the path to a key-value pair in a JSON-like structure. The key is a string, and the value can be either a string or a complex JSON value.
   - **Usage:**
     ```rust
     query_key_pair!(mydict, "key" => "value");
     ```
   - **Returns:** `String` (path to the parent object) or `None` if not found.

2. **`exist_same_level!`**
   - **Description:** Checks if all provided key-value pairs exist at the same level in a JSON-like structure.
   - **Usage:**
     ```rust
     exist_same_level!(mydict, "destination_ip" => destination_ip, "packet_type" => packet_type);
     ```
   - **Returns:** `String` (path to the parent object) or `None` if not found.

3. **`query_value!`**
   - **Description:** Searches for a specific value in a JSON-like structure and returns its path. The value can be a string or a complex JSON value.
   - **Usage:**
     ```rust
     query_value!(mydict, "value");
     ```
   - **Returns:** `String` (path to the value) or `None` if not found.

4. **`exists!`**
   - **Description:** Checks if a specific key or value exists in a JSON-like structure.
   - **Usage:**
     ```rust
     exists!(mydict, "key");
     ```
   - **Returns:** `bool` (whether the key or value exists).

5. **`get!`**
   - **Description:** Retrieves a value from a JSON-like structure by its path.
   - **Usage:**
     ```rust
     let value = get!(mydict, "key.subkey");
     ```
   - **Returns:** `serde_json::Value` (the retrieved value).

#### **Helper**

1. **`print_pretty!`**
   - **Description:** Pretty-prints a JSON-like structure in a formatted way.
   - **Usage:**
     ```rust
     print_pretty!(mydict);
     ```
   - **Returns:** This macro does not return a value; it prints the formatted JSON structure to the console.

2. **`root_append!`**
   - **Description:** Appends a key-value pair to the root of a JSON-like structure.
   - **Usage:**
     ```rust
     root_append!(mydict, "new_key" => "new_value");
     ```
   - **Returns:** This macro does not return a value; it appends the key-value pair to the root object.

3. **`set!`**
   - **Description:** Sets a value in a JSON-like structure by its path.
   - **Usage:**
     ```rust
     set!(mydict, "key.subkey", "new_value");
     ```
   - **Returns:** This macro does not return a value; it sets the value at the specified path.

4. **`append!`**
   - **Description:** Appends a value to an array within a JSON-like structure.
   - **Usage:**
     ```rust
     append!(mydict, "key.array", "new_value");
     ```
   - **Returns:** This macro does not return a value; it appends the value to the specified array.

5. **`delete!`**
   - **Description:** Deletes a key or value from a JSON-like structure by its path.
   - **Usage:**
     ```rust
     delete!(mydict, "key.subkey");
     ```
   - **Returns:** This macro does not return a value; it deletes the key or value at the specified path.

#### **File System**

1. **`ensure_exist_with_schema!`**
   - **Description:** Checks if a file exists, and if it does, checks if the schema exists inside it. If it does not exist, it will create the file with the specified JSON.
   - **Usage:**
     ```rust
     ensure_exist_with_schema!("test.db", serde_json::json!({"key": "value"}));
     ```
   - **Returns:** This macro does not return a value; it ensures the file and schema exist.

2. **`ensure_exist!`**
   - **Description:** Ensures that a file exists by creating it if it doesn't.
   - **Usage:**
     ```rust
     ensure_exist!("data.db");
     ```
   - **Returns:** This macro does not return a value; it creates the file if it doesn't exist.

3. **`load!`**
   - **Description:** Loads a JSON-like structure from a file.
   - **Usage:**
     ```rust
     let mydict = load!("data.json");
     ```
   - **Returns:** `serde_json::Value` (the loaded JSON structure).

4. **`save!`**
   - **Description:** Saves a JSON-like structure to a file.
   - **Usage:**
     ```rust
     save!(mydict, "data.json");
     ```
   - **Returns:** This macro does not return a value; it saves the JSON structure to a file.

---

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

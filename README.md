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
in a new rust project, add the following to your main.rs file
<pre>
#[macro_use]
extern crate serde_json;

macro_rules! get {
    ($map:expr, $path:expr) => {
        {
            let mut current = &$map;
            let path_parts: Vec<&str> = $path.split('.').collect();

            for key in path_parts {
                if let Some(next) = current.get(key) {
                    current = next;
                } else {
                    // Return None if the key doesn't exist
                    break;
                }
            }

            if current.is_null() {
                None
            } else {
                Some(current.clone())
            }
        }
    };
}

macro_rules! root_append {
    ($map:expr, $value:expr) => {
        {
            // Ensure $map is a mutable reference to the root map
            let root = $map.as_object_mut().unwrap();

            // Ensure $value is an object and append its contents to the root
            if let Some(new_values) = $value.as_object() {
                root.extend(new_values.clone());
            }
        }
    };
}

macro_rules! set {
    ($map:expr, $path:expr => $value:expr) => {
        {
            let mut current = $map.as_object_mut().unwrap();
            let path_parts: Vec<&str> = $path.split('.').collect();
            let last_key = path_parts.last().unwrap().to_string();

            for key in &path_parts[..path_parts.len()-1] {
                let key_str = key.to_string();
                current = current.entry(key_str).or_insert(json!({})).as_object_mut().unwrap();
            }

            current.insert(last_key, json!($value));
        }
    };
}

macro_rules! append {
    ($map:expr, $path:expr => $value:expr) => {
        {
            let mut current = $map.as_object_mut().unwrap();
            let path_parts: Vec<&str> = $path.split('.').collect();
            let last_key = path_parts.last().unwrap().to_string();

            for key in &path_parts[..path_parts.len()-1] {
                let key_str = key.to_string();
                current = current.entry(key_str).or_insert(json!({})).as_object_mut().unwrap();
            }

            let existing_value = current.entry(last_key).or_insert(json!({}));
            if let Some(existing_object) = existing_value.as_object_mut() {
                let new_values = $value.as_object().unwrap().clone();
                existing_object.extend(new_values);
            }
        }
    };
}

macro_rules! delete {
    ($map:expr, $path:expr) => {
        {
            let mut current = $map.as_object_mut().unwrap();
            let path_parts: Vec<&str> = $path.split('.').collect();

            if path_parts.len() == 1 {
                current.remove(path_parts[0]);
            } else {
                let last_key = path_parts.last().unwrap().to_string();
                for key in &path_parts[..path_parts.len()-1] {
                    let key_str = key.to_string();
                    current = current.entry(key_str).or_insert(json!({})).as_object_mut().unwrap();
                }
                current.remove(&last_key);
            }
        }
    };
}


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
 
    if let Some(value) = get!(mydict, "level1.level2") {
        println!("Found: {}", value);
    } else {
        println!("Not found");
    }

    root_append!(mydict, json!({"new_root_key": "new_root_value"}));
 
    // Setting values
    set!(mydict, "level1.level2.level4" => [1,2,3]);

    // Appending values
    append!(mydict, "level1.level2" => json!({"level5": "value_d"}));

    // Deleting values
    delete!(mydict, "level1.level2.level3b");
    println!("");
    println!("Output:");
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

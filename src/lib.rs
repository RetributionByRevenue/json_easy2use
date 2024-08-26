#[macro_use]
extern crate serde_json;

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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

#[macro_export]
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
#[macro_export]
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















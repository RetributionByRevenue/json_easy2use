# json_easy2use
Use Rust's JSON more easily like python's dict and javascript JSON. includes set, append, get, delete, and more
#How this works?
I made a series ofcustom macro's for rust Serde JSON crate. This will make it easier to work with JSON if you are farmiliar with Python or JavaScript. 

Let's take a look how this works. 

consider the following:

    `root_append!(mydict, json!({"new_root_key": "new_root_value"}));`
 
    // Setting values
    `set!(mydict, "level1.level2.level4" => [1,2,3]);`

    // Appending values
    `append!(mydict, "level1.level2" => json!({"level5": "value_d"}));`

    // Deleting values
    `delete!(mydict, "level1.level2.level3b");`


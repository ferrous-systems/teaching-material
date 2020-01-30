= Exercise: Accepting JSON
:icons: font
:source-highlighter: pygments
:pygments-style: borland

:source-language: rust

In this exercise, we will implement a simple JSON API.

You will learn:

* How to validate JSON using `serde_json::Value`
* How to describe a JSON structure in Rust types

== Task

1. Create a new Terrarium project
2. Accept the following JSON
+
[source,json]
----
{
    "name": "John Doe",
    "age": 43,
    "address": {
        "street": "10 Downing Street",
        "city": "London"
    },
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]
}
----
3. On error, return an appropriate error response
4. On success, return the first phone number

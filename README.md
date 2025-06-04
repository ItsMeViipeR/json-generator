# json-generator

Json generator is a scripting language and a cli tool to create json file easily.

#### Using:

```
string = "abc"
number = 3
boolean = true
array = [1, 2]
object = { "key": "value" }
result -> { string, number, boolean, array, object }
```

Create a .json file from our .jg file:

```
jg ./jgfile.jg
```

---

#### Comments:

Comments are also available with `//` or `#`

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

#### Variables:

You can use variables just to use it in a result

```jg
a = 1
obj -> { a }
```

Or you can pass it to arrays and objects

```
username1 = "user1"
age1 = 20
hobbies1 = ["first", "second"]
user1 = { username1, age1, hobbies1 }

username2 = "user2"
age2 = 21
hobbies2 = ["first", "second"]
user2 = { username2, age2, hobbies2 }

users -> { user1, user2 }
```

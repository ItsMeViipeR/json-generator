# json-generator
Json generator is a scripting language and a cli tool to create json file easily.
#
Using:
```
a -> ["a", "b", "c"];
b -> 3;
c -> "foo";
d -> { a, b, c }
```
Create a .json file from our .jg file:
```
jg ./jgfile.jg
```
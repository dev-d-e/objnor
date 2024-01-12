# objnor

A text format for object notation.

text.

 ```
0~a:a
+a
|b
|c
0~b:a
1~a
2~a:b
0~c:
|
|cc
|
```

object.

```
[Target { name: "a", text: ["a", "a\nb\nc"], value: [] },

Target { name: "b", text: ["a"], value: [Target { name: "a", text: [], value: [Target { name: "a", text: ["b"], value: [] }] }] },

Target { name: "c", text: ["\n\ncc\n"], value: [] }]
```

* start with offset and tilde(~), then data key, then colon(:). If next offset is greater, it's child object.

* multiple lines text use vertical(|).

* array text use plus(+).

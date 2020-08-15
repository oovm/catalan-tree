# Catalan Tree

Binary tree of n variables where the parent node represents a function and the child node represents a value.

- n = 2

```js
f(n, n)
```

- n = 3

```js
f(n, f(n, n))
f(f(n, n), n)
```

- n = 4

```js
f(n, f(n, f(n, n)))
f(n, f(f(n, n), n))
f(f(n, n), f(n, n))
f(f(n, f(n, n)), n)
f(f(f(n, n), n), n)
```

- n = 5

```js
f(n, f(n, f(n, f(n, n))))
f(n, f(n, f(f(n, n), n)))
f(n, f(f(n, n), f(n, n)))
f(n, f(f(n, f(n, n)), n))
f(n, f(f(f(n, n), n), n))
f(f(n, n), f(n, f(n, n)))
f(f(n, n), f(f(n, n), n))
f(f(n, f(n, n)), f(n, n))
f(f(f(n, n), n), f(n, n))
f(f(n, f(n, f(n, n))), n)
f(f(n, f(f(n, n), n)), n)
f(f(f(n, n), f(n, n)), n)
f(f(f(n, f(n, n)), n), n)
f(f(f(f(n, n), n), n), n)
```

- n = 6

```js
f(n, f(n, f(n, f(n, f(n, n)))))
f(n, f(n, f(n, f(f(n, n), n))))
f(n, f(n, f(f(n, n), f(n, n))))
f(n, f(n, f(f(n, f(n, n)), n)))
f(n, f(n, f(f(f(n, n), n), n)))
f(n, f(f(n, n), f(n, f(n, n))))
f(n, f(f(n, n), f(f(n, n), n)))
f(n, f(f(n, f(n, n)), f(n, n)))
f(n, f(f(f(n, n), n), f(n, n)))
f(n, f(f(n, f(n, f(n, n))), n))
f(n, f(f(n, f(f(n, n), n)), n))
f(n, f(f(f(n, n), f(n, n)), n))
f(n, f(f(f(n, f(n, n)), n), n))
f(n, f(f(f(f(n, n), n), n), n))
f(f(n, n), f(n, f(n, f(n, n))))
f(f(n, n), f(n, f(f(n, n), n)))
f(f(n, n), f(f(n, n), f(n, n)))
f(f(n, n), f(f(n, f(n, n)), n))
f(f(n, n), f(f(f(n, n), n), n))
f(f(n, f(n, n)), f(n, f(n, n)))
f(f(n, f(n, n)), f(f(n, n), n))
f(f(f(n, n), n), f(n, f(n, n)))
f(f(f(n, n), n), f(f(n, n), n))
f(f(n, f(n, f(n, n))), f(n, n))
f(f(n, f(f(n, n), n)), f(n, n))
f(f(f(n, n), f(n, n)), f(n, n))
f(f(f(n, f(n, n)), n), f(n, n))
f(f(f(f(n, n), n), n), f(n, n))
f(f(n, f(n, f(n, f(n, n)))), n)
f(f(n, f(n, f(f(n, n), n))), n)
f(f(n, f(f(n, n), f(n, n))), n)
f(f(n, f(f(n, f(n, n)), n)), n)
f(f(n, f(f(f(n, n), n), n)), n)
f(f(f(n, n), f(n, f(n, n))), n)
f(f(f(n, n), f(f(n, n), n)), n)
f(f(f(n, f(n, n)), f(n, n)), n)
f(f(f(f(n, n), n), f(n, n)), n)
f(f(f(n, f(n, f(n, n))), n), n)
f(f(f(n, f(f(n, n), n)), n), n)
f(f(f(f(n, n), f(n, n)), n), n)
f(f(f(f(n, f(n, n)), n), n), n)
f(f(f(f(f(n, n), n), n), n), n)
```

- more

```wl
treeR[1] = n;
treeR[n_] := treeR[n] = Table[f[treeR[a], treeR[n - a]], {a, 1, n - 1}]
treeC[n_] := Flatten[treeR[n] //. {f[a_List, b_] :> (f[#, b]& /@ a), f[a_, b_List] :> (f[a, #]& /@ b)}]
StringRiffle[
    StringReplace[ToString[#], {"[" -> "(", "]" -> ")"}]& /@ treeC[6],
    "\n"
]
% // CopyToClipboard
```

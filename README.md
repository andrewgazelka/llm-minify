# llm-minify

Minimize the number of tokens a file requires for an LLM to understand it.
For instance:

```html

<xml>
    <foo>
        <bar>baz</bar>
    </foo>
</xml>
```

becomes

```text
a=xml,b=foo,c=bar
<a><b><c>baz</c></b></a>
```


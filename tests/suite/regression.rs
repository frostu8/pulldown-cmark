// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn regression_test_1() {
    let original = r##"<details><summary>Testing 1..2..3..</summary>

This is a test of the details element.

</details>
"##;
    let expected = r##"<details><summary>Testing 1..2..3..</summary>
<p>This is a test of the details element.</p>
</details>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_2() {
    let original = r##"see the [many] [articles] [on] [QuickCheck].

[many]: https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6
[articles]: http://www.quviq.com/products/erlang-quickcheck/
[on]: https://wiki.haskell.org/Introduction_to_QuickCheck1
[QuickCheck]: https://hackage.haskell.org/package/QuickCheck
"##;
    let expected = r##"<p>see the 
  <a href="https://medium.com/@jlouis666/quickcheck-advice-c357efb4e7e6">many</a> 
  <a href="http://www.quviq.com/products/erlang-quickcheck/">articles</a> 
  <a href="https://wiki.haskell.org/Introduction_to_QuickCheck1">on</a> 
  <a href="https://hackage.haskell.org/package/QuickCheck">QuickCheck</a>.
</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_3() {
    let original = r##"[![debug-stub-derive on crates.io][cratesio-image]][cratesio]
[![debug-stub-derive on docs.rs][docsrs-image]][docsrs]

[cratesio-image]: https://img.shields.io/crates/v/debug_stub_derive.svg
[cratesio]: https://crates.io/crates/debug_stub_derive
[docsrs-image]: https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0
[docsrs]: https://docs.rs/debug_stub_derive/0.3.0/
"##;
    let expected = r##"<p><a href="https://crates.io/crates/debug_stub_derive"><img src="https://img.shields.io/crates/v/debug_stub_derive.svg" alt="debug-stub-derive on crates.io" /></a>
<a href="https://docs.rs/debug_stub_derive/0.3.0/"><img src="https://docs.rs/debug_stub_derive/badge.svg?version=0.3.0" alt="debug-stub-derive on docs.rs" /></a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_4() {
    let original = r##"|  Title A  |  Title B  |
| --------- | --------- |
| Content   | Content   |

|  Title A  |  Title B  |  Title C  |  Title D  |
| --------- | --------- | --------- | ---------:|
| Content   | Content   | Conent    | Content   |
"##;
    let expected = r##"<table><thead><tr><th>Title A  </th><th>Title B  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td></tr>
</tbody></table>
<table><thead><tr><th>Title A  </th><th>Title B  </th><th>Title C  </th><th style="text-align: right">Title D  </th></tr></thead><tbody>
<tr><td>Content   </td><td>Content   </td><td>Conent    </td><td style="text-align: right">Content   </td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_5() {
    let original = r##"foo§__(bar)__
"##;
    let expected = r##"<p>foo§<strong>(bar)</strong></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_6() {
    let original = r##"<https://example.com> hello
"##;
    let expected = r##"<p><a href="https://example.com">https://example.com</a> hello</p>

"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_7() {
    let original = r##"[foo][bar]

<!-- foo -->
[bar]: a
"##;
    let expected = r##"<p><a href="a">foo</a></p>
<!-- foo -->
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_8() {
    let original = r##"<!-- <dl> -->
- **foo** (u8, u8)

  make something

- **bar** (u16, u16)

  make something
"##;
    let expected = r##"<!-- <dl> -->
<ul>
<li>
<p><strong>foo</strong> (u8, u8)</p>
<p>make something</p>
</li>
<li>
<p><strong>bar</strong> (u16, u16)</p>
<p>make something</p>
</li>
</ul>

"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_9() {
    let original = r##"[`
i8
`](
../../../std/primitive.i8.html
)
"##;
    let expected = r##"<p><a href="../../../std/primitive.i8.html"><code>i8</code></a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_10() {
    let original = r##"[a]

[a]: /url (title\\*)
"##;
    let expected = r##"<p><a href="/url" title="title\*">a</a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_11() {
    let original = r##"[a]

[a]: /url (title\))
"##;
    let expected = r##"<p><a href="/url" title="title)">a</a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_12() {
    let original = r##"[a]

[a]: /url (title))
"##;
    let expected = r##"<p>[a]</p>
<p>[a]: /url (title))</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_13() {
    let original = r##"a <?php this is not a valid processing tag
---
b <?php but this is ?>
"##;
    let expected = r##"<h2>a &lt;?php this is not a valid processing tag</h2>
<p>b <?php but this is ?></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_14() {
    let original = r##"[a]: u\
foo
"##;
    let expected = r##"<p>foo</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_15() {
    let original = r##"\`foo`
"##;
    let expected = r##"<p>`foo`</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_16() {
    let original = r##"foo\\
bar
"##;
    let expected = r##"<p>foo\
bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_17() {
    let original = r##"1\. foo

1\) bar
"##;
    let expected = r##"<p>1. foo</p>
<p>1) bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_18() {
    let original = r##"1...

1.2.3.

1 2 3 .

1.|2.-3.

1)2)3)
"##;
    let expected = r##"<p>1...</p>
<p>1.2.3.</p>
<p>1 2 3 .</p>
<p>1.|2.-3.</p>
<p>1)2)3)</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_19() {
    let original = r##"[](<<>)
"##;
    let expected = r##"<p>[](&lt;&lt;&gt;)</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_20() {
    let original = r##"\``foo``bar`
"##;
    let expected = r##"<p>`<code>foo``bar</code></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_21() {
    let original = r##"\\`foo`
"##;
    let expected = r##"<p>\<code>foo</code></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_22() {
    let original = r##"[\\]: x

YOLO
"##;
    let expected = r##"<p>YOLO</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_23() {
    let original = r##"lorem ipsum
A | B
---|---
foo | bar
"##;
    let expected = r##"<p>lorem ipsum
A | B
---|---
foo | bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_24() {
    let original = r##"foo|bar  
---|---
foo|bar
"##;
    let expected = r##"<table><thead><tr><th>foo</th><th>bar</th></tr></thead>
<tbody><tr><td>foo</td><td>bar</td></tr></tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_25() {
    let original = r##"foo|bar\\
---|---
foo|bar
"##;
    let expected = r##"<table><thead><tr><th>foo</th><th>bar\</th></tr></thead>
<tbody><tr><td>foo</td><td>bar</td></tr></tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_26() {
    let original = r##"[<foo>](url)
"##;
    let expected = r##"<p><a href="url"><foo></a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_27() {
    let original = r##"[<foo>bar</foo>](url)
"##;
    let expected = r##"<p><a href="url"><foo>bar</foo></a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_28() {
    let original = r##"![<http://example.com>](http://example.com/logo.png)
"##;
    let expected = r##"<p><img alt="http://example.com" src="http://example.com/logo.png"></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_29() {
    let original = r##"[<http://one> <http://two>](url)
"##;
    let expected = r##"<p><a href="url"></a><a href="http://one">http://one</a> <a href="http://two">http://two</a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_30() {
    let original = r##"Markdown | Less | Pretty
--- | --- | ---
 
some text
"##;
    let expected = r##"<table><thead><tr><th>Markdown </th><th> Less </th><th> Pretty</th></tr></thead><tbody>
</tbody></table>
<p>some text</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_31() {
    let original = r##"1. > foo
2. >
"##;
    let expected = r##"<ol>
<li>
<blockquote>
<p>foo</p>
</blockquote>
</li>
<li>
<blockquote>
</blockquote>
</li>
</ol>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_32() {
    let original = r##"[
x

]: f
"##;
    let expected = r##"<p>[
x</p>
<p>]: f</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_33() {
    let original = r##"[foo]:
"##;
    let expected = r##"<p>[foo]:</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_34() {
    let original = r##"> [foo
> bar]: /url
>
> [foo bar]
"##;
    let expected = r##"<blockquote>
<p><a href="/url">foo bar</a></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_35() {
    let original = r##"> foo | bar
> --- | ---
yolo | swag
"##;
    let expected = r##"<blockquote>
<table><thead><tr><th>foo</th><th>bar</th></tr></thead></table>
</blockquote>
<p>yolo | swag</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_36() {
    let original = r##"<foo bar>
"##;
    let expected = r##"<foo bar>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_37() {
    let original = r##"<foo bar =
 "hi"> 
"##;
    let expected = r##"<p><foo bar =
 "hi"> </p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_38() {
    let original = r##"~~*_**__

__a__
"##;
    let expected = r##"<p>~~*_**__</p>
<p><strong>a</strong></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_39() {
    let original = r##"> `
> `
"##;
    let expected = r##"<blockquote>
<p><code></code></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_40() {
    let original = r##"`\|`
"##;
    let expected = r##"<p><code>\|</code></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_41() {
    let original = r##"Paragraph 1
    
Paragraph 2
"##;
    let expected = r##"<p>Paragraph 1</p>
<p>Paragraph 2</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_42() {
    let original = r##"\[[link text](https://www.google.com/)\]
"##;
    let expected = r##"<p>[<a href="https://www.google.com/">link text</a>]</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_43() {
    let original = r##"foo | bar
--- | ---
[a](< | url>)
"##;
    let expected = r##"<table><thead><tr><th>foo</th><th>bar</th></tr></thead><tbody><tr><td>[a](&lt;</td><td>url&gt;)</td></tr></tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_44() {
    let original = r##"[a](url "
- - -
")
"##;
    let expected = r##"<p>[a](url "</p>
<hr>
<p>")</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_45() {
    let original = r##"[a](url

)
"##;
    let expected = r##"<p>[a](url</p>
<p>)</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_46() {
    let original = r##"[a](b "

")
"##;
    let expected = r##"<p>[a](b &quot;</p>
<p>&quot;)</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_47() {
    let original = r##"<http:// >
"##;
    let expected = r##"<p>&lt;http:// &gt;</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_48() {
    let original = r##"<http://>
"##;
    let expected = r##"<p>&lt;http://&gt;</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_49() {
    let original = r##"foo | bar
--- | ---
<http://| baz
"##;
    let expected = r##"<table>
<thead>
<tr><th>foo</th><th>bar</th></tr>
</thead>
<tbody>
<tr><td>&lt;http://</td><td>baz</td></tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_50() {
    let original = r##"foo | bar
--- | ---
<http://|>
"##;
    let expected = r##"<table>
<thead>
<tr><th>foo</th><th>bar</th></tr>
</thead>
<tbody>
<tr><td>&lt;http://</td><td>&gt;</td></tr>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_51() {
    let original = r##"<sup>\*hi</sup>\_
"##;
    let expected = r##"<p><sup>*hi</sup>_</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_52() {
    let original = r##"email: <john@example.com>\_
"##;
    let expected = r##"<p>email: <a href="mailto:john@example.com">john@example.com</a>_</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_53() {
    let original = r##"> [link](/url 'foo
> bar')
"##;
    let expected = r##"<blockquote>
<p><a href="/url" title="foo
bar">link</a></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_54() {
    let original = r##"> [foo
> bar]: /url
>
> [foo bar]
"##;
    let expected = r##"<blockquote>
<p><a href="/url">foo bar</a></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_55() {
    let original = r##"> [foo   bar]: /url
>
> [foo
> bar]
"##;
    let expected = r##"<blockquote>
<p><a href="/url">foo
bar</a></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_56() {
    let original = r##"> - [a
> b c]: /foo

[a b c]
"##;
    let expected = r##"<blockquote>
<ul>
<li></li>
</ul>
</blockquote>
<p><a href="/foo">a b c</a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_57() {
    let original = r##"[a
> b]: /foo

[a b] [a > b]
"##;
    let expected = r##"<p>[a</p>
<blockquote>
<p>b]: /foo</p>
</blockquote>
<p>[a b] [a > b]</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_58() {
    let original = r##"[`cargo
package`]

[`cargo package`]: https://example.com
"##;
    let expected = r##"<p><a href="https://example.com"><code>cargo package</code></a></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_59() {
    let original = r##"> [`cargo
> package`]

[`cargo package`]: https://example.com
"##;
    let expected = r##"<blockquote>
<p><a href="https://example.com"><code>cargo package</code></a></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_60() {
    let original = r##"> `cargo
> package`
"##;
    let expected = r##"<blockquote>
<p><code>cargo package</code></p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_61() {
    let original = r##"> Note: Though you should not rely on this, all pointers to <abbr
> title="Dynamically Sized Types">DSTs</abbr> are currently twice the size of
> the size of `usize` and have the same alignment.
"##;
    let expected = r##"<blockquote>
<p>Note: Though you should not rely on this, all pointers to
<abbr title="Dynamically Sized Types">DSTs</abbr> are currently twice the size of
the size of <code>usize</code> and have the same alignment.</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_62() {
    let original = r##"Lorem ipsum.[^a]

An unordered list before the footnotes:
* Ipsum
* Lorem

[^a]: Cool.
"##;
    let expected = r##"<p>Lorem ipsum.<sup class="footnote-reference"><a href="#a">1</a></sup></p>
<p>An unordered list before the footnotes:</p>
<ul>
    <li>Ipsum</li>
    <li>Lorem</li>
</ul>
<div class="footnote-definition" id="a"><sup class="footnote-definition-label">1</sup>
    <p>Cool.</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_63() {
    let original = r##"[][a]

[a]: b

# assimp-rs [![][crates-badge]][crates]

[crates]: https://crates.io/crates/assimp
[crates-badge]: http://meritbadge.herokuapp.com/assimp
"##;
    let expected = r##"<p><a href="b"></a></p>

<h1>assimp-rs <a href="https://crates.io/crates/assimp"><img alt="" src="http://meritbadge.herokuapp.com/assimp"></a></h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_64() {
    let original = r##"* A list.

   * A sublist.

   * Another sublist.


* A list.
 
   * A sublist.
 
   * Another sublist.
 
"##;
    let expected = r##"<ul>
<li>
<p>A list.</p>
<ul>
<li>
<p>A sublist.</p>
</li>
<li>
<p>Another sublist.</p>
</li>
</ul>
</li>
<li>
<p>A list.</p>
<ul>
<li>
<p>A sublist.</p>
</li>
<li>
<p>Another sublist.</p>
</li>
</ul>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_65() {
    let original = r##"<foo
"##;
    let expected = r##"<p>&lt;foo</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_66() {
    let original = r##"> > a <a href
> > ="yo
> > lo">
"##;
    let expected = r##"<blockquote>
<blockquote>
<p>a <a href
="yo
lo"></p>
</blockquote>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_67() {
    let original = r##"	-	the whitespace here are tabs
"##;
    let expected = r##"<pre><code>-	the whitespace here are tabs
</code></pre>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_68() {
    let original = r##"1. a
   1. a

a
2. a
"##;
    let expected = r##"<ol>
<li>a
<ol>
<li>a</li>
</ol>
</li>
</ol>
<p>a
2. a</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_69() {
    let original = r##"1. a
2. a
   2. a
"##;
    let expected = r##"<ol>
<li>a</li>
<li>a
2. a</li>
</ol>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_70() {
    let original = r##"* [ ] foo

* [ ] bar

baz
"##;
    let expected = r##"<ul>
<li><p><input disabled="" type="checkbox"> foo</p></li><li><p><input disabled="" type="checkbox"> bar</p></li>
</ul>
<p>baz</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_71() {
    let original = r##"* foo
    + bar
    + baz
"##;
    let expected = r##"<ul>
<li>foo
<ul>
<li>bar</li>
<li>baz</li>
</ul>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_72() {
    let original = r##"[`]: xx:

[`]`]
"##;
    let expected = r##"<p>[<code>]</code>]</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_73() {
    let original = r##"~~foo~~bar
"##;
    let expected = r##"<p><del>foo</del>bar</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_74() {
    let original = r##"foo~~bar~~
"##;
    let expected = r##"<p>foo<del>bar</del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_75() {
    let original = r##"*~~__emphasis strike strong__~~* ~~*__strike emphasis strong__*~~
"##;
    let expected = r##"<p><em><del><strong>emphasis strike strong</strong></del></em> <del><em><strong>strike emphasis strong</strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_76() {
    let original = r##"*~~__emphasis strike strong__~~* ~~*__`strike emphasis strong code`__*~~
"##;
    let expected = r##"<p><em><del><strong>emphasis strike strong</strong></del></em> <del><em><strong><code>strike emphasis strong code</code></strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_77() {
    let original = r##"*~~`emphasis strike code`~~* ~~*__strike emphasis strong__*~~
"##;
    let expected = r##"<p><em><del><code>emphasis strike code</code></del></em> <del><em><strong>strike emphasis strong</strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_78() {
    let original = r##"*~~`emphasis strike code`~~* ~~*__`strike emphasis strong code`__*~~
"##;
    let expected = r##"<p><em><del><code>emphasis strike code</code></del></em> <del><em><strong><code>strike emphasis strong code</code></strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_79() {
    let original = r##"**~~_strong strike emphasis_~~** ~~*__strike emphasis strong__*~~
"##;
    let expected = r##"<p><strong><del><em>strong strike emphasis</em></del></strong> <del><em><strong>strike emphasis strong</strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_80() {
    let original = r##"**~~_strong strike emphasis_~~** ~~*__`strike emphasis strong code`__*~~
"##;
    let expected = r##"<p><strong><del><em>strong strike emphasis</em></del></strong> <del><em><strong><code>strike emphasis strong code</code></strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_81() {
    let original = r##"**~~`strong strike code`~~** ~~*__strike emphasis strong__*~~
"##;
    let expected = r##"<p><strong><del><code>strong strike code</code></del></strong> <del><em><strong>strike emphasis strong</strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_82() {
    let original = r##"**~~`strong strike code`~~** ~~*__`strike emphasis strong code`__*~~
"##;
    let expected = r##"<p><strong><del><code>strong strike code</code></del></strong> <del><em><strong><code>strike emphasis strong code</code></strong></em></del></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_83() {
    let original = r##"  | foo | bar  |
  |-----|------|
  | baz | alef |

  | foo | bar  |
  |-----|------|
| baz | alef |

  | foo | bar  |
|-----|------|
  | baz | alef |

| foo | bar  |
  |-----|------|
  | baz | alef |

  | foo | bar  |
|-----|------|
| baz | alef |

| foo | bar  |
|-----|------|
  | baz | alef |

| foo | bar  |
|-----|------|
| baz | alef |

| foo | bar  |
  |-----|------|
| baz | alef |

    | foo | bar  |
    |-----|------|
    | baz | alef |

| foo | bar  |
    |-----|------|
| baz | alef |

	| foo | bar  |
	|-----|------|
	| baz | alef |

| foo | bar  |
	|-----|------|
| baz | alef |
"##;
    let expected = r##"<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<table><thead><tr><th> foo </th><th> bar  </th></tr></thead>
<tr><td> baz </td><td> alef </td></tr>
</table>
<pre><code>| foo | bar  |
|-----|------|
| baz | alef |
</code></pre>
<p>| foo | bar  |
    |-----|------|
| baz | alef |</p>
<pre><code>| foo | bar  |
|-----|------|
| baz | alef |
</code></pre>
<p>| foo | bar  |
	|-----|------|
| baz | alef |</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_84() {
    let original = r##"### ###
"##;
    let expected = r##"<h3></h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_85() {
    let original = r##"### 
"##;
    let expected = r##"<h3></h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_86() {
    let original = r##"<!doctype html>
"##;
    let expected = r##"<!doctype html>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_87() {
    let original = r##"| A | table    |
| ------ | ---- |
| not   |  in  |
| a     | list|

* A blockquote:
  > inside a list item
* A Heading:
  # inside a list item
* A table:

  | inside | a       |
  | ------ | ------- |
  | list   | item    |
  | with   | leading |
  | empty  | lines   |
* A table:
  | inside  | a       |
  | ------- | ------- |
  | list    | item    |
  | without | leading |
  | empty   | lines   |
"##;
    let expected = r##"<table><thead><tr><th>A</th><th>table</th></tr></thead><tbody>
<tr><td>not</td><td>in</td></tr>
<tr><td>a</td><td>list</td></tr>
</tbody></table>
<ul>
<li>
<p>A blockquote:</p>
<blockquote>
<p>inside a list item</p>
</blockquote>
</li>
<li>
<p>A Heading:</p>
<h1>inside a list item</h1>
</li>
<li>
<p>A table:</p>
<table><thead><tr><th>inside</th><th>a</th></tr></thead><tbody>
<tr><td>list</td><td>item</td></tr>
<tr><td>with</td><td>leading</td></tr>
<tr><td>empty</td><td>lines</td></tr>
</tbody></table>
</li>
<li>
<p>A table:</p>
<table><thead><tr><th>inside</th><th>a</th></tr></thead><tbody>
<tr><td>list</td><td>item</td></tr>
<tr><td>without</td><td>leading</td></tr>
<tr><td>empty</td><td>lines</td></tr>
</tbody></table>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_88() {
    let original = r##"a\

b
"##;
    let expected = r##"<p>a\</p>
<p>b</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_89() {
    let original = r##"a\
* b
"##;
    let expected = r##"<p>a\</p>
<ul>
<li>b</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_90() {
    let original = r##"a\
> b
"##;
    let expected = r##"<p>a\</p>
<blockquote>
<p>b</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_91() {
    let original = r##"a\
# b
"##;
    let expected = r##"<p>a\</p>
<h1>b</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_92() {
    let original = r##"a\
==
"##;
    let expected = r##"<h1>a\</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_93() {
    let original = r##"> a\
>
"##;
    let expected = r##"<blockquote>
<p>a\</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_94() {
    let original = r##"<a
>
"##;
    let expected = r##"<p>&lt;a</p>
<blockquote></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_95() {
    let original = r##"<div
>
"##;
    let expected = r##"<div
>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_96() {
    let original = r##"<a
> quote
"##;
    let expected = r##"<p>&lt;a</p>
<blockquote><p>quote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_97() {
    let original = r##"<div
> not quote
"##;
    let expected = r##"<div
> not quote
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_98() {
    let original = r##"<a
>quote
"##;
    let expected = r##"<p>&lt;a</p>
<blockquote><p>quote</p></blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_99() {
    let original = r##"<div
>not quote
"##;
    let expected = r##"<div
>not quote
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_100() {
    let original = r##"> alpha
> | a | b |
> |---|---|
> | c | d |
"##;
    let expected = r##"<blockquote>
<p>alpha</p>
<table>
<thead><tr><th>a</th><th>b</th></tr></thead><tbody>
<tr><td>c</td><td>d</td></tr>
</tbody></table>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_101() {
    let original = r##"***R]*-*
"##;
    let expected = r##"<p>*<em><em>R]</em>-</em></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_102() {
    let original = r##"****foo*bar*baz****
"##;
    let expected = r##"<p><strong><em><em>foo</em>bar</em>baz</strong>**</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_103() {
    let original = r##";
*
%
"##;
    let expected = r##"<p>;
*
%</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_104() {
    let original = r##";
* 
%
"##;
    let expected = r##"<p>;
* 
%</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_105() {
    let original = r##"<@1>
"##;
    let expected = r##"<p>&lt;@1&gt;</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_106() {
    let original = r##"---
anything:
    indented4Spaces: true
---
Things
"##;
    let expected = r##"<p>Things</p>
"##;

    test_markdown_html(original, expected, false, true, false);
}

#[test]
fn regression_test_107() {
    let original = r##"---
something:
  nested:
    twice: true
---
Things
"##;
    let expected = r##"<p>Things</p>
"##;

    test_markdown_html(original, expected, false, true, false);
}

#[test]
fn regression_test_108() {
    let original = r##"---
lists:
    - indented 4 spaces
---
Things
"##;
    let expected = r##"<p>Things</p>
"##;

    test_markdown_html(original, expected, false, true, false);
}

#[test]
fn regression_test_109() {
    let original = r##"-

-


-



-
"##;
    let expected = r##"<ul>
<li></li>
<li></li>
<li></li>
<li></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_110() {
    let original = r##"-

  -  .
"##;
    let expected = r##"<ul>
<li></li>
<li><p>.</p></li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_111() {
    let original = r##"j***5*=*
"##;
    let expected = r##"<p>j*<em><em>5</em>=</em></p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_112() {
    let original = r##"Not enough table

|
|

Not enough table

|x
|

Not enough table

|
|-

Table

|x
|-

Not enough table
| col1 | col2 |
|      | ---- |

Not enough table
| col1 | col2 |
| :    | ---- |

Table
| col1 | col2 |
| ---- | ---- |
"##;
    let expected = r##"<p>Not enough table</p>
<p>|
|</p>
<p>Not enough table</p>
<p>|x
|</p>
<p>Not enough table</p>
<p>|
|-</p>
<p>Table</p>
<table>
<thead><tr><th>x</th></tr></thead>
<tbody>
</tbody>
</table>
<p>Not enough table
| col1 | col2 |
|      | ---- |</p>
<p>Not enough table
| col1 | col2 |
| :    | ---- |</p>
<p>Table</p>
<table>
<thead><tr><th>col1</th><th>col2</th></tr></thead>
<tbody>
</tbody>
</table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_113() {
    let original = r##"[x] is not a valid link definition, because parens aren't balanced.

[x]: (
"##;
    let expected = r##"<p>[x] is not a valid link definition, because parens aren't balanced.</p>
<p>[x]: (</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_114() {
    let original = r##"Both of these two paragraphs are structurally the same, but the first one has
an unmatched asterisk.

_*_
*{*{

_x_
*{*{
"##;
    let expected = r##"<p>Both of these two paragraphs are structurally the same, but the first one has
an unmatched asterisk.</p>
<p><em>*</em>
<em>{</em>{</p>
<p><em>x</em>
<em>{</em>{</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_115() {
    let original = r##"**a.*.**a*.**.
"##;
    let expected = r##"<p>*<em>a.*.<em><em>a</em>.</em></em>.</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_116() {
    let original = r##"_*__*_*

_*xx*_*

_*__-_-

_*xx-_-
"##;
    let expected = r##"<p><em><em>__</em></em>*</p>
<p><em><em>xx</em></em>*</p>
<p><em>*__-</em>-</p>
<p><em>*xx-</em>-</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_117() {
    let original = r##"-

	-


- x

	-
"##;
    let expected = r##"<ul>
<li></li>
</ul>
<pre><code>-
</code></pre>
<ul>
<li>
<p>x</p>
<ul>
<li></li>
</ul>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_118() {
    let original = r##"-

    -


- x

    -
"##;
    let expected = r##"<ul>
<li></li>
</ul>
<pre><code>-
</code></pre>
<ul>
<li>
<p>x</p>
<ul>
<li></li>
</ul>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_119() {
    let original = r##"[x\

]: https://rust-lang.org
"##;
    let expected = r##"<p>[x\</p>
<p>]: https://rust-lang.org</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_120() {
    let original = r##">>**#*
>
>**#*

**#*

|**#*|
|----|
|**#*|
|**#*
**#*|
**#*
"##;
    let expected = r##"<blockquote>
<blockquote>
<p>*<em>#</em></p>
</blockquote>
<p>*<em>#</em></p>
</blockquote>
<p>*<em>#</em></p>
<table><thead><tr><th>*<em>#</em></th></tr></thead><tbody>
<tr><td>*<em>#</em></td></tr>
<tr><td>*<em>#</em></td></tr>
<tr><td>*<em>#</em></td></tr>
<tr><td>*<em>#</em></td></tr>
</tbody></table>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_121() {
    let original = r##"The second hyphen should parse the same way in both samples.

 - >*

   -

The second hyphen should parse the same way in both samples.

 - >x

   -
"##;
    let expected = r##"<p>The second hyphen should parse the same way in both samples.</p>
<ul>
<li>
<blockquote>
<ul>
<li></li>
</ul>
</blockquote>
<ul>
<li></li>
</ul>
</li>
</ul>
<p>The second hyphen should parse the same way in both samples.</p>
<ul>
<li>
<blockquote>
<p>x</p>
</blockquote>
<ul>
<li></li>
</ul>
</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_122() {
    let original = r##"> Rewriting it in [Rust] is usually a bad idea.
>
> [Rust]:
https://rust-lang.org
"##;
    let expected = r##"<blockquote>
<p>Rewriting it in <a href="https://rust-lang.org">Rust</a> is usually a bad idea.</p>
</blockquote>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_123() {
    let original = r##"[First try
----------
Second try]: https://rust-lang.org
"##;
    let expected = r##"<h2>[First try</h2>
<p>Second try]: https://rust-lang.org</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_124() {
    let original = r##"[^foo][]

[^foo][baz]

[baz][^foo]

[^foo]: bar

[baz]: https://rust-lang.org
"##;
    let expected = r##"<p><sup class="footnote-reference"><a href="#foo">1</a></sup>[]</p>
<p><a href="https://rust-lang.org">^foo</a></p>
<p>[baz]<sup class="footnote-reference"><a href="#foo">1</a></sup></p>
<div class="footnote-definition" id="foo"><sup class="footnote-definition-label">1</sup>
<p>bar</p>
</div>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_125() {
    let original = r##"# foo \
bar \
"##;
    let expected = r##"<h1>foo \</h1>
<p>bar \</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn regression_test_126() {
    let original = r##"[third try]:
-

[third try]
"##;
    let expected = r##"<h2>[third try]:</h2>
<p>[third try]</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

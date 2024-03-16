// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn heading_attrs_test_1() {
    let original = r##"with the ID {#myh1}
===================
with a class {.myclass}
------------
with a custom attribute {myattr=myvalue}
========================================
multiple! {.myclass1 myattr #myh3 otherattr=value .myclass2}
--
"##;
    let expected = r##"<h1 id="myh1">with the ID</h1>
<h2 class="myclass">with a class</h2>
<h1 myattr="myvalue">with a custom attribute</h1>
<h2 id="myh3" class="myclass1 myclass2" myattr="" otherattr="value">multiple!</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_2() {
    let original = r##"# with the ID {#myh1}
## with a class {.myclass}
#### with a custom attribute {myattr=myvalue}
### multiple! {.myclass1 myattr #myh3 otherattr=value .myclass2}
"##;
    let expected = r##"<h1 id="myh1">with the ID</h1>
<h2 class="myclass">with a class</h2>
<h4 myattr="myvalue">with a custom attribute</h4>
<h3 id="myh3" class="myclass1 myclass2" myattr="" otherattr="value">multiple!</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_3() {
    let original = r##"# H1 # {#id1}
## H2 ## with ## multiple ## hashes ## {#id2}
### with trailing hash # ### {#id3}

#### non-attribute-block {#id4} ####
"##;
    let expected = r##"<h1 id="id1">H1</h1>
<h2 id="id2">H2 ## with ## multiple ## hashes</h2>
<h3 id="id3">with trailing hash #</h3>
<h4>non-attribute-block {#id4}</h4>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_4() {
    let original = r##"# spaces {#myid1}    
## tabs {#myid2}		
"##;
    let expected = r##"<h1 id="myid1">spaces</h1>
<h2 id="myid2">tabs</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_5() {
    let original = r##"# H1 \
nextline
"##;
    let expected = r##"<h1>H1 \</h1>
<p>nextline</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_6() {
    let original = r##"# H1 \
{#myid}

## H2 \
nextline {.class}

### H3 [link
](https://example.com/) {#myid3}
"##;
    let expected = r##"<h1>H1 \</h1>
<p>{#myid}</p>
<h2>H2 \</h2>
<p>nextline {.class}</p>
<h3>H3 [link</h3>
<p>](https://example.com/) {#myid3}</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_7() {
    let original = r##"H1
cont
{#myid}
==
"##;
    let expected = r##"<h1 id="myid">H1
cont
</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_8() {
    let original = r##"H1
{
  .class1
  .class2
}
==
"##;
    let expected = r##"<h1>H1
{
.class1
.class2
}</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_9() {
    let original = r##"# without space, not recommended{#id1}
## recommended style with spaces {#id2}
"##;
    let expected = r##"<h1 id="id1">without space, not recommended</h1>
<h2 id="id2">recommended style with spaces</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_10() {
    let original = r##"# H1 { #id1 }
## H2 {.myclass      #id2 }
### H3 {     .myclass}
"##;
    let expected = r##"<h1 id="id1">H1</h1>
<h2 id="id2" class="myclass">H2</h2>
<h3 class="myclass">H3</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_11() {
    let original = r##"# H1 {#id1.class1.class2 .class3}
## H2 {.class1#id2.class2}
"##;
    let expected = r##"<h1 id="id1.class1.class2" class="class3">H1</h1>
<h2 class="class1#id2.class2">H2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_12() {
    let original = r##"# H1 { #id1
## H2 {#id2
"##;
    let expected = r##"<h1>H1 { #id1</h1>
<h2>H2 {#id2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_13() {
    let original = r##"# H1 #id1 }
## H2 #id2}
"##;
    let expected = r##"<h1>H1 #id1 }</h1>
<h2>H2 #id2}</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_14() {
    let original = r##"# H1 { #id1 } foo
## H2 {#id2} <!-- hello -->
"##;
    let expected = r##"<h1>H1 { #id1 } foo</h1>
<h2>H2 {#id2} <!-- hello --></h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_15() {
    let original = r##"# *H1* { #id1 }
## **H2** {#id2}
### _H3_ {#id3}
#### ~~H4~~ {#id4}
##### [text](uri) {#id5}
"##;
    let expected = r##"<h1 id="id1"><em>H1</em></h1>
<h2 id="id2"><strong>H2</strong></h2>
<h3 id="id3"><em>H3</em></h3>
<h4 id="id4"><del>H4</del></h4>
<h5 id="id5"><a href="uri">text</a></h5>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_16() {
    let original = r##"# H1 {#first #second #last}
"##;
    let expected = r##"<h1 id="last">H1</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_17() {
    let original = r##"# H1 {.z .a .zz}
"##;
    let expected = r##"<h1 class="z a zz">H1</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_18() {
    let original = r##"# H1 {.a .a .a}
"##;
    let expected = r##"<h1 class="a a a">H1</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_19() {
    let original = r##"# H1 {.myclass #myid}
## H2 {.z #m .a}
"##;
    let expected = r##"<h1 id="myid" class="myclass">H1</h1>
<h2 id="m" class="z a">H2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_20() {
    let original = r##"# H1 {foo}
## H2 {#myid unknown this#is.ignored attr=value .myclass}
"##;
    let expected = r##"<h1 foo="">H1</h1>
<h2 id="myid" class="myclass" unknown="" this#is.ignored="" attr="value">H2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_21() {
    let original = r##"# Header # {myattr=value other_attr}
"##;
    let expected = r##"<h1 myattr="value" other_attr="">Header</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_22() {
    let original = r##"#### Header {#id myattr= .class1 other_attr=false}
"##;
    let expected = r##"<h4 id="id" class="class1" myattr="" other_attr="false">Header</h4>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_23() {
    let original = r##"# H1 {.foo{unknown}
## H2 {.foo{.bar}
"##;
    let expected = r##"<h1 unknown="">H1 {.foo</h1>
<h2 class="bar">H2 {.foo</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_24() {
    let original = r##"# H1 {.foo}bar}
"##;
    let expected = r##"<h1>H1 {.foo}bar}</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_25() {
    let original = r##"# H1 {<i>foo</i>}
"##;
    let expected = r##"<h1>H1 {<i>foo</i>}</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_26() {
    let original = r##"# H1 {.foo\}
"##;
    let expected = r##"<h1>H1 {.foo}</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_27() {
    let original = r##"H1 {.foo
.bar}
==
"##;
    let expected = r##"<h1>H1 {.foo
.bar}</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_28() {
    let original = r##"H1 {} {}
=====

## H2 {} {}
"##;
    let expected = r##"<h1>H1 {}</h1>
<h2>H2 {}</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_29() {
    let original = r##"## H2 {} ##
"##;
    let expected = r##"<h2>H2 {}</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_30() {
    let original = r##"# H1 {\}
## this is also ok \{\}

newline can be used for setext heading {
}
--
"##;
    let expected = r##"<h1>H1 {}</h1>
<h2>this is also ok {}</h2>
<h2>newline can be used for setext heading {
}</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_31() {
    let original = r##"# H1 \{.foo}
## H2 \\{.bar}
### stray backslash at the end is preserved \
"##;
    let expected = r##"<h1 class="foo">H1 \</h1>
<h2 class="bar">H2 \</h2>
<h3>stray backslash at the end is preserved \</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_32() {
    let original = r##"H1 \{.foo}
==
H2 \\{.bar}
--

stray backslash at the end is preserved \
--
"##;
    let expected = r##"<h1 class="foo">H1 \</h1>
<h2 class="bar">H2 \</h2>
<h2>stray backslash at the end is preserved \</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_33() {
    let original = r##"# H1 {#`code`}
## H2 {#foo__bar__baz}
### H3 {#foo**bar**baz}
"##;
    let expected = r##"<h1 id="`code`">H1</h1>
<h2 id="foo__bar__baz">H2</h2>
<h3 id="foo**bar**baz">H3</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_34() {
    let original = r##"H1 {#`code`}
==

H2-1 {#foo__bar__baz}
----

H2-2 {#foo**bar**baz}
--
"##;
    let expected = r##"<h1 id="`code`">H1</h1>
<h2 id="foo__bar__baz">H2-1</h2>
<h2 id="foo**bar**baz">H2-2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_35() {
    let original = r##"# H1 {.foo#bar}
## H2 {#foo.bar}
### H3 {.a"b'c&d}
"##;
    let expected = r##"<h1 class="foo#bar">H1</h1>
<h2 id="foo.bar">H2</h2>
<h3 class="a&quot;b&#39;c&amp;d">H3</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_36() {
    let original = r##"# H1 {#}
## H2 {.}
"##;
    let expected = r##"<h1>H1</h1>
<h2>H2</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_37() {
    let original = r##"# H1 {#foo #}
# H1 {.foo . . .bar}
"##;
    let expected = r##"<h1 id="foo">H1</h1>
<h1 class="foo bar">H1</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_38() {
    let original = r##"# {}
## {}
### {\}
#### {} {}

#{}
"##;
    let expected = r##"<h1></h1>
<h2></h2>
<h3>{}</h3>
<h4>{}</h4>
<p>#{}</p>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_39() {
    let original = r##"{}
==

\{}
--

\
--

{\}
==

{}{}
--
"##;
    let expected = r##"<h1></h1>
<h2>\</h2>
<h2>\</h2>
<h1>{}</h1>
<h2>{}</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_40() {
    let original = r##"# horizontal tab	
# horizontal tab	{#ht}
## form feed
## form feed{#ff}
### vertical tab
### vertical tab{#vt}
"##;
    let expected = r##"<h1>horizontal tab	</h1>
<h1 id="ht">horizontal tab	</h1>
<h2>form feed</h2>
<h2 id="ff">form feed</h2>
<h3>vertical tab</h3>
<h3 id="vt">vertical tab</h3>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_41() {
    let original = r##"# horizontal tab (U+000A) {#ht	.myclass}
## form feed (U+000C) {#ff.myclass}

# vertical tab (U+000B) {#vt.myclass}
"##;
    let expected = r##"<h1 id="ht" class="myclass">horizontal tab (U+000A)</h1>
<h2 id="ff" class="myclass">form feed (U+000C)</h2>
<h1 id="vt.myclass">vertical tab (U+000B)</h1>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn heading_attrs_test_42() {
    let original = r##"# EN SPACE (U+2002) {#en-space .myclass}
## IDEOGRAPHIC SPACE (U+3000) {#ideographic-space　.myclass}
"##;
    let expected = r##"<h1 id="en-space .myclass">EN SPACE (U+2002)</h1>
<h2 id="ideographic-space　.myclass">IDEOGRAPHIC SPACE (U+3000)</h2>
"##;

    test_markdown_html(original, expected, false, false, false);
}
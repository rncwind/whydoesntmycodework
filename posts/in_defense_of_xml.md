---
title: In Defense of XML
slug: indefenseofxml
published: 2024-05-30
tags: [xml, json, markups, schemas]
public: true
---

# In Defense of XML

XML often gets a bad rep these days, it's associated with Enterprise Crud Apps,
the ill-fated XHTML, Java, AJAX and SOAP.

This is not a defence of these things (XHTML was cool though), but rather a way
for me to express my sadness that JSON is now the lingua franca for
serialization in the modern tech stack.

## What XML is

XML is a markup format that is often used for serialization. XML was designed to
be Human Readable, Machine Readable, Extendable, General and importantly for the
topic of this post, _able to be validated_.

XML has a long and storied history, and is defined by the W3C. XML is rather
verbose, especially compared to "modern" Serialization and Document formats like
JSON, but it's verbosity is traded for a number of advantages, the most glaring
of which are Schemas, Namespaces and speed of parsing.

This post will focus on Schemas, and why I have grown over time to love
and appreciate XML because of them.

## Validation and Schemas

A schema is a document, which describes how other documents should be formatted,
laid out and defined. It's often best to think of Schemas as Strong Typing for
Documents.

One of the great many things that Spark Joy about XML, is that an XML document
is able to be validated against a given Schema.

as a somewhat contrived and small example, every one of my blogposts has a block
of YAML at the top of the markdown file (More about how that works
[here](https://whydoesntmycode.work/post/buildingablog)).
This YAML defines metadata about the blog post; things such as the title, when
it's set to be automatically published and if its public etc.

The YAML frontmatter for this blog post looks like this.

```YAML
---
title: In Defense of XML
slug: indefenseofxml
published: 2077-10-23
tags: [xml, json]
public: false
---
```

While for a small, homebrew project like this, YAML is acceptable, it comes with
[a mountain of issues](https://noyaml.com/), not least of which is the fact that
this metadata is entirely informally specified and stringly typed!

In contrast, if I were to write this frontmatter as XML, it would be somewhat
more verbose.

```XML
<BlogPostFrontMatter>
  <Title>In Defense of XML</Title>
  <Slug>indefenseofxml</Slug>
  <Published>2077-10-23</Published>
  <Tags>
    <Tag>xml</Tag>
    <Tag>json</Tag>
  </Tags>
  <Public>false</Public>
</BlogPostFrontMatter>
```

The major advantage here, is that It's trivial to create a valid XSD schema for
this XML document, which any and all future frontmatter documents can be checked
against, to ensure that they are compliant with the spec!

A XML schema is somewhat verbose, but the technical amongst the crowd should be
able to follow it rather easily.

A frontmatter document for a blogpost is composed of a title which is a string,
a slug which is a string, a published field which is a date, an unlimited
sequence of tags, and finally a stringy boolean for if the post is public or not.

```XML
<xs:schema attributeFormDefault="unqualified" elementFormDefault="qualified" xmlns:xs="http://www.w3.org/2001/XMLSchema">
  <xs:element name="BlogPostFrontMatter">
    <xs:complexType>
      <xs:sequence>
        <xs:element type="xs:string" name="Title"/>
        <xs:element type="xs:string" name="Slug"/>
        <xs:element type="xs:date" name="Published"/>
        <xs:element name="Tags">
          <xs:complexType>
            <xs:sequence>
              <xs:element type="xs:string" name="Tag" maxOccurs="unbounded" minOccurs="0"/>
            </xs:sequence>
          </xs:complexType>
        </xs:element>
        <xs:element type="xs:string" name="Public"/>
      </xs:sequence>
    </xs:complexType>
  </xs:element>
</xs:schema>
```

One very interesting aspect of XML schemas, which is rather "love it or hate
it", is the fact that the **ordering of elements is constrained by the schema**.
This means that the document *must* follow the order of fields defined in the
schema, or else it will not pass validation!

Take for example the following XML Document.

```XML
<BlogPostFrontMatter>
  <Title>In Defense of XML</Title>
  <Tags>
    <Tag>xml</Tag>
    <Tag>json</Tag>
  </Tags>
  <Slug>indefenseofxml</Slug>
  <Published>2077-10-23</Published>
  <Public>false</Public>
</BlogPostFrontMatter>
```

This document is invalid because the next field after Title _must_ be slug, but
in this case it is the Tag field, which violates the schema.

### Schemas as Documentation

Since Schemas are in essence type checking and struct definitions for "things on
the wire", they make for fantastic documentation for all the same reasons! By
looking at an XML schema I can learn rather quickly which fields are required,
what order they should be in, and which types they should be.

To contrast this with JSON, which is rather lax in how it can be formatted,
ordered, and parsed; I never feel particularly secure in the knowledge that any
JSON data I send will not be mangled by conversion steps.

## Schemas are very powerful

I've touched on only the most basic aspects of XML schemas, they are incredibly
powerful and versatile, able to place arbitrary constraints on data and grammar.

There are tools to add schemas to other languages like JSON, but these were
added post-hoc and as such are nowhere near as closely tied to the language as
they are with XML.

## But what about REST?!

A lot of the time when I bring up my love for JSON, there's a (understandable)
knee jerk reaction whereby people think I'm advocating for SOAP instead of REST.
This is unequivocally not the case, as I rather dispose SOAP!

REST was designed rather well, in that it does not specify the communications
format. This means it's just as valid to write a REST API which returns XML
instead of JSON, in fact as long as it's "machine readable" it's allowed!

## Conclusion

XML has a bad reputation, somewhat deservedly so. It _is_ verbose, it _is_ used
a lot by Enterprise CRUD Apps, it _is_ complex, but hopefully this post will
make you appreciate the _why_ of it instead!

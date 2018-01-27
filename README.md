# Writium

[Documentation](https://docs.rs/writium/0.1.0/writium/index.html)

Light-weight plug-in web framework for all variants of Hyper.

## Why Writium Framework?

Writium Framework is not so versatile but it does its best to fulfill most of your needs, if parts of your web apps requires:

* JSON ser/de;
* chunk-based (rather than stream-based) interaction;
* separation of duties;
* hierarchic organization.

Writium Framework works well with all web frameworks which can provide HyperRequests and accept HyperResponses, but itself is not a server to-go. It might bring you a few more codes to write, but such design allows you to separate the web engine and your API logics perfectly; it brings you flexibility you always want.

For example, after finishing your RESTful API, and you find you have to write something stream-based. Then you can add it to somewhere in your same application; you don't need to port codes to another web framework simply because it doesn't support stream-based interaction.

## Example Project

Writus is a blog server based on Writium. Take a glance at the codes and you will get how writium brings you satisfying web application development experience.

## Sister Projects

During the course of development of Writus, sister projects of writium are created:

* [writium-cache]: Cache system for writium apps.
* [writium-auth]: Authentication/Authorization interfaces for writium apps.

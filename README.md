# `http-server-rust`

A simple HTTP server implemented in Rust. This project serves as a learning exercise to build an HTTP server from scratch and learn Rust.

## HTTP

**HyperText Transfer Protocol** (HTTP) is an application-level protocol built on top of **Transmission Control Protocol** (TCP) and **Internet Protocol** (IP). It is the foundation of data communication for the World Wide Web. HTTP defines how messages are formatted and transmitted, and how web servers and browsers should respond to various commands.

HTTP allows an open-ended set of methods to be used to indicate the purpose of a request. It uses the **Uniform Resource Identifier** (URI) to identify the resource that the action is to be performed on. HTTP also allows for the transmission of arbitrary data in the body of a request or response, which can be used for various purposes such as submitting form data or sending JSON payloads.

The HTTP Protocol is _stateless_, meaning that each request from a client to a server is independent and does not require the server to retain any information about previous requests.

The HTTP Protocol is based on a request-response paradigm. A client establishes a connection with a server and sends a request to the server in the form of a request method, URI, protocol version, headers and optional message body. The server processes the request, does whatever it needs to, and responds with a status code, headers and an optional message body.

## URI

A Uniform Resource Identifier (URI) is a unique identifier that refers to a resource on the web.

The most common type of URIs are URLs (Uniform Resource Locators) that specify the location of a resource, such as a document, image, or binary data, on the web.

URIs can also be used to trigger behaviour other than fetching a resource, like opening an email client, or executing JavaScript code. 


## Reference

- [HyperText Transfer Protocol RFC](https://www.rfc-editor.org/rfc/rfc1945)
- [MDN Web Docs - URI](https://developer.mozilla.org/en-US/docs/Web/URI)
- [MDN Web Docs - URI Reference](https://developer.mozilla.org/en-US/docs/Web/URI/Reference)
- [Uniform Resource Identifier RFC](https://datatracker.ietf.org/doc/html/rfc3986)
- [RFC 1738 - Uniform Resource Locators (URL)](https://www.rfc-editor.org/rfc/rfc1738)

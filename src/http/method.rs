// ------------
// HTTP METHODS
// ------------

/// The HTTP method that describes the desired action to be performed on the identified resource.
///
/// - See https://www.rfc-editor.org/rfc/rfc1945#section-5.1.1
/// - See [RFC - Method Definitions](https://www.rfc-editor.org/rfc/rfc1945#section-8)
pub enum Method {
    /// The GET method requests a representation of the specified resource. Requests using GET should only retrieve data.
    ///
    /// If the request URI refers to a data-producing process, it is the produced data that shall be returned as the entity in the response (and not the source text of the process).
    ///
    /// The semantics of the `GET` method changes to a "conditional GET" if the request message includes an `If-Modified-Since` header field.
    /// A conditional GET method requests the identified resource only if it has not been modified since the date given by the `If-Modified-Since` header field.
    /// The conditional GET method is intended to reduce network usage by allowing cached entities to be refreshed without requiring multiple requests or transferring unnecessary data.
    GET,

    /// The `HEAD` method is identical to `GET` except that the server MUST NOT return any entity-body in the response.
    /// The meta-information contained in the HTTP headers in response to a `HEAD` should be identical to a `GET` response.
    ///
    /// This method can be used for obtaining the meta-information about the resource identified by the URI without transferring the entire entity-body itself.
    /// This method is often used for testing hypertext links for validity, accessibility, and recent modification.
    ///
    /// There is no "conditional HEAD" analog to the conditional GET; the `If-Modified-Since` header field is not applicable to a `HEAD` request.
    HEAD,

    /// The `POST` method is used to request that the destination server accept the entity enclosed in the request as a new subordinate of the identified resource.
    /// The posted entity is subordinate to that URI in the same way that a file is subordinate to a directory in a file system, or a record is a subordinate to a database.
    ///
    /// `POST` is designed to allow a uniform method to cover the following functions:
    /// - Annotation of existing resources
    /// - Posting a message to a bulletin board, newsgroup, mailing list, or similar group of articles
    /// - Providing a block of data, such as the result of submitting a form, to a data-handling process
    /// - Extending a database through an append operation
    ///
    /// The actual implementation of the `POST` is determined by the server and is usually dependent on the request URI.
    ///
    /// A successful `POST` does not require that the entity be created as a resource on the origin server or be made accessible for future reference.
    /// That is, the action performed by the `POST` may not result in a new resource being created that can be identified by a URI. In such a case,
    /// either a `200 (Ok)` or `204 (No Content)` is the appropriate response status code to indicate success.
    ///
    /// If a resource has been created on the origin server, the response should be `201 (Created)` and contain an entity which describes the status of the request and refers to the new resource.
    ///
    /// A valid `Content-Length` is required on all `HTTP/1.0 POST` requests. An `HTTP/1.0` compliant server should respond with a `400 (Bad Request)` message
    /// if it cannot determine the length of the request's message content.
    POST,

    /// The `PUT` method requests that the enclosed entity be stored under the supplied Request-URI.
    /// If the Request-URI refers to an already existing resource, the enclosed entity should be considered as a modified version of the one residing on the origin server.
    PUT,

    /// The `DELETE` method requests that the origin server delete the resource identified by the Request-URI.
    DELETE,

    /// The `OPTIONS` method represents a request for information about the communication options available on the request/response chain identified by the Request-URI.
    OPTIONS,

    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    CONNECT,

    /// The `TRACE` method performs a message loop-back test along the path to the target resource.
    TRACE,

    /// The `PATCH` method is used to apply partial modifications to a resource.
    PATCH,

    /// Any other method that is not defined in the RFC.
    Other(String),
}

impl<T: AsRef<str>> From<T> for Method {
    fn from(method: T) -> Self {
        match method.as_ref() {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            "CONNECT" => Method::CONNECT,
            "TRACE" => Method::TRACE,
            "PATCH" => Method::PATCH,
            other => Method::Other(other.to_string()),
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

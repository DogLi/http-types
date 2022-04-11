use std::fmt::{self, Debug, Display};

/// HTTP response status codes.
///
/// As defined by [rfc7231 section 6](https://tools.ietf.org/html/rfc7231#section-6).
/// [Read more](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum StatusCode {
    /// 100 Continue
    ///
    /// This interim response indicates that everything so far is OK and that
    /// the client should continue the request, or ignore the response if
    /// the request is already finished.
    Continue,

    /// 101 Switching Protocols
    ///
    /// This code is sent in response to an Upgrade request header from the
    /// client, and indicates the protocol the server is switching to.
    SwitchingProtocols,

    /// 103 Early Hints
    ///
    /// This status code is primarily intended to be used with the Link header,
    /// letting the user agent start preloading resources while the server
    /// prepares a response.
    EarlyHints,

    /// 200 Ok
    ///
    /// The request has succeeded
    Ok,

    /// 201 Created
    ///
    /// The request has succeeded and a new resource has been created as a
    /// result. This is typically the response sent after POST requests, or
    /// some PUT requests.
    Created,

    /// 202 Accepted
    ///
    /// The request has been received but not yet acted upon. It is
    /// noncommittal, since there is no way in HTTP to later send an
    /// asynchronous response indicating the outcome of the request. It is
    /// intended for cases where another process or server handles the request,
    /// or for batch processing.
    Accepted,

    /// 203 Non Authoritative Information
    ///
    /// This response code means the returned meta-information is not exactly
    /// the same as is available from the origin server, but is collected
    /// from a local or a third-party copy. This is mostly used for mirrors
    /// or backups of another resource. Except for that specific case, the
    /// "200 OK" response is preferred to this status.
    NonAuthoritativeInformation,

    /// 204 No Content
    ///
    /// There is no content to send for this request, but the headers may be
    /// useful. The user-agent may update its cached headers for this
    /// resource with the new ones.
    NoContent,

    /// 205 Reset Content
    ///
    /// Tells the user-agent to reset the document which sent this request.
    ResetContent,

    /// 206 Partial Content
    ///
    /// This response code is used when the Range header is sent from the client
    /// to request only part of a resource.
    PartialContent,

    /// 207 Multi-Status
    ///
    /// A Multi-Status response conveys information about
    /// multiple resources in situations where multiple
    /// status codes might be appropriate.
    MultiStatus,

    /// 226 Im Used
    ///
    /// The server has fulfilled a GET request for the resource, and the
    /// response is a representation of the result of one or more
    /// instance-manipulations applied to the current instance.
    ImUsed,

    /// 300 Multiple Choice
    ///
    /// The request has more than one possible response. The user-agent or user
    /// should choose one of them. (There is no standardized way of choosing
    /// one of the responses, but HTML links to the possibilities are
    /// recommended so the user can pick.)
    MultipleChoice,

    /// 301 Moved Permanently
    ///
    /// The URL of the requested resource has been changed permanently. The new
    /// URL is given in the response.
    MovedPermanently,

    /// 302 Found
    ///
    /// This response code means that the URI of requested resource has been
    /// changed temporarily. Further changes in the URI might be made in the
    /// future. Therefore, this same URI should be used by the client in
    /// future requests.
    Found,

    /// 303 See Other
    ///
    /// The server sent this response to direct the client to get the requested
    /// resource at another URI with a GET request.
    SeeOther,

    /// 304 Not Modified
    ///
    /// This is used for caching purposes. It tells the client that the response
    /// has not been modified, so the client can continue to use the same
    /// cached version of the response.
    NotModified,

    /// 307 Temporary Redirect
    ///
    /// The server sends this response to direct the client to get the requested
    /// resource at another URI with same method that was used in the prior
    /// request. This has the same semantics as the 302 Found HTTP response
    /// code, with the exception that the user agent must not change the
    /// HTTP method used: If a POST was used in the first request, a POST must
    /// be used in the second request.
    TemporaryRedirect,

    /// 308 Permanent Redirect
    ///
    /// This means that the resource is now permanently located at another URI,
    /// specified by the Location: HTTP Response header. This has the same
    /// semantics as the 301 Moved Permanently HTTP response code, with the
    /// exception that the user agent must not change the HTTP method
    /// used: If a POST was used in the first request, a POST must be used in
    /// the second request.
    PermanentRedirect,

    /// 400 Bad Request
    ///
    /// The server could not understand the request due to invalid syntax.
    BadRequest,

    /// 401 Unauthorized
    ///
    /// Although the HTTP standard specifies "unauthorized", semantically this
    /// response means "unauthenticated". That is, the client must
    /// authenticate itself to get the requested response.
    Unauthorized,

    /// 402 Payment Required
    ///
    /// This response code is reserved for future use. The initial aim for
    /// creating this code was using it for digital payment systems, however
    /// this status code is used very rarely and no standard convention
    /// exists.
    PaymentRequired,

    /// 403 Forbidden
    ///
    /// The client does not have access rights to the content; that is, it is
    /// unauthorized, so the server is refusing to give the requested
    /// resource. Unlike 401, the client's identity is known to the server.
    Forbidden,

    /// 404 Not Found
    ///
    /// The server can not find requested resource. In the browser, this means
    /// the URL is not recognized. In an API, this can also mean that the
    /// endpoint is valid but the resource itself does not exist. Servers
    /// may also send this response instead of 403 to hide the existence of
    /// a resource from an unauthorized client. This response code is probably
    /// the most famous one due to its frequent occurrence on the web.
    NotFound,

    /// 405 Method Not Allowed
    ///
    /// The request method is known by the server but has been disabled and
    /// cannot be used. For example, an API may forbid DELETE-ing a
    /// resource. The two mandatory methods, GET and HEAD, must never be
    /// disabled and should not return this error code.
    MethodNotAllowed,

    /// 406 Not Acceptable
    ///
    /// This response is sent when the web server, after performing
    /// server-driven content negotiation, doesn't find any content that
    /// conforms to the criteria given by the user agent.
    NotAcceptable,

    /// 407 Proxy Authentication Required
    ///
    /// This is similar to 401 but authentication is needed to be done by a
    /// proxy.
    ProxyAuthenticationRequired,

    /// 408 Request Timeout
    ///
    /// This response is sent on an idle connection by some servers, even
    /// without any previous request by the client. It means that the server
    /// would like to shut down this unused connection. This response is
    /// used much more since some browsers, like Chrome, Firefox 27+,
    /// or IE9, use HTTP pre-connection mechanisms to speed up surfing. Also
    /// note that some servers merely shut down the connection without
    /// sending this message.
    RequestTimeout,

    /// 409 Conflict
    ///
    /// This response is sent when a request conflicts with the current state of
    /// the server.
    Conflict,

    /// 410 Gone
    ///
    /// This response is sent when the requested content has been permanently
    /// deleted from server, with no forwarding address. Clients are
    /// expected to remove their caches and links to the resource. The HTTP
    /// specification intends this status code to be used for "limited-time,
    /// promotional services". APIs should not feel compelled to indicate
    /// resources that have been deleted with this status code.
    Gone,

    /// 411 Length Required
    ///
    /// Server rejected the request because the Content-Length header field is
    /// not defined and the server requires it.
    LengthRequired,

    /// 412 Precondition Failed
    ///
    /// The client has indicated preconditions in its headers which the server
    /// does not meet.
    PreconditionFailed,

    /// 413 Payload Too Large
    ///
    /// Request entity is larger than limits defined by server; the server might
    /// close the connection or return an Retry-After header field.
    PayloadTooLarge,

    /// 414 URI Too Long
    ///
    /// The URI requested by the client is longer than the server is willing to
    /// interpret.
    UriTooLong,

    /// 415 Unsupported Media Type
    ///
    /// The media format of the requested data is not supported by the server,
    /// so the server is rejecting the request.
    UnsupportedMediaType,

    /// 416 Requested Range Not Satisfiable
    ///
    /// The range specified by the Range header field in the request can't be
    /// fulfilled; it's possible that the range is outside the size of the
    /// target URI's data.
    RequestedRangeNotSatisfiable,

    /// 417 Expectation Failed
    ///
    /// This response code means the expectation indicated by the Expect request
    /// header field can't be met by the server.
    ExpectationFailed,
    ///
    /// 418 I'm a teapot
    ///
    /// The server refuses the attempt to brew coffee with a teapot.
    ImATeapot,

    /// 421 Misdirected Request
    ///
    /// The request was directed at a server that is not able to produce a
    /// response. This can be sent by a server that is not configured to
    /// produce responses for the combination of scheme and authority that
    /// are included in the request URI.
    MisdirectedRequest,

    /// 422 Unprocessable Entity
    ///
    /// The request was well-formed but was unable to be followed due to
    /// semantic errors.
    UnprocessableEntity,

    /// 423 Locked
    ///
    /// The resource that is being accessed is locked.
    Locked,

    /// 424 Failed Dependency
    ///
    /// The request failed because it depended on another request and that
    /// request failed (e.g., a PROPPATCH).
    FailedDependency,

    /// 425 Too Early
    ///
    /// Indicates that the server is unwilling to risk processing a request that
    /// might be replayed.
    TooEarly,

    /// 426 Upgrade Required
    ///
    /// The server refuses to perform the request using the current protocol but
    /// might be willing to do so after the client upgrades to a different
    /// protocol. The server sends an Upgrade header in a 426 response to
    /// indicate the required protocol(s).
    UpgradeRequired,

    /// 428 Precondition Required
    ///
    /// The origin server requires the request to be conditional. This response
    /// is intended to prevent the 'lost update' problem, where a client
    /// GETs a resource's state, modifies it, and PUTs it back to the
    /// server, when meanwhile a third party has modified the state on the
    /// server, leading to a conflict.
    PreconditionRequired,

    /// 429 Too Many Requests
    ///
    /// The user has sent too many requests in a given amount of time ("rate
    /// limiting").
    TooManyRequests,

    /// 431 Request Header Fields Too Large
    ///
    /// The server is unwilling to process the request because its header fields
    /// are too large. The request may be resubmitted after reducing the
    /// size of the request header fields.
    RequestHeaderFieldsTooLarge,

    /// 451 Unavailable For Legal Reasons
    ///
    /// The user-agent requested a resource that cannot legally be provided,
    /// such as a web page censored by a government.
    UnavailableForLegalReasons,

    /// 500 Internal Server Error
    ///
    /// The server has encountered a situation it doesn't know how to handle.
    InternalServerError,

    /// 501 Not Implemented
    ///
    /// The request method is not supported by the server and cannot be handled.
    /// The only methods that servers are required to support (and therefore
    /// that must not return this code) are GET and HEAD.
    NotImplemented,

    /// 502 Bad Gateway
    ///
    /// This error response means that the server, while working as a gateway to
    /// get a response needed to handle the request, got an invalid
    /// response.
    BadGateway,

    /// 503 Service Unavailable
    ///
    /// The server is not ready to handle the request. Common causes are a
    /// server that is down for maintenance or that is overloaded. Note that
    /// together with this response, a user-friendly page explaining the
    /// problem should be sent. This responses should be used for temporary
    /// conditions and the Retry-After: HTTP header should, if possible, contain
    /// the estimated time before the recovery of the service. The webmaster
    /// must also take care about the caching-related headers that are sent
    /// along with this response, as these temporary condition responses
    /// should usually not be cached.
    ServiceUnavailable,

    /// 504 Gateway Timeout
    ///
    /// This error response is given when the server is acting as a gateway and
    /// cannot get a response in time.
    GatewayTimeout,

    /// 505 HTTP Version Not Supported
    ///
    /// The HTTP version used in the request is not supported by the server.
    HttpVersionNotSupported,

    /// 506 Variant Also Negotiates
    ///
    /// The server has an internal configuration error: the chosen variant
    /// resource is configured to engage in transparent content negotiation
    /// itself, and is therefore not a proper end point in the negotiation
    /// process.
    VariantAlsoNegotiates,

    /// 507 Insufficient Storage
    ///
    /// The server is unable to store the representation needed to complete the
    /// request.
    InsufficientStorage,

    /// 508 Loop Detected
    ///
    /// The server detected an infinite loop while processing the request.
    LoopDetected,

    /// 510 Not Extended
    ///
    /// Further extensions to the request are required for the server to fulfil
    /// it.
    NotExtended,

    /// 511 Network Authentication Required
    ///
    /// The 511 status code indicates that the client needs to authenticate to
    /// gain network access.
    NetworkAuthenticationRequired,

    /// Other Invalid response
    OtherInvalidStatusCode(u16),
}

impl StatusCode {
    /// Returns `true` if the status code is `1xx` range.
    ///
    /// If this returns `true` it indicates that the request was received,
    /// continuing process.
    pub fn is_informational(&self) -> bool {
        let num: u16 = (*self).into();
        (100..200).contains(&num)
    }

    /// Returns `true` if the status code is the `2xx` range.
    ///
    /// If this returns `true` it indicates that the request was successfully
    /// received, understood, and accepted.
    pub fn is_success(&self) -> bool {
        let num: u16 = (*self).into();
        (200..300).contains(&num)
    }

    /// Returns `true` if the status code is the `3xx` range.
    ///
    /// If this returns `true` it indicates that further action needs to be
    /// taken in order to complete the request.
    pub fn is_redirection(&self) -> bool {
        let num: u16 = (*self).into();
        (300..400).contains(&num)
    }

    /// Returns `true` if the status code is the `4xx` range.
    ///
    /// If this returns `true` it indicates that the request contains bad syntax
    /// or cannot be fulfilled.
    pub fn is_client_error(&self) -> bool {
        let num: u16 = (*self).into();
        (400..500).contains(&num)
    }

    /// Returns `true` if the status code is the `5xx` range.
    ///
    /// If this returns `true` it indicates that the server failed to fulfill an
    /// apparently valid request.
    pub fn is_server_error(&self) -> bool {
        let num: u16 = (*self).into();
        (500..600).contains(&num)
    }

    /// The canonical reason for a given status code
    pub fn canonical_reason(&self) -> String {
        match self {
            StatusCode::Continue => "Continue".into(),
            StatusCode::SwitchingProtocols => "Switching Protocols".into(),
            StatusCode::EarlyHints => "Early Hints".into(),
            StatusCode::Ok => "OK".into(),
            StatusCode::Created => "Created".into(),
            StatusCode::Accepted => "Accepted".into(),
            StatusCode::NonAuthoritativeInformation => "Non Authoritative Information".into(),
            StatusCode::NoContent => "No Content".into(),
            StatusCode::ResetContent => "Reset Content".into(),
            StatusCode::PartialContent => "Partial Content".into(),
            StatusCode::MultiStatus => "Multi-Status".into(),
            StatusCode::ImUsed => "Im Used".into(),
            StatusCode::MultipleChoice => "Multiple Choice".into(),
            StatusCode::MovedPermanently => "Moved Permanently".into(),
            StatusCode::Found => "Found".into(),
            StatusCode::SeeOther => "See Other".into(),
            StatusCode::NotModified => "Not Modified".into(),
            StatusCode::TemporaryRedirect => "Temporary Redirect".into(),
            StatusCode::PermanentRedirect => "Permanent Redirect".into(),
            StatusCode::BadRequest => "Bad Request".into(),
            StatusCode::Unauthorized => "Unauthorized".into(),
            StatusCode::PaymentRequired => "Payment Required".into(),
            StatusCode::Forbidden => "Forbidden".into(),
            StatusCode::NotFound => "Not Found".into(),
            StatusCode::MethodNotAllowed => "Method Not Allowed".into(),
            StatusCode::NotAcceptable => "Not Acceptable".into(),
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required".into(),
            StatusCode::RequestTimeout => "Request Timeout".into(),
            StatusCode::Conflict => "Conflict".into(),
            StatusCode::Gone => "Gone".into(),
            StatusCode::LengthRequired => "Length Required".into(),
            StatusCode::PreconditionFailed => "Precondition Failed".into(),
            StatusCode::PayloadTooLarge => "Payload Too Large".into(),
            StatusCode::UriTooLong => "URI Too Long".into(),
            StatusCode::UnsupportedMediaType => "Unsupported Media Type".into(),
            StatusCode::RequestedRangeNotSatisfiable => "Requested Range Not Satisfiable".into(),
            StatusCode::ExpectationFailed => "Expectation Failed".into(),
            StatusCode::ImATeapot => "I'm a teapot".into(),
            StatusCode::MisdirectedRequest => "Misdirected Request".into(),
            StatusCode::UnprocessableEntity => "Unprocessable Entity".into(),
            StatusCode::Locked => "Locked".into(),
            StatusCode::FailedDependency => "Failed Dependency".into(),
            StatusCode::TooEarly => "Too Early".into(),
            StatusCode::UpgradeRequired => "Upgrade Required".into(),
            StatusCode::PreconditionRequired => "Precondition Required".into(),
            StatusCode::TooManyRequests => "Too Many Requests".into(),
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large".into(),
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons".into(),
            StatusCode::InternalServerError => "Internal Server Error".into(),
            StatusCode::NotImplemented => "Not Implemented".into(),
            StatusCode::BadGateway => "Bad Gateway".into(),
            StatusCode::ServiceUnavailable => "Service Unavailable".into(),
            StatusCode::GatewayTimeout => "Gateway Timeout".into(),
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported".into(),
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates".into(),
            StatusCode::InsufficientStorage => "Insufficient Storage".into(),
            StatusCode::LoopDetected => "Loop Detected".into(),
            StatusCode::NotExtended => "Not Extended".into(),
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required".into(),
            StatusCode::OtherInvalidStatusCode(code) => {
                format!("Other invalid status with code {}", code)
            }
        }
    }

    /// get the code of status
    pub fn code(&self) -> u16 {
        (*self).into()
    }
}

#[cfg(feature = "serde")]
mod serde {
    use super::StatusCode;
    use serde_crate::de::{Error as DeError, Visitor};
    use serde_crate::{Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    impl Serialize for StatusCode {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let value: u16 = (*self).into();
            serializer.serialize_u16(value)
        }
    }

    struct StatusCodeU16Visitor;

    impl<'de> Visitor<'de> for StatusCodeU16Visitor {
        type Value = StatusCode;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a u16 representing the status code")
        }

        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            let status = StatusCode::from(v);
            Ok(status)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: DeError,
        {
            self.visit_u16(v as u16)
        }
    }

    impl<'de> Deserialize<'de> for StatusCode {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(StatusCodeU16Visitor)
        }
    }
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        match self {
            Self::Continue => 100,
            Self::SwitchingProtocols => 101,
            Self::EarlyHints => 103,
            Self::Ok => 200,
            Self::Created => 201,
            Self::Accepted => 202,
            Self::NonAuthoritativeInformation => 203,
            Self::NoContent => 204,
            Self::ResetContent => 205,
            Self::PartialContent => 206,
            Self::MultiStatus => 207,
            Self::ImUsed => 226,
            Self::MultipleChoice => 300,
            Self::MovedPermanently => 301,
            Self::Found => 302,
            Self::SeeOther => 303,
            Self::NotModified => 304,
            Self::TemporaryRedirect => 307,
            Self::PermanentRedirect => 308,
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::PaymentRequired => 402,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::MethodNotAllowed => 405,
            Self::NotAcceptable => 406,
            Self::ProxyAuthenticationRequired => 407,
            Self::RequestTimeout => 408,
            Self::Conflict => 409,
            Self::Gone => 410,
            Self::LengthRequired => 411,
            Self::PreconditionFailed => 412,
            Self::PayloadTooLarge => 413,
            Self::UriTooLong => 414,
            Self::UnsupportedMediaType => 415,
            Self::RequestedRangeNotSatisfiable => 416,
            Self::ExpectationFailed => 417,
            Self::ImATeapot => 418,
            Self::MisdirectedRequest => 421,
            Self::UnprocessableEntity => 422,
            Self::Locked => 423,
            Self::FailedDependency => 424,
            Self::TooEarly => 425,
            Self::UpgradeRequired => 426,
            Self::PreconditionRequired => 428,
            Self::TooManyRequests => 429,
            Self::RequestHeaderFieldsTooLarge => 431,
            Self::UnavailableForLegalReasons => 451,
            Self::InternalServerError => 500,
            Self::NotImplemented => 501,
            Self::BadGateway => 502,
            Self::ServiceUnavailable => 503,
            Self::GatewayTimeout => 504,
            Self::HttpVersionNotSupported => 505,
            Self::VariantAlsoNegotiates => 506,
            Self::InsufficientStorage => 507,
            Self::LoopDetected => 508,
            Self::NotExtended => 510,
            Self::NetworkAuthenticationRequired => 511,
            Self::OtherInvalidStatusCode(code) => code,
        }
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            100 => Self::Continue,
            101 => Self::SwitchingProtocols,
            103 => Self::EarlyHints,
            200 => Self::Ok,
            201 => Self::Created,
            202 => Self::Accepted,
            203 => Self::NonAuthoritativeInformation,
            204 => Self::NoContent,
            205 => Self::ResetContent,
            206 => Self::PartialContent,
            207 => Self::MultiStatus,
            226 => Self::ImUsed,
            300 => Self::MultipleChoice,
            301 => Self::MovedPermanently,
            302 => Self::Found,
            303 => Self::SeeOther,
            304 => Self::NotModified,
            307 => Self::TemporaryRedirect,
            308 => Self::PermanentRedirect,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            402 => Self::PaymentRequired,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            405 => Self::MethodNotAllowed,
            406 => Self::NotAcceptable,
            407 => Self::ProxyAuthenticationRequired,
            408 => Self::RequestTimeout,
            409 => Self::Conflict,
            410 => Self::Gone,
            411 => Self::LengthRequired,
            412 => Self::PreconditionFailed,
            413 => Self::PayloadTooLarge,
            414 => Self::UriTooLong,
            415 => Self::UnsupportedMediaType,
            416 => Self::RequestedRangeNotSatisfiable,
            417 => Self::ExpectationFailed,
            418 => Self::ImATeapot,
            421 => Self::MisdirectedRequest,
            422 => Self::UnprocessableEntity,
            423 => Self::Locked,
            424 => Self::FailedDependency,
            425 => Self::TooEarly,
            426 => Self::UpgradeRequired,
            428 => Self::PreconditionRequired,
            429 => Self::TooManyRequests,
            431 => Self::RequestHeaderFieldsTooLarge,
            451 => Self::UnavailableForLegalReasons,
            500 => Self::InternalServerError,
            501 => Self::NotImplemented,
            502 => Self::BadGateway,
            503 => Self::ServiceUnavailable,
            504 => Self::GatewayTimeout,
            505 => Self::HttpVersionNotSupported,
            506 => Self::VariantAlsoNegotiates,
            507 => Self::InsufficientStorage,
            508 => Self::LoopDetected,
            510 => Self::NotExtended,
            511 => Self::NetworkAuthenticationRequired,
            code => Self::OtherInvalidStatusCode(code),
        }
    }
}

impl PartialEq<StatusCode> for u16 {
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.code()
    }
}

impl PartialEq<u16> for StatusCode {
    fn eq(&self, other: &u16) -> bool {
        self.code() == *other
    }
}

impl Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.canonical_reason())
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code(), self.canonical_reason())
    }
}

#[cfg(test)]
mod test {
    use super::StatusCode;
    #[test]
    fn serde_as_u16() -> Result<(), serde_json::Error> {
        let status_code: StatusCode = serde_json::from_str("202")?;
        assert_eq!(StatusCode::Accepted, status_code);
        assert_eq!(
            Some(202),
            serde_json::to_value(&StatusCode::Accepted)?.as_u64()
        );
        Ok(())
    }
}

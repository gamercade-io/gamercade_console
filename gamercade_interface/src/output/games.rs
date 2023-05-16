#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameInfoRequest {
    #[prost(uint32, tag = "1")]
    pub game_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GamesInfoRequest {
    #[prost(uint32, repeated, tag = "1")]
    pub game_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GamesInfoResponse {
    #[prost(message, repeated, tag = "1")]
    pub games_info: ::prost::alloc::vec::Vec<GameInfoBasic>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameInfoBasic {
    #[prost(uint32, tag = "1")]
    pub game_id: u32,
    #[prost(uint32, tag = "2")]
    pub author_id: u32,
    #[prost(string, tag = "3")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub short_description: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub size: u32,
    #[prost(enumeration = "Tags", repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameInfoDetailed {
    #[prost(message, optional, tag = "1")]
    pub basic_info: ::core::option::Option<GameInfoBasic>,
    #[prost(string, tag = "2")]
    pub long_description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub game_version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub release_date: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub last_updated: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Tags {
    Unknown = 0,
    /// Player Count
    Singleplayer = 1,
    Multiplayer = 2,
    Localmultiplayer = 3,
    /// Play Style
    Pvp = 4,
    Pve = 5,
    Coop = 6,
    Competitive = 7,
    Asymmetric = 8,
    /// Genre
    Action = 9,
    Platformer = 10,
    Shooter = 11,
    Fighting = 12,
    Puzzle = 13,
    Rpg = 14,
    Strategy = 15,
    Driving = 16,
    Sports = 17,
    Turnbased = 18,
    Simulation = 19,
    /// Controls
    Mouse = 20,
    Keyboard = 21,
    Controller = 22,
    /// Feel
    Fantasy = 23,
    Scifi = 24,
    Historical = 25,
    Horror = 26,
    Funny = 27,
    Cute = 28,
    Casual = 29,
}
impl Tags {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Tags::Unknown => "UNKNOWN",
            Tags::Singleplayer => "SINGLEPLAYER",
            Tags::Multiplayer => "MULTIPLAYER",
            Tags::Localmultiplayer => "LOCALMULTIPLAYER",
            Tags::Pvp => "PVP",
            Tags::Pve => "PVE",
            Tags::Coop => "COOP",
            Tags::Competitive => "COMPETITIVE",
            Tags::Asymmetric => "ASYMMETRIC",
            Tags::Action => "ACTION",
            Tags::Platformer => "PLATFORMER",
            Tags::Shooter => "SHOOTER",
            Tags::Fighting => "FIGHTING",
            Tags::Puzzle => "PUZZLE",
            Tags::Rpg => "RPG",
            Tags::Strategy => "STRATEGY",
            Tags::Driving => "DRIVING",
            Tags::Sports => "SPORTS",
            Tags::Turnbased => "TURNBASED",
            Tags::Simulation => "SIMULATION",
            Tags::Mouse => "MOUSE",
            Tags::Keyboard => "KEYBOARD",
            Tags::Controller => "CONTROLLER",
            Tags::Fantasy => "FANTASY",
            Tags::Scifi => "SCIFI",
            Tags::Historical => "HISTORICAL",
            Tags::Horror => "HORROR",
            Tags::Funny => "FUNNY",
            Tags::Cute => "CUTE",
            Tags::Casual => "CASUAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "SINGLEPLAYER" => Some(Self::Singleplayer),
            "MULTIPLAYER" => Some(Self::Multiplayer),
            "LOCALMULTIPLAYER" => Some(Self::Localmultiplayer),
            "PVP" => Some(Self::Pvp),
            "PVE" => Some(Self::Pve),
            "COOP" => Some(Self::Coop),
            "COMPETITIVE" => Some(Self::Competitive),
            "ASYMMETRIC" => Some(Self::Asymmetric),
            "ACTION" => Some(Self::Action),
            "PLATFORMER" => Some(Self::Platformer),
            "SHOOTER" => Some(Self::Shooter),
            "FIGHTING" => Some(Self::Fighting),
            "PUZZLE" => Some(Self::Puzzle),
            "RPG" => Some(Self::Rpg),
            "STRATEGY" => Some(Self::Strategy),
            "DRIVING" => Some(Self::Driving),
            "SPORTS" => Some(Self::Sports),
            "TURNBASED" => Some(Self::Turnbased),
            "SIMULATION" => Some(Self::Simulation),
            "MOUSE" => Some(Self::Mouse),
            "KEYBOARD" => Some(Self::Keyboard),
            "CONTROLLER" => Some(Self::Controller),
            "FANTASY" => Some(Self::Fantasy),
            "SCIFI" => Some(Self::Scifi),
            "HISTORICAL" => Some(Self::Historical),
            "HORROR" => Some(Self::Horror),
            "FUNNY" => Some(Self::Funny),
            "CUTE" => Some(Self::Cute),
            "CASUAL" => Some(Self::Casual),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod games_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GamesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GamesServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GamesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GamesServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GamesServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn get_game_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GameInfoRequest>,
        ) -> Result<tonic::Response<super::GameInfoBasic>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/games.GamesService/GetGameInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_games_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GamesInfoRequest>,
        ) -> Result<tonic::Response<super::GamesInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/games.GamesService/GetGamesInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_game_detailed_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GameInfoRequest>,
        ) -> Result<tonic::Response<super::GameInfoDetailed>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/games.GamesService/GetGameDetailedInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod games_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GamesServiceServer.
    #[async_trait]
    pub trait GamesService: Send + Sync + 'static {
        async fn get_game_info(
            &self,
            request: tonic::Request<super::GameInfoRequest>,
        ) -> Result<tonic::Response<super::GameInfoBasic>, tonic::Status>;
        async fn get_games_info(
            &self,
            request: tonic::Request<super::GamesInfoRequest>,
        ) -> Result<tonic::Response<super::GamesInfoResponse>, tonic::Status>;
        async fn get_game_detailed_info(
            &self,
            request: tonic::Request<super::GameInfoRequest>,
        ) -> Result<tonic::Response<super::GameInfoDetailed>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct GamesServiceServer<T: GamesService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GamesService> GamesServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GamesServiceServer<T>
    where
        T: GamesService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/games.GamesService/GetGameInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameInfoSvc<T: GamesService>(pub Arc<T>);
                    impl<
                        T: GamesService,
                    > tonic::server::UnaryService<super::GameInfoRequest>
                    for GetGameInfoSvc<T> {
                        type Response = super::GameInfoBasic;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GameInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_game_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGameInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/games.GamesService/GetGamesInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetGamesInfoSvc<T: GamesService>(pub Arc<T>);
                    impl<
                        T: GamesService,
                    > tonic::server::UnaryService<super::GamesInfoRequest>
                    for GetGamesInfoSvc<T> {
                        type Response = super::GamesInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GamesInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_games_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGamesInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/games.GamesService/GetGameDetailedInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameDetailedInfoSvc<T: GamesService>(pub Arc<T>);
                    impl<
                        T: GamesService,
                    > tonic::server::UnaryService<super::GameInfoRequest>
                    for GetGameDetailedInfoSvc<T> {
                        type Response = super::GameInfoDetailed;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GameInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_game_detailed_info(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGameDetailedInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: GamesService> Clone for GamesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GamesService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GamesService> tonic::server::NamedService for GamesServiceServer<T> {
        const NAME: &'static str = "games.GamesService";
    }
}

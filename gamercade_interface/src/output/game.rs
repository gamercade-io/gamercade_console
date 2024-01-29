#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameInfoRequest {
    #[prost(uint32, tag = "1")]
    pub game_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultipleGamesInfoRequest {
    #[prost(uint32, repeated, tag = "1")]
    pub game_ids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultipleGamesInfoResponse {
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
    #[prost(uint32, tag = "3")]
    pub hash: u32,
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
pub mod game_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GameServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GameServiceClient<T>
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
        ) -> GameServiceClient<InterceptedService<T, F>>
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
            GameServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn get_single_game_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GameInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::GameInfoBasic>, tonic::Status> {
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
                "/game.GameService/GetSingleGameInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("game.GameService", "GetSingleGameInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_multiple_games_info(
            &mut self,
            request: impl tonic::IntoRequest<super::MultipleGamesInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultipleGamesInfoResponse>,
            tonic::Status,
        > {
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
                "/game.GameService/GetMultipleGamesInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("game.GameService", "GetMultipleGamesInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_game_detailed_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GameInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GameInfoDetailed>,
            tonic::Status,
        > {
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
                "/game.GameService/GetGameDetailedInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("game.GameService", "GetGameDetailedInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod game_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GameServiceServer.
    #[async_trait]
    pub trait GameService: Send + Sync + 'static {
        async fn get_single_game_info(
            &self,
            request: tonic::Request<super::GameInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::GameInfoBasic>, tonic::Status>;
        async fn get_multiple_games_info(
            &self,
            request: tonic::Request<super::MultipleGamesInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MultipleGamesInfoResponse>,
            tonic::Status,
        >;
        async fn get_game_detailed_info(
            &self,
            request: tonic::Request<super::GameInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GameInfoDetailed>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GameServiceServer<T: GameService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GameService> GameServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GameServiceServer<T>
    where
        T: GameService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/game.GameService/GetSingleGameInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetSingleGameInfoSvc<T: GameService>(pub Arc<T>);
                    impl<
                        T: GameService,
                    > tonic::server::UnaryService<super::GameInfoRequest>
                    for GetSingleGameInfoSvc<T> {
                        type Response = super::GameInfoBasic;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GameInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GameService>::get_single_game_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSingleGameInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/game.GameService/GetMultipleGamesInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetMultipleGamesInfoSvc<T: GameService>(pub Arc<T>);
                    impl<
                        T: GameService,
                    > tonic::server::UnaryService<super::MultipleGamesInfoRequest>
                    for GetMultipleGamesInfoSvc<T> {
                        type Response = super::MultipleGamesInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MultipleGamesInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GameService>::get_multiple_games_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMultipleGamesInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/game.GameService/GetGameDetailedInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameDetailedInfoSvc<T: GameService>(pub Arc<T>);
                    impl<
                        T: GameService,
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as GameService>::get_game_detailed_info(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGameDetailedInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: GameService> Clone for GameServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: GameService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameService> tonic::server::NamedService for GameServiceServer<T> {
        const NAME: &'static str = "game.GameService";
    }
}

/// / Scroll Request
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrollRequest {
    /// / The `scroll_id` is the given in the response of a search request including a scroll.
    #[prost(string, tag = "1")]
    pub scroll_id: ::prost::alloc::string::String,
    #[prost(uint32, optional, tag = "2")]
    pub scroll_ttl_secs: ::core::option::Option<u32>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutKvRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub ttl_secs: u32,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutKvResponse {}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKvRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKvResponse {
    #[prost(bytes = "vec", optional, tag = "1")]
    pub payload: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSplit {
    /// Split id (ULID format `01HAV29D4XY3D462FS3D8K5Q2H`)
    #[prost(string, tag = "2")]
    pub split_id: ::prost::alloc::string::String,
    /// The storage uri. This URI does NOT include the split id.
    #[prost(string, tag = "1")]
    pub storage_uri: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSplitsRequest {
    #[prost(message, repeated, tag = "1")]
    pub report_splits: ::prost::alloc::vec::Vec<ReportSplit>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSplitsResponse {}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[derive(Eq, Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Index ID patterns
    #[prost(string, repeated, tag = "1")]
    pub index_id_patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Json object representing Quickwit's QueryAst.
    #[prost(string, tag = "13")]
    pub query_ast: ::prost::alloc::string::String,
    /// Time filter, expressed in seconds since epoch.
    /// That filter is to be interpreted as the semi-open interval:
    /// [start_timestamp, end_timestamp).
    #[prost(int64, optional, tag = "4")]
    pub start_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub end_timestamp: ::core::option::Option<i64>,
    /// Maximum number of hits to return.
    #[prost(uint64, tag = "6")]
    pub max_hits: u64,
    /// First hit to return. Together with max_hits, this parameter
    /// can be used for pagination.
    ///
    /// E.g.
    /// The results with rank [start_offset..start_offset + max_hits) are returned.
    #[prost(uint64, tag = "7")]
    pub start_offset: u64,
    /// json serialized aggregation_request
    #[prost(string, optional, tag = "11")]
    pub aggregation_request: ::core::option::Option<::prost::alloc::string::String>,
    /// Fields to extract snippet on
    #[prost(string, repeated, tag = "12")]
    pub snippet_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional sort by one or more fields (limited to 2 at the moment).
    #[prost(message, repeated, tag = "14")]
    pub sort_fields: ::prost::alloc::vec::Vec<SortField>,
    /// If set, the search response will include a search id
    /// that will make it possible to paginate through the results
    /// in a consistent manner.
    #[prost(uint32, optional, tag = "15")]
    pub scroll_ttl_secs: ::core::option::Option<u32>,
    /// Document with sort tuple smaller or equal to this are discarded to
    /// enable pagination.
    /// If split_id is empty, no comparison with _shard_doc should be done
    #[prost(message, optional, tag = "16")]
    pub search_after: ::core::option::Option<PartialHit>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[derive(Eq, Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortField {
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    #[prost(enumeration = "SortOrder", tag = "2")]
    pub sort_order: i32,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// Number of hits matching the query.
    #[prost(uint64, tag = "1")]
    pub num_hits: u64,
    /// Matched hits
    #[prost(message, repeated, tag = "2")]
    pub hits: ::prost::alloc::vec::Vec<Hit>,
    /// Elapsed time to perform the request. This time is measured
    /// server-side and expressed in microseconds.
    #[prost(uint64, tag = "3")]
    pub elapsed_time_micros: u64,
    /// The searcherrors that occurred formatted as string.
    #[prost(string, repeated, tag = "4")]
    pub errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Serialized aggregation response
    #[prost(string, optional, tag = "5")]
    pub aggregation: ::core::option::Option<::prost::alloc::string::String>,
    /// Scroll Id (only set if scroll_secs was set in the request)
    #[prost(string, optional, tag = "6")]
    pub scroll_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitSearchError {
    /// The searcherror that occurred formatted as string.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// Split id that failed.
    #[prost(string, tag = "2")]
    pub split_id: ::prost::alloc::string::String,
    /// Flag to indicate if the error can be considered a retryable error
    #[prost(bool, tag = "3")]
    pub retryable_error: bool,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafSearchRequest {
    /// Search request. This is a perfect copy of the original search request,
    /// that was sent to root apart from the start_offset & max_hits params.
    #[prost(message, optional, tag = "1")]
    pub search_request: ::core::option::Option<SearchRequest>,
    /// Index split ids to apply the query on.
    /// This ids are resolved from the index_uri defined in the search_request.
    #[prost(message, repeated, tag = "4")]
    pub split_offsets: ::prost::alloc::vec::Vec<SplitIdAndFooterOffsets>,
    /// `DocMapper` as json serialized trait.
    #[prost(string, tag = "5")]
    pub doc_mapper: ::prost::alloc::string::String,
    /// Index URI. The index URI defines the location of the storage that contains the
    /// split files.
    #[prost(string, tag = "6")]
    pub index_uri: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitIdAndFooterOffsets {
    /// Index split id to apply the query on.
    /// This id is resolved from the index_uri defined in the search_request.
    #[prost(string, tag = "1")]
    pub split_id: ::prost::alloc::string::String,
    /// The offset of the start of footer in the split bundle. The footer contains the file bundle metadata and the hotcache.
    #[prost(uint64, tag = "2")]
    pub split_footer_start: u64,
    /// The offset of the end of the footer in split bundle. The footer contains the file bundle metadata and the hotcache.
    #[prost(uint64, tag = "3")]
    pub split_footer_end: u64,
    /// The lowest timestamp appearing in the split
    #[prost(int64, optional, tag = "4")]
    pub timestamp_start: ::core::option::Option<i64>,
    /// The highest timestamp appearing in the split
    #[prost(int64, optional, tag = "5")]
    pub timestamp_end: ::core::option::Option<i64>,
}
/// Hits returned by a FetchDocRequest.
///
/// The json that is joined is the raw tantivy json doc.
/// It is very different from a quickwit json doc.
///
/// For instance:
/// - it may contain a _source and a _dynamic field.
/// - since tantivy has no notion of cardinality,
/// all fields is  are arrays.
/// - since tantivy has no notion of object, the object is
/// flattened by concatenating the path to the root.
///
/// See  `quickwit_search::convert_leaf_hit`
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafHit {
    /// The actual content of the hit/
    #[prost(string, tag = "1")]
    pub leaf_json: ::prost::alloc::string::String,
    /// The partial hit (ie: the sorting field + the document address)
    #[prost(message, optional, tag = "2")]
    pub partial_hit: ::core::option::Option<PartialHit>,
    /// A snippet of the matching content
    #[prost(string, optional, tag = "3")]
    pub leaf_snippet_json: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hit {
    /// The actual content of the hit/
    #[prost(string, tag = "1")]
    pub json: ::prost::alloc::string::String,
    /// The partial hit (ie: the sorting field + the document address)
    #[prost(message, optional, tag = "2")]
    pub partial_hit: ::core::option::Option<PartialHit>,
    /// A snippet of the matching content
    #[prost(string, optional, tag = "3")]
    pub snippet: ::core::option::Option<::prost::alloc::string::String>,
}
/// A partial hit, is a hit for which we have not fetch the content yet.
/// Instead, it holds a document_uri which is enough information to
/// go and fetch the actual document data, by performing a `get_doc(...)`
/// request.
///
/// Value of the sorting key for the given document.
///
/// Quickwit only computes top-K of this sorting field.
/// If the user requested for a bottom-K of a given fast field, then quickwit simply
/// emits an decreasing mapping of this fast field.
///
/// In case of a tie, quickwit uses the increasing order of
/// - the split_id,
/// - the segment_ord,
/// - the doc id.
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[derive(Eq, Hash)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialHit {
    #[prost(message, optional, tag = "10")]
    pub sort_value: ::core::option::Option<SortByValue>,
    #[prost(message, optional, tag = "11")]
    pub sort_value2: ::core::option::Option<SortByValue>,
    #[prost(string, tag = "2")]
    pub split_id: ::prost::alloc::string::String,
    /// (segment_ord, doc) form a tantivy DocAddress, which is sufficient to identify a document
    /// within a split
    #[prost(uint32, tag = "3")]
    pub segment_ord: u32,
    /// The DocId identifies a unique document at the scale of a tantivy segment.
    #[prost(uint32, tag = "4")]
    pub doc_id: u32,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[derive(Ord, PartialOrd)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SortByValue {
    #[prost(oneof = "sort_by_value::SortValue", tags = "1, 2, 3, 4")]
    pub sort_value: ::core::option::Option<sort_by_value::SortValue>,
}
/// Nested message and enum types in `SortByValue`.
pub mod sort_by_value {
    #[derive(Serialize, Deserialize, utoipa::ToSchema)]
    #[serde(rename_all = "snake_case")]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SortValue {
        #[prost(uint64, tag = "1")]
        U64(u64),
        #[prost(int64, tag = "2")]
        I64(i64),
        #[prost(double, tag = "3")]
        F64(f64),
        #[prost(bool, tag = "4")]
        Boolean(bool),
    }
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafSearchResponse {
    /// Total number of documents matched by the query.
    #[prost(uint64, tag = "1")]
    pub num_hits: u64,
    /// List of the best top-K candidates for the given leaf query.
    #[prost(message, repeated, tag = "2")]
    pub partial_hits: ::prost::alloc::vec::Vec<PartialHit>,
    /// The list of splits that failed. LeafSearchResponse can be an aggregation of results, so there may be multiple.
    #[prost(message, repeated, tag = "3")]
    pub failed_splits: ::prost::alloc::vec::Vec<SplitSearchError>,
    /// Total number of splits the leaf(s) were in charge of.
    /// num_attempted_splits = num_successful_splits + num_failed_splits.
    #[prost(uint64, tag = "4")]
    pub num_attempted_splits: u64,
    /// postcard serialized intermediate aggregation_result.
    #[prost(bytes = "vec", optional, tag = "6")]
    pub intermediate_aggregation_result: ::core::option::Option<
        ::prost::alloc::vec::Vec<u8>,
    >,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnippetRequest {
    #[prost(string, repeated, tag = "1")]
    pub snippet_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "2")]
    pub query_ast_resolved: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDocsRequest {
    /// Request fetching the content of a given list of partial_hits.
    #[prost(message, repeated, tag = "1")]
    pub partial_hits: ::prost::alloc::vec::Vec<PartialHit>,
    /// Split footer offsets. They are required for fetch docs to
    /// fetch the document content in two reads, when the footer is not
    /// cached.
    #[prost(message, repeated, tag = "3")]
    pub split_offsets: ::prost::alloc::vec::Vec<SplitIdAndFooterOffsets>,
    /// Index URI. The index URI defines the location of the storage that contains the
    /// split files.
    #[prost(string, tag = "4")]
    pub index_uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub snippet_request: ::core::option::Option<SnippetRequest>,
    /// `DocMapper` as json serialized trait.
    #[prost(string, tag = "6")]
    pub doc_mapper: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDocsResponse {
    /// List of complete hits.
    #[prost(message, repeated, tag = "1")]
    pub hits: ::prost::alloc::vec::Vec<LeafHit>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTermsRequest {
    /// Index ID
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    /// Field to search on
    #[prost(string, tag = "3")]
    pub field: ::prost::alloc::string::String,
    /// Time filter
    #[prost(int64, optional, tag = "4")]
    pub start_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub end_timestamp: ::core::option::Option<i64>,
    /// Maximum number of hits to return.
    #[prost(uint64, optional, tag = "6")]
    pub max_hits: ::core::option::Option<u64>,
    /// start_key is included, end_key is excluded
    #[prost(bytes = "vec", optional, tag = "7")]
    pub start_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub end_key: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTermsResponse {
    /// Number of hits matching the query.
    #[prost(uint64, tag = "1")]
    pub num_hits: u64,
    /// Matched hits
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub terms: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Elapsed time to perform the request. This time is measured
    /// server-side and expressed in microseconds.
    #[prost(uint64, tag = "3")]
    pub elapsed_time_micros: u64,
    /// The searcherrors that occurred formatted as string.
    #[prost(string, repeated, tag = "4")]
    pub errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafListTermsRequest {
    /// Search request. This is a perfect copy of the original list request,
    #[prost(message, optional, tag = "1")]
    pub list_terms_request: ::core::option::Option<ListTermsRequest>,
    /// Index split ids to apply the query on.
    /// This ids are resolved from the index_uri defined in the search_request.
    #[prost(message, repeated, tag = "2")]
    pub split_offsets: ::prost::alloc::vec::Vec<SplitIdAndFooterOffsets>,
    /// Index URI. The index URI defines the location of the storage that contains the
    /// split files.
    #[prost(string, tag = "3")]
    pub index_uri: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafListTermsResponse {
    /// Total number of documents matched by the query.
    #[prost(uint64, tag = "1")]
    pub num_hits: u64,
    /// List of the first K terms the given leaf query.
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub terms: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// The list of splits that failed. LeafSearchResponse can be an aggregation of results, so there may be multiple.
    #[prost(message, repeated, tag = "3")]
    pub failed_splits: ::prost::alloc::vec::Vec<SplitSearchError>,
    /// Total number of splits the leaf(s) were in charge of.
    /// num_attempted_splits = num_successful_splits + num_failed_splits.
    #[prost(uint64, tag = "4")]
    pub num_attempted_splits: u64,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchStreamRequest {
    /// Index ID
    #[prost(string, tag = "1")]
    pub index_id: ::prost::alloc::string::String,
    /// Quickwit Query AST encoded in Json
    #[prost(string, tag = "11")]
    pub query_ast: ::prost::alloc::string::String,
    /// The time filter is interpreted as a semi-open interval. [start, end)
    #[prost(int64, optional, tag = "4")]
    pub start_timestamp: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "5")]
    pub end_timestamp: ::core::option::Option<i64>,
    /// Name of the fast field to extract
    #[prost(string, tag = "6")]
    pub fast_field: ::prost::alloc::string::String,
    /// The output format
    #[prost(enumeration = "OutputFormat", tag = "7")]
    pub output_format: i32,
    /// The field by which we want to partition
    #[prost(string, optional, tag = "9")]
    pub partition_by_field: ::core::option::Option<::prost::alloc::string::String>,
    /// Fields to extract snippet on.
    #[prost(string, repeated, tag = "10")]
    pub snippet_fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafSearchStreamRequest {
    /// Stream request. This is a perfect copy of the original stream request,
    /// that was sent to root.
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<SearchStreamRequest>,
    /// Index split ids to apply the query on.
    /// This ids are resolved from the index_uri defined in the stream request.
    #[prost(message, repeated, tag = "2")]
    pub split_offsets: ::prost::alloc::vec::Vec<SplitIdAndFooterOffsets>,
    /// `DocMapper` as json serialized trait.
    #[prost(string, tag = "5")]
    pub doc_mapper: ::prost::alloc::string::String,
    /// Index URI. The index URI defines the location of the storage that contains the
    /// split files.
    #[prost(string, tag = "6")]
    pub index_uri: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafSearchStreamResponse {
    /// Row of data serialized in bytes.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Split id.
    #[prost(string, tag = "2")]
    pub split_id: ::prost::alloc::string::String,
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortOrder {
    /// Ascending order.
    Asc = 0,
    /// Descending order.
    ///
    /// < This will be the default value;
    Desc = 1,
}
impl SortOrder {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortOrder::Asc => "ASC",
            SortOrder::Desc => "DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASC" => Some(Self::Asc),
            "DESC" => Some(Self::Desc),
            _ => None,
        }
    }
}
#[derive(Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputFormat {
    /// Comma Separated Values format (<https://datatracker.ietf.org/doc/html/rfc4180>).
    /// The delimiter is `,`.
    ///
    /// < This will be the default value
    Csv = 0,
    /// Format data by row in ClickHouse binary format.
    /// <https://clickhouse.tech/docs/en/interfaces/formats/#rowbinary>
    ClickHouseRowBinary = 1,
}
impl OutputFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutputFormat::Csv => "CSV",
            OutputFormat::ClickHouseRowBinary => "CLICK_HOUSE_ROW_BINARY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CSV" => Some(Self::Csv),
            "CLICK_HOUSE_ROW_BINARY" => Some(Self::ClickHouseRowBinary),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchServiceClient<tonic::transport::Channel> {
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
    impl<T> SearchServiceClient<T>
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
        ) -> SearchServiceClient<InterceptedService<T, F>>
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
            SearchServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Root search API.
        /// This RPC identifies the set of splits on which the query should run on,
        /// and dispatch the several calls to `LeafSearch`.
        ///
        /// It is also in charge of merging back the results.
        pub async fn root_search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
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
                "/quickwit.search.SearchService/RootSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "RootSearch"));
            self.inner.unary(req, path, codec).await
        }
        /// Perform a leaf search on a given set of splits.
        ///
        /// It is like a regular search except that:
        /// - the node should perform the search locally instead of dispatching
        /// it to other nodes.
        /// - it should be applied on the given subset of splits
        /// - Hit content is not fetched, and we instead return so called `PartialHit`.
        pub async fn leaf_search(
            &mut self,
            request: impl tonic::IntoRequest<super::LeafSearchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeafSearchResponse>,
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
                "/quickwit.search.SearchService/LeafSearch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "LeafSearch"));
            self.inner.unary(req, path, codec).await
        }
        /// / Fetches the documents contents from the document store.
        /// / This methods takes `PartialHit`s and returns `Hit`s.
        pub async fn fetch_docs(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchDocsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchDocsResponse>,
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
                "/quickwit.search.SearchService/FetchDocs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "FetchDocs"));
            self.inner.unary(req, path, codec).await
        }
        /// Perform a leaf stream on a given set of splits.
        pub async fn leaf_search_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::LeafSearchStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::LeafSearchStreamResponse>>,
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
                "/quickwit.search.SearchService/LeafSearchStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("quickwit.search.SearchService", "LeafSearchStream"),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Root list terms API.
        /// This RPC identifies the set of splits on which the query should run on,
        /// and dispatches the several calls to `LeafListTerms`.
        ///
        /// It is also in charge of merging back the results.
        pub async fn root_list_terms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTermsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTermsResponse>,
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
                "/quickwit.search.SearchService/RootListTerms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("quickwit.search.SearchService", "RootListTerms"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs a leaf list terms on a given set of splits.
        ///
        /// It is like a regular list term except that:
        /// - the node should perform the listing locally instead of dispatching
        /// it to other nodes.
        /// - it should be applied on the given subset of splits
        pub async fn leaf_list_terms(
            &mut self,
            request: impl tonic::IntoRequest<super::LeafListTermsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeafListTermsResponse>,
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
                "/quickwit.search.SearchService/LeafListTerms",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("quickwit.search.SearchService", "LeafListTerms"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Performs a scroll request.
        pub async fn scroll(
            &mut self,
            request: impl tonic::IntoRequest<super::ScrollRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status> {
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
                "/quickwit.search.SearchService/Scroll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "Scroll"));
            self.inner.unary(req, path, codec).await
        }
        /// gRPC request used to store a key in the local storage of the targetted node.
        /// This RPC is used in the mini distributed immutable KV store embedded in quickwit.
        pub async fn put_kv(
            &mut self,
            request: impl tonic::IntoRequest<super::PutKvRequest>,
        ) -> std::result::Result<tonic::Response<super::PutKvResponse>, tonic::Status> {
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
                "/quickwit.search.SearchService/PutKV",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "PutKV"));
            self.inner.unary(req, path, codec).await
        }
        /// Gets a key from the local storage of the targetted node.
        /// This RPC is used in the mini distributed immutable KV store embedded in quickwit.
        pub async fn get_kv(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKvRequest>,
        ) -> std::result::Result<tonic::Response<super::GetKvResponse>, tonic::Status> {
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
                "/quickwit.search.SearchService/GetKV",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("quickwit.search.SearchService", "GetKV"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn report_splits(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportSplitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReportSplitsResponse>,
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
                "/quickwit.search.SearchService/ReportSplits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("quickwit.search.SearchService", "ReportSplits"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod search_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SearchServiceServer.
    #[async_trait]
    pub trait SearchService: Send + Sync + 'static {
        /// Root search API.
        /// This RPC identifies the set of splits on which the query should run on,
        /// and dispatch the several calls to `LeafSearch`.
        ///
        /// It is also in charge of merging back the results.
        async fn root_search(
            &self,
            request: tonic::Request<super::SearchRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status>;
        /// Perform a leaf search on a given set of splits.
        ///
        /// It is like a regular search except that:
        /// - the node should perform the search locally instead of dispatching
        /// it to other nodes.
        /// - it should be applied on the given subset of splits
        /// - Hit content is not fetched, and we instead return so called `PartialHit`.
        async fn leaf_search(
            &self,
            request: tonic::Request<super::LeafSearchRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeafSearchResponse>,
            tonic::Status,
        >;
        /// / Fetches the documents contents from the document store.
        /// / This methods takes `PartialHit`s and returns `Hit`s.
        async fn fetch_docs(
            &self,
            request: tonic::Request<super::FetchDocsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FetchDocsResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the LeafSearchStream method.
        type LeafSearchStreamStream: futures_core::Stream<
                Item = std::result::Result<
                    super::LeafSearchStreamResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        /// Perform a leaf stream on a given set of splits.
        async fn leaf_search_stream(
            &self,
            request: tonic::Request<super::LeafSearchStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::LeafSearchStreamStream>,
            tonic::Status,
        >;
        /// Root list terms API.
        /// This RPC identifies the set of splits on which the query should run on,
        /// and dispatches the several calls to `LeafListTerms`.
        ///
        /// It is also in charge of merging back the results.
        async fn root_list_terms(
            &self,
            request: tonic::Request<super::ListTermsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTermsResponse>,
            tonic::Status,
        >;
        /// Performs a leaf list terms on a given set of splits.
        ///
        /// It is like a regular list term except that:
        /// - the node should perform the listing locally instead of dispatching
        /// it to other nodes.
        /// - it should be applied on the given subset of splits
        async fn leaf_list_terms(
            &self,
            request: tonic::Request<super::LeafListTermsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LeafListTermsResponse>,
            tonic::Status,
        >;
        /// Performs a scroll request.
        async fn scroll(
            &self,
            request: tonic::Request<super::ScrollRequest>,
        ) -> std::result::Result<tonic::Response<super::SearchResponse>, tonic::Status>;
        /// gRPC request used to store a key in the local storage of the targetted node.
        /// This RPC is used in the mini distributed immutable KV store embedded in quickwit.
        async fn put_kv(
            &self,
            request: tonic::Request<super::PutKvRequest>,
        ) -> std::result::Result<tonic::Response<super::PutKvResponse>, tonic::Status>;
        /// Gets a key from the local storage of the targetted node.
        /// This RPC is used in the mini distributed immutable KV store embedded in quickwit.
        async fn get_kv(
            &self,
            request: tonic::Request<super::GetKvRequest>,
        ) -> std::result::Result<tonic::Response<super::GetKvResponse>, tonic::Status>;
        async fn report_splits(
            &self,
            request: tonic::Request<super::ReportSplitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ReportSplitsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SearchServiceServer<T: SearchService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SearchService> SearchServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SearchServiceServer<T>
    where
        T: SearchService,
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
                "/quickwit.search.SearchService/RootSearch" => {
                    #[allow(non_camel_case_types)]
                    struct RootSearchSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::SearchRequest>
                    for RootSearchSvc<T> {
                        type Response = super::SearchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).root_search(request).await };
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
                        let method = RootSearchSvc(inner);
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
                "/quickwit.search.SearchService/LeafSearch" => {
                    #[allow(non_camel_case_types)]
                    struct LeafSearchSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::LeafSearchRequest>
                    for LeafSearchSvc<T> {
                        type Response = super::LeafSearchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeafSearchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).leaf_search(request).await };
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
                        let method = LeafSearchSvc(inner);
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
                "/quickwit.search.SearchService/FetchDocs" => {
                    #[allow(non_camel_case_types)]
                    struct FetchDocsSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::FetchDocsRequest>
                    for FetchDocsSvc<T> {
                        type Response = super::FetchDocsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FetchDocsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).fetch_docs(request).await };
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
                        let method = FetchDocsSvc(inner);
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
                "/quickwit.search.SearchService/LeafSearchStream" => {
                    #[allow(non_camel_case_types)]
                    struct LeafSearchStreamSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::ServerStreamingService<
                        super::LeafSearchStreamRequest,
                    > for LeafSearchStreamSvc<T> {
                        type Response = super::LeafSearchStreamResponse;
                        type ResponseStream = T::LeafSearchStreamStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeafSearchStreamRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).leaf_search_stream(request).await
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
                        let method = LeafSearchStreamSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/quickwit.search.SearchService/RootListTerms" => {
                    #[allow(non_camel_case_types)]
                    struct RootListTermsSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::ListTermsRequest>
                    for RootListTermsSvc<T> {
                        type Response = super::ListTermsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTermsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).root_list_terms(request).await
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
                        let method = RootListTermsSvc(inner);
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
                "/quickwit.search.SearchService/LeafListTerms" => {
                    #[allow(non_camel_case_types)]
                    struct LeafListTermsSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::LeafListTermsRequest>
                    for LeafListTermsSvc<T> {
                        type Response = super::LeafListTermsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeafListTermsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).leaf_list_terms(request).await
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
                        let method = LeafListTermsSvc(inner);
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
                "/quickwit.search.SearchService/Scroll" => {
                    #[allow(non_camel_case_types)]
                    struct ScrollSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::ScrollRequest>
                    for ScrollSvc<T> {
                        type Response = super::SearchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScrollRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).scroll(request).await };
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
                        let method = ScrollSvc(inner);
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
                "/quickwit.search.SearchService/PutKV" => {
                    #[allow(non_camel_case_types)]
                    struct PutKVSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::PutKvRequest> for PutKVSvc<T> {
                        type Response = super::PutKvResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutKvRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).put_kv(request).await };
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
                        let method = PutKVSvc(inner);
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
                "/quickwit.search.SearchService/GetKV" => {
                    #[allow(non_camel_case_types)]
                    struct GetKVSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::GetKvRequest> for GetKVSvc<T> {
                        type Response = super::GetKvResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetKvRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_kv(request).await };
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
                        let method = GetKVSvc(inner);
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
                "/quickwit.search.SearchService/ReportSplits" => {
                    #[allow(non_camel_case_types)]
                    struct ReportSplitsSvc<T: SearchService>(pub Arc<T>);
                    impl<
                        T: SearchService,
                    > tonic::server::UnaryService<super::ReportSplitsRequest>
                    for ReportSplitsSvc<T> {
                        type Response = super::ReportSplitsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportSplitsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).report_splits(request).await
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
                        let method = ReportSplitsSvc(inner);
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
    impl<T: SearchService> Clone for SearchServiceServer<T> {
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
    impl<T: SearchService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SearchService> tonic::server::NamedService for SearchServiceServer<T> {
        const NAME: &'static str = "quickwit.search.SearchService";
    }
}

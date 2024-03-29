#[cfg(feature = "RunAspectSentiment")]
pub mod RunAspectSentiment;
#[cfg(feature = "RunAspectSentimentAdvance")]
pub mod RunAspectSentimentAdvance;
#[cfg(feature = "RunClassification")]
pub mod RunClassification;
#[cfg(feature = "RunConstituencyParser")]
pub mod RunConstituencyParser;
#[cfg(feature = "RunDependencyParser")]
pub mod RunDependencyParser;
#[cfg(feature = "RunDocClassification")]
pub mod RunDocClassification;
#[cfg(feature = "RunDomainSentiment")]
pub mod RunDomainSentiment;
#[cfg(feature = "RunEntityLinking")]
pub mod RunEntityLinking;
#[cfg(feature = "RunEntitySentiment")]
pub mod RunEntitySentiment;
#[cfg(feature = "RunEventExtraction")]
pub mod RunEventExtraction;
#[cfg(feature = "RunFileTranslation")]
pub mod RunFileTranslation;
#[cfg(feature = "RunGetFileTranslationResult")]
pub mod RunGetFileTranslationResult;
#[cfg(feature = "RunKeywordExtract")]
pub mod RunKeywordExtract;
#[cfg(feature = "RunLanguageDetection")]
pub mod RunLanguageDetection;
#[cfg(feature = "RunMultiGrainedSegment")]
pub mod RunMultiGrainedSegment;
#[cfg(feature = "RunNer")]
pub mod RunNer;
#[cfg(feature = "RunNerDomain")]
pub mod RunNerDomain;
#[cfg(feature = "RunPoem")]
pub mod RunPoem;
#[cfg(feature = "RunSegment")]
pub mod RunSegment;
#[cfg(feature = "RunSemanticParser")]
pub mod RunSemanticParser;
#[cfg(feature = "RunSentenceEmbedding")]
pub mod RunSentenceEmbedding;
#[cfg(feature = "RunSentiment")]
pub mod RunSentiment;
#[cfg(feature = "RunSummary")]
pub mod RunSummary;
#[cfg(feature = "RunSummaryDomain")]
pub mod RunSummaryDomain;
#[cfg(feature = "RunTextSimilarity")]
pub mod RunTextSimilarity;
#[cfg(feature = "RunTextSimilarityAdvance")]
pub mod RunTextSimilarityAdvance;
#[cfg(feature = "RunTextTranslation")]
pub mod RunTextTranslation;

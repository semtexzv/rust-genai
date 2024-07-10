use super::groq::MODELS as GROQ_MODELS;
use crate::Result;
use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, Eq, PartialEq, Hash)]
pub enum AdapterKind {
	OpenAI,
	Ollama,
	Anthropic,
	Cohere,
	Gemini,
	Groq,
	// Note: Variants will probalby be suffixed
	// AnthropicBerock,
}

impl AdapterKind {
	/// Very simplistic mapper for now.
	///  - starts_with "gpt"      -> OpenAI
	///  - starts_with "claude"   -> Anthropic
	///  - starts_with "command"  -> Cohere
	///  - starts_with "gemini"   -> Gemini
	///  - model in Groq models   -> Groq
	///  - For anything else      -> Ollama
	pub fn from_model(model: &str) -> Result<Self> {
		if model.starts_with("gpt") {
			Ok(AdapterKind::OpenAI)
		} else if model.starts_with("claude") {
			Ok(AdapterKind::Anthropic)
		} else if model.starts_with("command") {
			Ok(AdapterKind::Cohere)
		} else if model.starts_with("gemini") {
			Ok(AdapterKind::Gemini)
		} else if GROQ_MODELS.contains(&model) {
			return Ok(AdapterKind::Groq);
		}
		// for now, fallback on Ollama
		else {
			Ok(Self::Ollama)
		}
	}
}
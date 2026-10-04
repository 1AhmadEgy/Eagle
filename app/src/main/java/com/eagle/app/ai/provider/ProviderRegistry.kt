package com.eagle.app.ai.provider
class ProviderRegistry(providers: Iterable<AIProvider> = emptyList()) {
    private val entries = linkedMapOf<String, AIProvider>()
    init { providers.forEach(::register) }
    fun register(provider: AIProvider) {
        require(provider.id.isNotBlank())
        require(entries.putIfAbsent(provider.id, provider) == null) { "provider already registered: ${provider.id}" }
    }
    fun get(id: String): AIProvider? = entries[id]
    fun all(): List<AIProvider> = entries.values.toList()
    fun clear() { entries.clear() }
}

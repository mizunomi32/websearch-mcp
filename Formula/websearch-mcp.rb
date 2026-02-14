class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.5"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.5/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "97b72e88003711c0a6a1fcfd40ab6235d18f21d2c96745bbf5aa8b62393e9b45"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.5/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "5aca07767a72597863e0040a1154a2747a281e890f13fff092a3ace00aca84f0"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.0/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "PLACEHOLDER"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.0/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "PLACEHOLDER"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

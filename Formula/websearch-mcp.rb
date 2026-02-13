class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.0/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "b1034c0bdf7d391d921a5a93bb9dcfb19034b92d8c5866bccbc3751b5b5796fd"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.0/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "216a5676589e3d415d559b1a6a34c7d55e3c43baf8f069ff256b49e8bb315955"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

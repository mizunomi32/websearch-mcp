class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.0.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.0.1/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "aab28a14f492a03f84901542fdc0836cbeb67e4c1a1b6f668005d1505f1ccc1c"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.0.1/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "fb3d285065bdaae7b699439804fbb90d45b9a1e6ccf6662efd7d69dd51eaa505"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

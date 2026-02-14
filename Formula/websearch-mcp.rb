class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.3"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.3/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "e531158e484fa7c2e955cf20549dbc24496c79411705f174424607abac7d24ce"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.3/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "9bfaf0ddc8eec2942f982a3b713597cdbe16cc248ab9317512a575ce4110eab1"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

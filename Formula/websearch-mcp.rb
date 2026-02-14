class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.2"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.2/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "9d9dd247ec81f52bf1278cd5669ad5dc4871a281355a28b20e2fb05d82912998"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.2/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "2b5c2fd92139de9347a2db6813a647fe424339045eb0e5a7b5025c18d9465be1"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end

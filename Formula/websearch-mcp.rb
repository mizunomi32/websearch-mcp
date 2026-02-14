class WebsearchMcp < Formula
  desc "MCP server providing DuckDuckGo web search"
  homepage "https://github.com/mizunomi32/websearch-mcp"
  version "0.1.4"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.4/websearch-mcp-darwin-arm64.tar.gz"
      sha256 "ab4fd98ff8144ab7a991d3c871402c923abf204e909a78549f89321309fd3a26"
    else
      url "https://github.com/mizunomi32/websearch-mcp/releases/download/v0.1.4/websearch-mcp-darwin-amd64.tar.gz"
      sha256 "b84b6e08f84686a73ccbbad2fb3ee96123483adc0ff65eed0946dad913376136"
    end
  end

  def install
    bin.install "websearch-mcp"
  end

  test do
    assert_predicate bin/"websearch-mcp", :executable?
  end
end
